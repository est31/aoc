use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let net = parse(INPUT);
	println!("count with t: {}", count_with_t(&net));
	println!("password: {}", password(&net));
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

fn try_extend_sizes(net :&Network, cliques :&HashSet<Vec<usize>>) -> HashSet<Vec<usize>> {
	let mut res = HashSet::new();
	for cl in cliques {
		for adj in &net.adj[&cl[0]] {
			let mut stays_clique = true;
			for in_cl in cl.iter() {
				if adj == in_cl || !net.adj[in_cl].contains(adj) {
					stays_clique = false;
					break;
				}
			}
			if stays_clique {
				let mut new_cl = cl.clone();
				new_cl.push(*adj);
				new_cl.sort();
				res.insert(new_cl);
			}
		}
	}
	res
}

fn password(net :&Network) -> String {
	let mut k3s = HashSet::new();
	for (id, adj_id) in net.adj.iter() {
		for adj in adj_id {
			for adjj in &net.adj[&adj] {
				if *adjj == *id { continue }
				if !adj_id.contains(&adjj) { continue }
				let mut cl = vec![*id, *adj, *adjj];
				cl.sort();
				k3s.insert(cl);
			}
		}
	}

	let mut max_cliques = k3s;
	let mut new_max_cliques = max_cliques.clone();
	while !new_max_cliques.is_empty() {
		max_cliques = new_max_cliques;
		new_max_cliques = try_extend_sizes(net, &max_cliques);
	}

	let max_clique = max_cliques.iter().next().unwrap();
	let mut s = max_clique.iter()
		.map(|cl| net.id_to_name[cl].to_owned())
		.collect::<Vec<String>>();
	s.sort();
	s.join(",")
}
