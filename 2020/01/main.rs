use std::collections::HashSet;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	let (a, b) = two_sum(&lines);
	println!("product: {}", a * b);

	let (a, b, c) = three_sum(&lines);
	println!("product: {}", a * b * c);
}

fn parse(input :&str) -> Vec<u64> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| u64::from_str(l).unwrap())
		.collect::<Vec<_>>()
}

const TARGET :u64 = 2020;

fn two_sum(input :&[u64]) -> (u64, u64) {
	// TODO: this doesn't support the case of precisely the half occuring twice
	let set = input.iter()
		.copied()
		.collect::<HashSet<_>>();
	for v in input.iter() {
		if set.contains(&(TARGET - v)) {
			return (*v, TARGET - v);
		}
	}
	panic!("Not found!");
}

fn three_sum(input :&[u64]) -> (u64, u64, u64) {
	let set = input.iter()
		.copied()
		.collect::<HashSet<_>>();
	for (vi, v) in input.iter().enumerate() {
		for (wi, w) in input.iter().enumerate() {
			if vi == wi {
				continue;
			}
			if v + w > TARGET {
				continue;
			}
			let u = TARGET - (v + w);
			if set.contains(&u) {
				return (*v, *w, u);
			}
		}
	}
	panic!("Not found!");
}
