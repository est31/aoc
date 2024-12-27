use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	println!("secret num 2000 sum: {}", secret_num_2000_sum(&nums));
	println!("most bananas: {}", most_bananas(&nums));
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
	p % 16_777_216

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

fn bananas_for(nums :&[u64], pattern :&[i8]) -> u64 {
	nums.iter()
		.map(|num| {
			let mut n = *num;
			let mut diffs = [0; 4];
			for i in 0..2000 {
				for j in 0..3 {
					diffs[j] = diffs[j + 1];
				}
				let next = advance(n);
				diffs[3] = (next % 10) as i8 - (n % 10) as i8;
				n = next;
				if i >= 3 && diffs == pattern {
					return n % 10;
				}
			}
			0
		})
		.sum()
}

fn most_bananas_simple(nums :&[u64]) -> u64 {
	let mut max_b = None;
	let mut max_pat = None;
	for i0 in -9..=9 {
		for i1 in -9..=9 {
			for i2 in -9..=9 {
				for i3 in -9..=9 {
					let pat = [i0, i1, i2, i3];
					let b = bananas_for(nums, &pat);
					if max_b.is_none() || max_b.unwrap() <= b {
						max_b = Some(b);
						max_pat = Some(pat);
					}
				}
			}
		}
	}
	dprint!("max pat: {max_pat:?}");
	max_b.unwrap() as u64
}

fn most_bananas(nums :&[u64]) -> u64 {
	most_bananas_simple(nums)
}
