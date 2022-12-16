use std::str::FromStr;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (valves, names) = parse(INPUT);
	let m = max_pressure_release(&valves, &names);
	println!("Max pressure release: {m}");
}

fn parse(input :&str) -> (HashMap<u8, (u32, Vec<u8>)>, HashMap<String, u8>) {
	let mut interned = HashMap::new();
	let adj = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			assert!(l.starts_with("Valve "));
			let mut words = l.splitn(3, " ");
			words.next().unwrap();
			let name = words.next().unwrap();
			let it_len = interned.len() as u8;
			let name = *interned.entry(name.to_owned()).or_insert(it_len);

			let post = words.next().unwrap();
			let mut r = post.split(['=', ';']);
			r.next().unwrap();
			let rate = r.next().unwrap();
			let rate = u32::from_str(rate).unwrap();

			let post = r.next().unwrap();
			let mut list = if post.contains("valve ") {
				post.split("valve ")
			} else {
				post.split("valves ")
			};
			list.next().unwrap();
			let list = list.next().unwrap();
			let items = list.split(", ");
			let items = items
				.map(|item| {
					let it_len = interned.len() as u8;
					let name = *interned.entry(item.to_owned())
						.or_insert(it_len);
					name
				})
				.collect::<Vec<_>>();

			(name, (rate, items))
		})
		.collect::<HashMap<_, _>>();
	(adj, interned)
}

fn max_release_after(valves :&HashMap<u8, (u32, Vec<u8>)>, valve :u8, remaining :u16, visited :u128, memoized :&mut HashMap<(u8, u128, u16), u32>) -> u32 {
	if remaining == 0 {
		return 0;
	}

	if let Some(res) = memoized.get(&(valve, visited, remaining)) {
		return *res;
	}

	let vl = &valves[&valve];

	let max_turned_off = vl.1.iter()
		.map(|nxt| max_release_after(valves, *nxt, remaining - 1, visited, memoized))
		.max()
		.unwrap_or(0);


	let flow_rate = vl.0;
	let can_activate = (visited >> valve) & 1 == 0;
	let res = if flow_rate > 0 && can_activate {
		let visited = visited | 1 << valve;
		let prod = flow_rate * (remaining as u32 - 1);
		let max_turned_on = if remaining > 1 {
			vl.1.iter()
				.map(|nxt| max_release_after(valves, *nxt, remaining - 2, visited, memoized))
				.max()
				.unwrap_or(0)
		} else {
			0
		};
		(max_turned_on + prod).max(max_turned_off)
	} else {
		max_turned_off
	};
	memoized.insert((valve, visited, remaining), res);
	res
}

fn max_pressure_release(valves :&HashMap<u8, (u32, Vec<u8>)>, names :&HashMap<String, u8>) -> u32 {
	let mut memoized = HashMap::new();
	let aa_idx = names["AA"];
	max_release_after(valves, aa_idx, 30, 0, &mut memoized)
}
