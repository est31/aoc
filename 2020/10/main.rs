use std::collections::HashSet;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let (v0, v1) = jolts_diff_count(&nums);
	println!("Product: {}", v0 * v1);
}

fn parse(input :&str) -> Vec<u64> {
	input.lines()
		.map(|l| u64::from_str(l).unwrap())
		.collect()
}

#[cfg(test)]
fn device_jolts(jolts :&[u64]) -> u64 {
	let max = jolts.iter()
		.max()
		.unwrap();
	max + 3
}

fn search(max_connected :u64, to_add :&HashSet<u64>, one_steps :u64, three_steps :u64) -> Option<(u64, u64)> {
	//println!("Search {}: {connected:?} {to_add:?}", connected.len());
	if to_add.is_empty() {
		return Some((one_steps, three_steps));
	}
	let c = max_connected;
	for a in to_add {
		if ((c + 1)..=(c + 3)).contains(&a) {
			let (no, nt) = match a - c {
				1 => (one_steps + 1, three_steps),
				2 => (one_steps, three_steps),
				3 => (one_steps, three_steps + 1),
				_ => panic!(),
			};
			let mut to_add_removed = to_add.clone();
			to_add_removed.remove(a);
			let res = search(*a, &to_add_removed, no, nt);
			if res.is_some() {
				return res;
			}
		}
	}
	None
}

fn jolts_diff_count(jolts :&[u64]) -> (u64, u64) {
	let largest_connected_start = 0; //HashSet::new();
	//connected.insert(0);
	let to_add = jolts.iter()
		.copied()
		.collect::<HashSet<_>>();
	let res = search(largest_connected_start, &to_add, 0, 0);
	let res = res.expect("Couldn't find a setting where all devices are used!");
	(res.0, res.1 + 1)
}
