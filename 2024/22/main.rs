use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	println!("secret num 2000 sum: {}", secret_num_2000_sum(&nums));
}

fn parse(s :&str) -> Vec<u64> {
	s.trim()
		.lines()
		.map(str::trim)
		.map(|l| u64::from_str(l).unwrap())
		.collect::<Vec<_>>()
}

macro_rules! dprint {
	($($args:expr),*) => {
		//if false
			{ print!($($args),*); }
	};
}

fn prune(p :u64) -> u64 {
	p % 16777216

}

fn advance(mut v :u64) -> u64 {
	v ^= v * 64;
	v = prune(v);
	v ^= v / 32;
	v = prune(v);
	v ^= v * 2048;
	v = prune(v);
	v
}

fn secret_num_2000_sum(nums :&[u64]) -> u64 {
	nums.into_iter()
		.map(|n| {
			let mut n = *n;
			for _ in 0..2000 {
				n = advance(n);
			}
			n
		})
		.sum()
}
