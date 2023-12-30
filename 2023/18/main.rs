use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let dig_plan = parse(INPUT);
	println!("lava area: {}", lava_cubes(&dig_plan));
}

fn parse(input :&str) -> Vec<(Direction, i32, &str)> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut cmp = l.split_whitespace();
			let dir_st = cmp.next().unwrap();
			let dir = match dir_st {
				"R" => Direction::Right,
				"D" => Direction::Down,
				"L" => Direction::Left,
				"U" => Direction::Up,
				_ => panic!("invalid direction {dir_st}"),
			};
			let len_st = cmp.next().unwrap();
			let len = i32::from_str(len_st).unwrap();
			let col_st = cmp.next().unwrap();
			(dir, len, col_st)
		})
		.collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
	Right,
	Down,
	Left,
	Up,
}

fn lava_cubes(dig_plan :&[(Direction, i32, &str)]) -> u32 {
	let mut vertices = Vec::new();
	let start_pos = (0i32, 0i32);
	let mut pos = start_pos;
	vertices.push(pos);

	for (dir, len, _col_st) in dig_plan {
		let (x, y) = pos;
		//print!("before {pos:?}, ");
		pos = match dir {
			Direction::Right => (x + len, y),
			Direction::Down => (x, y - len),
			Direction::Left => (x - len, y),
			Direction::Up => (x, y + len),
		};
		//println!("at {pos:?} after {dir:?} {len}");
		if pos != start_pos {
			vertices.push(pos);
		}
	}
	let sum_len = dig_plan.iter()
		.map(|(_, len, _)| len)
		.sum::<i32>();
	//println!("vertices={vertices:?}");
	//println!("sum_len={sum_len}");

	// Use shoelace formula (trapezoid formula), similar to day 10
	let first = vertices.first().unwrap();
	let last = vertices.last().unwrap();

	let sum = vertices.windows(2)
		.chain(std::iter::once([*last, *first].as_slice()))
		.map(|w| {
			let prev = w[0];
			let cur = w[1];

			(prev.0 + cur.0) * (prev.1 - cur.1)
		})
		.sum::<i32>();

	let area = (sum / 2).abs() + (sum_len / 2) + 1;

	area as u32
}
