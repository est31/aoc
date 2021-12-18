use std::fmt::{Display, Debug, self};
use std::mem::replace;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct StrErr(String);

impl<T :Display> From<T> for StrErr {
	fn from(v :T) -> Self {
		StrErr(format!("{}", v))
	}
}

type Result<T> = std::result::Result<T, StrErr>;

fn main() -> Result<()> {
	let sum = calculate_sum_lines(INPUT);
	println!("Magnitude of the sum: {}", sum.magnitude());
	println!("Maximum total sum: {}", max_sum(INPUT)?.unwrap());
	Ok(())
}

fn calculate_sum_lines(input :&str) -> Num {
	input.trim().lines()
		.map(|l| Num::parse(l).unwrap())
		.reduce(|mut la, lb| {
			la.add(lb); la
		})
		.unwrap()
}

fn max_sum(input :&str) -> Result<Option<u32>> {
	let nums = input.trim().lines()
		.map(|l| Num::parse(l))
		.collect::<Result<Vec<Num>>>()?;
	let mut max_magnitude = None;
	for (i, na) in nums.iter().enumerate() {
		for (j, nb) in nums.iter().enumerate() {
			if i == j {
				continue;
			}
			let mut sum = na.clone();
			sum.add(nb.clone());
			let m = sum.magnitude();
			let max_so_far = max_magnitude.unwrap_or(0);
			max_magnitude = Some(m.max(max_so_far));
		}
	}
	Ok(max_magnitude)
}



#[derive(PartialEq, Eq, Clone)]
enum Num {
	Number(u32),
	Pair(Box<[Num; 2]>),
}

impl Num {
	fn parse(input :&str) -> Result<Self> {
		let input = input.trim();
		Ok(Self::parse_inner(input.as_bytes())?.0)
	}
	fn parse_inner(input :&[u8]) -> Result<(Self, usize)> {
		match input.get(0) {
			None => Err("Expected char but got none")?,
			Some(c @ b'0'..=b'9') => {
				let n = (c - b'0') as u32;
				// Limited support for 2 digit numbers
				// as they are needed for the tests
				// (but don't occur in the actual input)
				let (n, l) = if let Some(c @ b'0'..=b'9') = input.get(1) {
					let m = (c - b'0') as u32;
					(n * 10 + m, 2)
				} else {
					(n, 1)
				};
				Ok((Num::Number(n), l))
			},
			Some(b'[') => {
				let (first, f_ends_at) = Self::parse_inner(&input[1..])?;
				let (second, s_ends_at) = Self::parse_inner(&input[(f_ends_at + 2)..])?;
				let ends_at = f_ends_at + s_ends_at + 3;
				Ok((Num::Pair(Box::new([first, second])), ends_at))
			},
			Some(c) => Err(format!("Encountered unexpected character '{}'", *c as char))?,
		}
	}
	fn add(&mut self, other :Num) {
		let old_self = replace(self, Num::Number(0));
		*self = Num::Pair(Box::new([old_self, other]));
		self.reduce();
	}
	fn reduce(&mut self) {
		while self.reduce_step() {}
	}
	fn reduce_step(&mut self) -> bool {
		self.maybe_explode() || self.maybe_split()
	}
	fn maybe_explode(&mut self) -> bool {
		self.maybe_explode_inner(0).0
	}
	fn maybe_explode_inner(&mut self, depth :usize) -> (bool, [Option<u32>; 2]) {
		let mut num_pair = None;
		match self {
			Num::Number(_) => (),
			Num::Pair(p) => if depth == 4 {
				if let (Num::Number(a), Num::Number(b)) = (&p[0], &p[1]) {
					num_pair = Some([Some(*a), Some(*b)]);
				}
			} else {
				let (exploded, mut payload) = p[0].maybe_explode_inner(depth + 1);
				if exploded {
					p[1].try_put_payload(payload[1].take(), true);
					return (exploded, payload);
				} else {
					let (exploded, mut payload) = p[1].maybe_explode_inner(depth + 1);
					if exploded {
						p[0].try_put_payload(payload[0].take(), false);
						return (exploded, payload);
					}
				}
			},
		}
		if let Some(nums) = num_pair {
			*self = Num::Number(0);
			return (true, nums);
		}
		return (false, [None, None]);
	}
	fn try_put_payload(&mut self, payload :Option<u32>, leftmost :bool) {
		match self {
			Num::Number(n) => {
				if let Some(p) = payload {
					*n += p;
				}
			},
			Num::Pair(p) => if leftmost {
				p[0].try_put_payload(payload, leftmost);
			} else {
				p[1].try_put_payload(payload, leftmost);
			},
		}
	}
	fn maybe_split(&mut self) -> bool {
		let mut new_self = None;
		match self {
			Num::Number(m) => if *m >= 10 {
				// Do a split
				let a = *m / 2;
				let b = *m - a;
				let arr = [Num::Number(a), Num::Number(b)];
				new_self = Some(Num::Pair(Box::new(arr)));
			} else {
				// No split needed
			},
			Num::Pair(p) => {
				return p[0].maybe_split() || p[1].maybe_split();
			},
		}
		if let Some(new_self) = new_self {
			*self = new_self;
			return true;
		} else {
			return false;
		}
	}
	fn magnitude(&self) -> u32 {
		match &self {
			Num::Number(n) => *n,
			Num::Pair(b) => b[0].magnitude() * 3 + b[1].magnitude() * 2,
		}
	}
}

impl Debug for Num {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
		match &self {
			Num::Number(n) => write!(f, "{}",  n),
			Num::Pair(b) => write!(f, "[{:?},{:?}]", b[0], b[1]),
		}
	}
}
