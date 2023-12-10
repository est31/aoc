use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let network = parse(INPUT);
	println!("steps required: {}", steps_required(&network));
	println!("steps required ghosts: {}", steps_required_ghosts(&network));
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

fn gcd(a :u128, b :u128) -> u128 {
	let mut a = a;
	let mut b = b;
	while b != 0 {
		let tmp = b;
		b = a % b;
		a = tmp;
	}
	a
}

fn lcm(a :u128, b :u128) -> u128 {
	let res = (a / gcd(a, b)) * b;
	println!("lcm({a}, {b}) = {res}");
	res
}

fn steps_required_ghosts(network :&Network) -> u128 {
	let (l_r, nodes, node_ids) = network;

	let start_nodes = node_ids.iter()
		.filter(|(name, _id)| name.ends_with('A'))
		.map(|(_name, id)| *id)
		.collect::<Vec<_>>();

	let mut do_bf = false;

	let mut cycles = start_nodes.iter()
		.map(|start_node| {
			let mut l_r_it = l_r.iter().cycle().enumerate();
			let mut cur_id = *start_node;
			let mut ending_offsets = Vec::new();
			let mut visited = HashMap::new();
			let (prefix_len, total_len) = loop {
				let (l_r_offs, right) = l_r_it.next().unwrap();
				let m = l_r_offs % l_r.len();

				if let Some(prior_l_r_offs) = visited.insert((m, cur_id), l_r_offs) {
					break (prior_l_r_offs, l_r_offs);
				}

				let node = &nodes[&cur_id];
				cur_id = if *right {
					node.1.1
				} else {
					node.1.0
				};
				if node.0.ends_with('Z') {
					ending_offsets.push(l_r_offs);
				}
			};
			if ending_offsets.len() != 1 {
				do_bf = true;
			}
			let first = ending_offsets[0];
			if first + prefix_len != total_len {
				do_bf = true;
			}
			(ending_offsets, prefix_len as u64, total_len as u64)
		})
		.collect::<Vec<_>>();

	if do_bf {
		return steps_required_ghosts_bf(network) as u128;
	}

	println!("cycles: {cycles:?}");

	let lcm_cycles = cycles.iter()
		.map(|(_ending_offsets, _prefix_len, total_len)| *total_len as u128)
		.reduce(lcm)
		.unwrap();

	lcm_cycles - 1
}

fn steps_required_ghosts_bf(network :&Network) -> u64 {
	let (l_r, nodes, node_ids) = network;

	let start_nodes = node_ids.iter()
		.filter(|(name, _id)| name.ends_with('A'))
		.map(|(_name, id)| *id)
		.collect::<Vec<_>>();

	let mut l_r_it = l_r.iter().cycle();

	let mut cur_nodes = start_nodes;
	let mut cnt = 0;
	let mut max_end_node_count = 0;
	loop {
		let right = l_r_it.next().unwrap();
		let mut end_node_count = 0;
		for cur_id in cur_nodes.iter_mut() {
			let node = &nodes[&*cur_id];
			*cur_id = if *right {
				node.1.1
			} else {
				node.1.0
			};
			if node.0.ends_with('Z') {
				end_node_count += 1;
			}
		}
		max_end_node_count = max_end_node_count.max(end_node_count);
		if cnt % 1_000_000 == 0 {
			println!("after {cnt}: {end_node_count} max {max_end_node_count}");
		}
		if end_node_count == cur_nodes.len() {
			return cnt;
		}

		cnt += 1;
	}
}
