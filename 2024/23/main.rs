use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let net = parse(INPUT);
	println!("count with t: {}", count_with_t(&net));
}

fn parse(s :&str) -> Network {
	let mut name_to_id = HashMap::new();
	let mut id_to_name = HashMap::new();
	let intern = |nti :&mut HashMap<_, _>, id_to_name :&mut HashMap<_, _>, s :&str| {
		let nti_len = nti.len();
		let id = *nti.entry(s.to_owned()).or_insert(nti_len);
		id_to_name.insert(id, s.to_owned());
		id
	};
	let mut adj = HashMap::<usize, HashSet<_>>::new();
	let lines = s.trim()
		.lines()
		.map(str::trim);
	for l in lines {
		let mut c = l.split('-');
		let l = intern(&mut name_to_id, &mut id_to_name, c.next().unwrap());
		let r = intern(&mut name_to_id, &mut id_to_name, c.next().unwrap());
		adj.entry(l)
			.or_default()
			.insert(r);
		adj.entry(r)
			.or_default()
			.insert(l);
	}
	Network {
		id_to_name,
		adj,
	}
}

#[derive(Clone)]
struct Network {
	id_to_name :HashMap<usize, String>,
	adj :HashMap<usize, HashSet<usize>>,
}

fn count_with_t(net :&Network) -> u32 {
	let ids_with_t = net.id_to_name.iter()
		.filter(|(_id, n)| n.starts_with("t"))
		.map(|(id, _n)| id)
		.collect::<Vec<_>>();
	let mut k3s = HashSet::new();
	for id in ids_with_t.iter() {
		let adj_id = &net.adj[&id];
		for adj in adj_id {
			for adjj in &net.adj[&adj] {
				if adjj == *id { continue }
				if !adj_id.contains(&adjj) { continue }
				let mut cl = [**id, *adj, *adjj];
				cl.sort();
				k3s.insert(cl);
			}
		}
	}
	k3s.len() as u32
}
