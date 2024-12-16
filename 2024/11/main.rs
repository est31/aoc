use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let stn = parse(INPUT);
	println!("stone count: {}", stone_count_25(&stn));
	println!("stone count: {}", stone_count_n(&stn, 75));
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

fn parse(s: &str) -> Vec<u64> {
	let s = s.trim();
	s.split(' ')
		.map(|v| {
			u64::from_str(v).unwrap()
		})
		.collect::<Vec<_>>()
}

fn stone_count_25(stones :&[u64]) -> u64 {
	stone_count_n(stones, 25)
}

fn stone_count_n(stones :&[u64], n: u32) -> u64 {
	let mut stones = stones.iter()
		.cloned()
		.map(|st| (st, 1))
		.collect::<HashMap<u64, u64>>();
	for _ in 0..n {
		stones = blink(&stones);
		dprint!("{stones:?}\n");
	}
	stones.iter()
		.map(|(_st, cnt)| *cnt)
		.sum()
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
	let mut new_stones = HashMap::with_capacity(stones.len());
	for (st, cnt) in stones.iter() {
		let v = match st {
			0 => {
				1
			},
			_ if (*st).ilog10() % 2 == 1 => {
				let (upper, lower) = split(*st);
				*new_stones.entry(upper)
					.or_default() += cnt;
				*new_stones.entry(lower)
					.or_default() += cnt;
				continue;
			},
			_ => {
				*st * 2024
			},
		};
		*new_stones.entry(v)
			.or_default() += cnt;
	}
	new_stones
}

fn split(v: u64) -> (u64, u64) {
	let digit_count = v.ilog10() + 1;
	let first_half = 10_u64.pow(digit_count / 2);
	let upper = v /first_half;
	let lower = v % first_half;
	(upper, lower)
}
