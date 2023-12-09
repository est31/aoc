use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let network = parse(INPUT);
	println!("steps required: {}", steps_required(&network));
}

type Network = (Vec<bool>, HashMap<u16, (String, (u16, u16))>, HashMap<String, u16>);

fn parse(input :&str) -> Network {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let first_line = lines.next().unwrap();
	let l_r = first_line.chars()
		.map(|ch| match ch {
			'L' => false,
			'R' => true,
			_ => panic!("Invalid char '{ch}'"),
		})
		.collect::<Vec<_>>();
	assert_eq!(lines.next().unwrap(), "");
	let mut node_ids = HashMap::new();
	let nodes = lines
		.map(|l| {
			let mut it = l.split(" = ");
			let first = it.next().unwrap();
			let second = it.next().unwrap();
			let mut l_r_it = second.split(&[',', ' ', '(', ')'])
				.filter(|s| !s.is_empty());
			let l = l_r_it.next().unwrap();
			let r = l_r_it.next().unwrap();

			let next_id = node_ids.len() as u16;
			let first_id = *node_ids.entry(first.to_string()).or_insert(next_id);

			let next_id = node_ids.len() as u16;
			let l_id = *node_ids.entry(l.to_string()).or_insert(next_id);

			let next_id = node_ids.len() as u16;
			let r_id = *node_ids.entry(r.to_string()).or_insert(next_id);

			(first_id, (first.to_string(), (l_id, r_id)))
		})
		.collect::<HashMap<_, _>>();
	(l_r, nodes, node_ids)
}

fn steps_required(network :&Network) -> u32 {
	let (l_r, nodes, node_ids) = network;
	let goal_id = node_ids["ZZZ"];
	let mut l_r_it = l_r.iter().cycle();
	let mut cur_id = node_ids["AAA"];
	let mut cnt = 0;
	while cur_id != goal_id {
		let right = l_r_it.next().unwrap();
		let node = &nodes[&cur_id];
		cur_id = if *right {
			node.1.1
		} else {
			node.1.0
		};
		cnt += 1;
	}
	cnt
}
