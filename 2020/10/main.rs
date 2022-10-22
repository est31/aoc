use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let (v0, v1) = jolts_diff_count(&nums);
	println!("Product: {}", v0 * v1);
	let tc = jolts_combinations(&nums);
	println!("Total combinations: {tc}");
}

fn parse(input :&str) -> Vec<u64> {
	input.lines()
		.map(|l| u64::from_str(l).unwrap())
		.collect()
}

fn device_jolts(jolts :&[u64]) -> u64 {
	let max = jolts.iter()
		.max()
		.unwrap();
	max + 3
}

fn search_diff_count(max_connected :u64, to_add :&HashSet<u64>, one_steps :u64, three_steps :u64) -> Option<(u64, u64)> {
	if to_add.is_empty() {
		return Some((one_steps, three_steps));
	}
	let c = max_connected;
	for a in (c + 1)..=(c + 3) {
		if !to_add.contains(&a) {
			continue
		}
		let (no, nt) = match a - c {
			1 => (one_steps + 1, three_steps),
			2 => (one_steps, three_steps),
			3 => (one_steps, three_steps + 1),
			_ => panic!(),
		};
		let mut to_add_removed = to_add.clone();
		to_add_removed.remove(&a);
		let res = search_diff_count(a, &to_add_removed, no, nt);
		if let Some(res) = res {
			return Some(res);
		}
	}
	None
}

fn jolts_diff_count(jolts :&[u64]) -> (u64, u64) {
	let largest_connected_start = 0;
	let to_add = jolts.iter()
		.copied()
		.collect::<HashSet<_>>();
	let res = search_diff_count(largest_connected_start, &to_add, 0, 0);
	let (one_steps, three_steps) = res.expect("Couldn't find a setting where all devices are used!");
	(one_steps, three_steps + 1)
}

#[cfg(test)]
fn search_slow(max_connected :u64, jolts :&HashSet<u64>) -> u128 {
	if max_connected >= *jolts.iter().max().unwrap() {
		return 1;
	}
	let c = max_connected;
	let mut sum = 0;
	for a in (c + 1)..=(c + 3) {
		if !jolts.contains(&a) {
			continue
		}
		let s = search_slow(a, &jolts);
		sum += s;
	}
	sum
}

fn search(max_connected :u64, hm :&mut HashMap<u64, u128>, jolts :&HashSet<u64>, tgt :u64) -> u128 {
	if max_connected >= tgt {
		return 1;
	}
	if let Some(v) = hm.get(&max_connected) {
		return *v;
	}
	let c = max_connected;
	let mut sum = 0;
	for a in (c + 1)..=(c + 3) {
		if !jolts.contains(&a) {
			continue
		}
		let s = search(a, hm, jolts, tgt);
		sum += s;
	}
	hm.insert(max_connected, sum);
	sum
}

#[cfg(test)]
fn jolts_combinations_slow(jolts :&[u64]) -> u128 {
	let largest_connected_start = 0;
	let jolts = jolts.iter()
		.copied()
		.collect::<HashSet<_>>();
	let total_combinations = search_slow(largest_connected_start, &jolts);
	total_combinations
}

fn jolts_combinations(jolts :&[u64]) -> u128 {
	let largest_connected_start = 0;
	let device_jolts = device_jolts(jolts) - 3;
	let jolts = jolts.iter()
		.copied()
		.collect::<HashSet<_>>();
	let total_combinations = search(largest_connected_start, &mut HashMap::new(), &jolts, device_jolts);
	total_combinations
}
