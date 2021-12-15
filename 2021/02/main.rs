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
	let final_pos = final_position(INPUT)?;
	println!("Final position multiplied: {}", final_pos.0 * final_pos.1);
	let final_pos_ext = final_position_ext(INPUT)?;
	println!("Final position multiplied part 2: {}", final_pos_ext.0 * final_pos_ext.1);
	Ok(())
}

fn final_position(input :&str) -> Result<(u64, u64)> {
	let mut pos = (0, 0);
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	for l in lines {
		let mut l_it = l.split(' ');
		// TODO: use let_else once available
		let (command, number) = if let (Some(c), Some(n))  = (l_it.next(), l_it.next()) {
			(c, n)
		} else {
			Err(format!("Parse error at line: '{}'", l))?
		};
		let number = u64::from_str(number)?;
		match command {
			"forward" => pos.0 += number,
			"down" => pos.1 += number,
			"up" => pos.1 -= number,
			_ => Err(format!("Unknown command: '{}'", command))?,
		}
	}
	Ok(pos)
}

fn final_position_ext(input :&str) -> Result<(u64, u64)> {
	let mut pos = (0, 0);
	let mut aim = 0;
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	for l in lines {
		let mut l_it = l.split(' ');
		// TODO: use let_else once available
		let (command, number) = if let (Some(c), Some(n))  = (l_it.next(), l_it.next()) {
			(c, n)
		} else {
			Err(format!("Parse error at line: '{}'", l))?
		};
		let number = u64::from_str(number)?;
		match command {
			"forward" => {
				pos.0 += number;
				pos.1 += number * aim;
			},
			"down" => aim += number,
			"up" => aim -= number,
			_ => Err(format!("Unknown command: '{}'", command))?,
		}
	}
	Ok(pos)
}
