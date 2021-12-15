use std::fmt::Display;
use std::str::FromStr;

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
	println!("Increases: {}", count_increases(INPUT)?);
	println!("Increases convolved: {}", count_increases_conv(INPUT)?);
	Ok(())
}

fn count_increases(input :&str) -> Result<usize> {
	let mut v_prior = None;
	let mut sum = 0;
	let numbers = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| u64::from_str(l))
		.collect::<std::result::Result<Vec<_>, _>>();
	for v in numbers? {
		if let Some(v_prior) = v_prior {
			sum += (v > v_prior) as usize;
		}
		v_prior = Some(v);
	}
	Ok(sum)
}

fn count_increases_conv(input :&str) -> Result<usize> {
	let mut v_prior = None;
	let mut sum = 0;
	let numbers = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| u64::from_str(l))
		.collect::<std::result::Result<Vec<_>, _>>();
	for v in numbers?.windows(3) {
		let v :u64 = v.iter().sum();
		if let Some(v_prior) = v_prior {
			sum += (v > v_prior) as usize;
		}
		v_prior = Some(v);
	}
	Ok(sum)
}
