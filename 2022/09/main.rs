use std::collections::HashSet;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let grid = parse(INPUT);
	let ov = visited_positions(&grid, 1);
	println!("visited positions len 2: {}", ov);
	let ov = visited_positions(&grid, 9);
	println!("visited positions len 10: {}", ov);
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

fn modify_for_head(hx :i32, hy :i32, tx :&mut i32, ty :&mut i32) -> bool {
	let x_diff = (hx - *tx).abs();
	let y_diff = (hy - *ty).abs();
	if x_diff.max(y_diff) > 1 {
		match (x_diff, y_diff) {
			(2, 1) => {
				// Move diagonally
				*tx += (hx - *tx).signum();
				*ty = hy;
			},
			(1, 2) => {
				// Move diagonally
				*tx = hx;
				*ty += (hy - *ty).signum();
			},
			(2, 2) => {
				// Move diagonally
				*tx = (hx - *tx).signum();
				*ty += (hy - *ty).signum();
			},
			(2, 0) => *tx += (hx - *tx).signum(),
			(0, 2) => *ty += (hy - *ty).signum(),
			_ => panic!("invalid x_diff={x_diff}, y_diff={y_diff}"),
		}
		true
	} else {
		false
	}
}

fn visited_positions(cmds :&[(char, u32)], tail_len :usize) -> usize {
	let mut visited = HashSet::new();
	let mut hx = 0i32;
	let mut hy = 0i32;
	let mut tp = vec![(0i32, 0i32); tail_len];
	visited.insert(*tp.last().unwrap());
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
			let mut rhx = hx;
			let mut rhy = hy;
			let tp_len = tp.len();
			for (i, (rtx, rty)) in tp.iter_mut().enumerate() {
				if modify_for_head(rhx, rhy, rtx, rty) {
					if i == tp_len - 1 {
						visited.insert((*rtx, *rty));
					}
				}
				rhx = *rtx;
				rhy = *rty;
			}
		}
	}
	visited.len()
}
