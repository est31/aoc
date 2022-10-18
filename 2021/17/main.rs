use std::fmt::Display;
use core::ops::RangeInclusive;
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
	let target = parse_target_area(INPUT)?;
	println!("Highest possible y: {}", find_highest_possible_y(&target).unwrap());
	println!("Number of successes: {}", find_number_of_successes(&target));

	Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct TargetArea {
	x_range :RangeInclusive<i16>,
	y_range :RangeInclusive<i16>,
}

fn parse_target_area(input :&str) -> Result<TargetArea> {
	let mut it = input.trim().split(|c| c == '=' || c == ',' || c == '.');
	let _ = it.next().unwrap();
	let x_min_str = it.next().ok_or("Couldn't parse")?;
	it.next().ok_or("Couldn't parse")?;
	let x_max_str = it.next().ok_or("Couldn't parse")?;
	it.next().ok_or("Couldn't parse")?;
	let y_min_str = it.next().ok_or("Couldn't parse")?;
	it.next().ok_or("Couldn't parse")?;
	let y_max_str = it.next().ok_or("Couldn't parse")?;

	let x_min = i16::from_str(x_min_str)?;
	let x_max = i16::from_str(x_max_str)?;
	let y_min = i16::from_str(y_min_str)?;
	let y_max = i16::from_str(y_max_str)?;

	Ok(TargetArea {
		x_range : x_min..=x_max,
		y_range : y_min..=y_max,
	})
}

fn find_highest_possible_y(tgt :&TargetArea) -> Option<i16> {
	let mut highest = None;
	for v_x in -200..200 {
		for v_y in -1000..1000 {
			if let Some(h_c) = find_target_area(tgt, [v_x, v_y]) {
				if let Some(h) = &mut highest {
					*h = h_c.max(*h);
				} else {
					highest = Some(h_c);
				}
			}
		}
	}
	highest
}

fn find_number_of_successes(tgt :&TargetArea) -> i16 {
	let mut num = 0;
	for v_x in -200..200 {
		for v_y in -1000..1000 {
			num += find_target_area(tgt, [v_x, v_y]).is_some() as i16;
		}
	}
	num
}

fn find_target_area(tgt :&TargetArea, starting_vel :[i16; 2]) -> Option<i16> {
	let [mut x, mut y] = [0, 0];
	let [mut v_x, mut v_y] = starting_vel;
	let mut reached_target = false;
	let mut highest_y = 0;
	loop {
		reached_target |= tgt.x_range.contains(&x) && tgt.y_range.contains(&y);
		highest_y = highest_y.max(y);

		x += v_x;
		y = y.checked_add(v_y)?;
		// TODO use exclusive range patterns in a match
		// once they stabilize
		if v_x < 0 {
			v_x += 1;
		} else if v_x > 0 {
			v_x -= 1;
		}
		v_y -= 1;

		// Termination condition: the probe falls
		// and is below the target area
		if v_y < 0 && y < *tgt.y_range.start() {
			break;
		}
	}
	reached_target.then_some(highest_y)
}
