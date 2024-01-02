use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (round_rocks, cube_rocks, hg) = parse(INPUT);
	println!("total load: {}", total_load_tilted(&round_rocks, &cube_rocks, hg));
}

fn parse(input :&str) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>, usize) {
	let lines = input.lines()
		.map(|l| l.trim());
	let mut round_rocks = HashSet::new();
	let mut cube_rocks = HashSet::new();
	let mut height = 0;
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
		height += 1;
	}
	(round_rocks, cube_rocks, height)
}

fn total_load_tilted(round_rocks :&HashSet<(usize, usize)>, cube_rocks :&HashSet<(usize, usize)>, height :usize) -> u32 {
	// 1. tilt
	let mut round_rocks = round_rocks.clone();
	loop {
		let mut movement = false;
		let mut new_round_rocks = HashSet::new();
		for rock_pos in round_rocks.iter() {
			if rock_pos.1 == 0 {
				new_round_rocks.insert(*rock_pos);
				continue;
			}
			let new_pos = (rock_pos.0, rock_pos.1 - 1);
			if !round_rocks.contains(&new_pos) && !cube_rocks.contains(&new_pos) && !new_round_rocks.contains(&new_pos) {
				movement = true;
				new_round_rocks.insert(new_pos);
			} else {
				new_round_rocks.insert(*rock_pos);
			}
		}
		assert_eq!(round_rocks.len(), new_round_rocks.len());
		round_rocks = new_round_rocks;
		//println!("movement = {movement}");
		if !movement {
			break;
		}
	}
	// 2. compute total load
	round_rocks.iter()
		.map(|(_r_x, r_y)| (height - r_y) as u32)
		.sum::<u32>()
}
