use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let stn = parse(INPUT);
	println!("stone count: {}", stone_count_25(&stn));
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

fn stone_count_25(stones :&[u64]) -> u32 {
	stone_count_n(stones, 25)
}

fn stone_count_n(stones :&[u64], n: u32) -> u32 {
	let mut stones = stones.to_vec();
	for _ in 0..n {
		stones = blink(&stones);
		dprint!("{stones:?}\n");
	}
	stones.len() as u32
}

fn blink(stones: &[u64]) -> Vec<u64> {
	let mut new_stones = Vec::with_capacity(stones.len());
	for st in stones.iter() {
		match *st {
			0 => {
				new_stones.push(1);
			},
			_ if (*st).ilog10() % 2 == 1 => {
				let (upper, lower) = split(*st);
				new_stones.push(upper);
				new_stones.push(lower);
			},
			_ => {
				new_stones.push(*st * 2024);
			},
		}
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
