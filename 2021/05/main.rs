use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let crossings_straight = find_crossings(&INPUT, true);
	println!("Crossings straight: {}", crossings_straight);
	let crossings = find_crossings(&INPUT, false);
	println!("Crossings all lines: {}", crossings);
}

macro_rules! dprint {
	($($args:expr),*) => {
		//print!($($args),*);
	};
}

fn find_crossings(input :&str, discard_diag :bool) -> u32 {
	let mut floor = HashSet::<(i16, i16)>::new();
	let mut overlaps = HashSet::<(i16, i16)>::new();
	for l in input.lines() {
		let l = l.trim();
		if l.is_empty() {
			continue;
		}
		let mut it = l.split(|c :char| c.is_whitespace() || c == ',');
		let start_x = u16::from_str(it.next().unwrap()).unwrap() as i16;
		let start_y = u16::from_str(it.next().unwrap()).unwrap() as i16;
		it.next().unwrap();
		let end_x = u16::from_str(it.next().unwrap()).unwrap() as i16;
		let end_y = u16::from_str(it.next().unwrap()).unwrap() as i16;
		let mut visit = |x, y| {
			let is_overlap = !floor.insert((x,y));
			if is_overlap {
				overlaps.insert((x, y));
			}
			dprint!("{},{}: {}; ", x, y, is_overlap);
		};
		if (start_x != end_x) && (start_y != end_y) {
			// The line is neither horizontal nor vertical
			if discard_diag {
				dprint!("discarding: {},{} -> {},{}\n", start_x, start_y, end_x, end_y);
				continue;
			}
		}
		let mut x = start_x;
		let mut y = start_y;
		let dir_x = (end_x as i16 - start_x as i16).signum();
		let dir_y = (end_y as i16 - start_y as i16).signum();

		dprint!("{},{} -> {},{}\n    ", start_x, start_y, end_x, end_y);
		while !((x, y) == (end_x, end_y)) {
			visit(x, y);
			x += dir_x;
			y += dir_y;
		}
		visit(x, y);
		dprint!("\n");
	}
	overlaps.len() as u32
}
