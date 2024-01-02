use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (round_rocks, cube_rocks, h_w) = parse(INPUT);
	println!("total load: {}", total_load_tilted(&round_rocks, &cube_rocks, h_w.0));
	println!("total load (circles): {}", total_load_circles(&round_rocks, &cube_rocks, h_w));
}

fn parse(input :&str) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>, (usize, usize)) {
	let lines = input.lines()
		.map(|l| l.trim());
	let mut round_rocks = HashSet::new();
	let mut cube_rocks = HashSet::new();
	let mut height = 0;
	let mut width = 0;
	for (line_idx, line) in lines.enumerate() {
		for (col_idx, c) in line.chars().enumerate() {
			match c {
				'O' => {
					round_rocks.insert((col_idx, line_idx));
				},
				'#' => {
					cube_rocks.insert((col_idx, line_idx));
				},
				'.' => (),
				c => {
					panic!("Invalid char '{c}' in line '{line}'");
				},
			}
		}
		width = line.len();
		height += 1;
	}
	(round_rocks, cube_rocks, (height, width))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
	North,
	West,
	South(usize),
	East(usize),
}

impl Direction {
	fn new_pos(&self, x :usize, y :usize) -> Option<(usize, usize)> {
		match self {
			Direction::North => if y == 0 {
				None
			} else {
				Some((x, y - 1))
			},
			Direction::West => if x == 0 {
				None
			} else {
				Some((x - 1, y))
			},
			Direction::South(height) => if y == height - 1 {
				None
			} else {
				Some((x, y + 1))
			},
			Direction::East(width) => if x == width - 1 {
				None
			} else {
				Some((x + 1, y))
			},
		}
	}
}

fn tilt(round_rocks :&mut HashSet<(usize, usize)>, cube_rocks :&HashSet<(usize, usize)>, dir :Direction) {
	loop {
		let mut movement = false;
		let mut new_round_rocks = HashSet::new();
		for rock_pos in round_rocks.iter() {
			let Some(new_pos) = dir.new_pos(rock_pos.0, rock_pos.1) else {
				new_round_rocks.insert(*rock_pos);
				continue;
			};
			if !round_rocks.contains(&new_pos) && !cube_rocks.contains(&new_pos) && !new_round_rocks.contains(&new_pos) {
				movement = true;
				new_round_rocks.insert(new_pos);
			} else {
				new_round_rocks.insert(*rock_pos);
			}
		}
		assert_eq!(round_rocks.len(), new_round_rocks.len());
		*round_rocks = new_round_rocks;
		//println!("movement = {movement}");
		if !movement {
			break;
		}
	}
}

fn total_load(round_rocks :&HashSet<(usize, usize)>, height :usize) -> u32 {
	round_rocks.iter()
		.map(|(_r_x, r_y)| (height - r_y) as u32)
		.sum::<u32>()
}

fn total_load_tilted(round_rocks :&HashSet<(usize, usize)>, cube_rocks :&HashSet<(usize, usize)>, height :usize) -> u32 {
	// 1. tilt
	let mut round_rocks = round_rocks.clone();
	tilt(&mut round_rocks, cube_rocks, Direction::North);
	// 2. compute total load
	total_load(&round_rocks, height)
}

fn advance_n(round_rocks :&mut HashSet<(usize, usize)>, cube_rocks :&HashSet<(usize, usize)>, h_w :(usize, usize), count :u64) {
	let dirs = [
		Direction::North,
		Direction::West,
		Direction::South(h_w.0),
		Direction::East(h_w.1),
	];
	let adv = |round_rocks :&mut _| {
		//println!("adv");
		for dir in dirs {
			//println!("    -> tilt {dir:?}");
			tilt(round_rocks, cube_rocks, dir);
		}
	};
	let mut seen = HashMap::new();
	let mut idx = 0;
	let (loop_st, loop_end) = loop {
		if idx >= count {
			return;
		}
		adv(round_rocks);
		let mut rocks_list = round_rocks.iter().cloned().collect::<Vec<_>>();
		rocks_list.sort();
		if let Some(prev) = seen.insert(rocks_list, idx) {
			break (prev, idx);
		}
		idx += 1;
		//println!("idx: {idx}");
	};
	let loop_size = loop_end - loop_st;
	let remaining = (count - loop_end - 1) % loop_size;
	//println!("found loop. loop_size = {loop_size}, loop_end = {loop_end}, loop_st = {loop_st}, remaining = {remaining}");
	for _ in 0..remaining {
		adv(round_rocks);
	}
}

fn total_load_circles(round_rocks :&HashSet<(usize, usize)>, cube_rocks :&HashSet<(usize, usize)>, h_w :(usize, usize)) -> u32 {
	// 1. tilt in all four directions, many times
	let mut round_rocks = round_rocks.clone();
	const TARGET :u64 = 1_000_000_000;
	advance_n(&mut round_rocks, cube_rocks, h_w, TARGET);

	// 2. compute total load
	total_load(&round_rocks, h_w.0)
}
