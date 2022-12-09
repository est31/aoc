use std::collections::HashSet;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let grid = parse(INPUT);
	let ov = visited_positions(&grid);
	println!("visited positions: {}", ov);
}

fn parse(input :&str) -> Vec<(char, u32)> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut words = l.split_whitespace();
			let (Some(ch), Some(n)) = (words.next(), words.next()) else {
				panic!("invalid line '{l}'");
			};
			let cnt = u32::from_str(n).unwrap();
			assert_eq!(ch.len(), 1);
			let ch = ch.chars().next().unwrap();
			(ch, cnt)
		})
		.collect::<Vec<_>>()
}

fn visited_positions(cmds :&[(char, u32)]) -> usize {
	let mut visited = HashSet::new();
	let mut hx = 0i32;
	let mut hy = 0i32;
	let mut tx = 0i32;
	let mut ty = 0i32;
	visited.insert((tx, ty));
	for (dir, num) in cmds.iter() {
		let (dx, dy) = match dir {
			'R' => (0, 1),
			'U' => (1, 0),
			'D' => (-1, 0),
			'L' => (0, -1),
			_ => panic!("unknown command '{dir}'"),
		};
		for _ in 0..*num {
			hx += dx;
			hy += dy;
			let x_diff = (hx - tx).abs();
			let y_diff = (hy - ty).abs();
			if x_diff.max(y_diff) > 1 {
				match (x_diff, y_diff) {
					(2, 1) => {
						// Move diagonally
						tx += (hx - tx).signum();
						ty = hy;
					},
					(1, 2) => {
						// Move diagonally
						tx = hx;
						ty += (hy - ty).signum();
					},
					(2, 0) => tx += dx,
					(0, 2) => ty += dy,
					_ => panic!("invalid x_diff={x_diff}, y_diff={y_diff}"),
				}
				visited.insert((tx, ty));
			}
		}
	}
	visited.len()
}
