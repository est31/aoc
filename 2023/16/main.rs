use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	println!("energized count: {}", energized_count(&field));
	println!("energized count from anywhere: {}", energized_count_from_anywhere(&field));
}

fn parse(input :&str) -> Vec<Vec<char>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| l.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn apply(&self, p :(usize, usize), w_h :(usize, usize)) -> Option<(usize, usize)> {
		Some(match self {
			Direction::Up => {
				if p.1 == 0 {
					return None;
				}
				(p.0, p.1 - 1)
			},
			Direction::Down => {
				if p.1 >= w_h.1 - 1 {
					return None;
				}
				(p.0, p.1 + 1)
			},
			Direction::Left => {
				if p.0 == 0 {
					return None;
				}
				(p.0 - 1, p.1)
			},
			Direction::Right => {
				if p.0 >= w_h.0 - 1 {
					return None;
				}
				(p.0 + 1, p.1)
			},
		})
	}
}

fn energized_count(field :&[Vec<char>]) -> u32 {
	let p = (0, 0);
	let dir = Direction::Right;
	energized_count_from(field, p, dir)
}

fn energized_count_from_anywhere(field :&[Vec<char>]) -> u32 {
	let height = field.len();
	let width = field[0].len();
	let approaches: [(_, fn(usize, usize, usize) -> (usize, usize), _); 4] = [
		(0..height, |v, _h, _w| (0, v), Direction::Right),
		(0..height, |v, _h, width| (width - 1, v), Direction::Left),
		(0..width, |v, _h, _w| (v, 0), Direction::Down),
		(0..width, |v, height, _w| (v, height - 1), Direction::Up),
	];
	let mut max = None;
	for (range, fun, dir) in approaches {
		for v in range {
			let pos = fun(v, height, width);
			let cnt = energized_count_from(field, pos, dir);
			max = max.max(Some(cnt));
		}
	}
	max.unwrap()
}

fn energized_count_from(field :&[Vec<char>], p :(usize, usize), dir :Direction) -> u32 {
	use Direction as Dir;
	let height = field.len();
	let width = field[0].len();
	let w_h = (width, height);
	let mut energized = vec![vec![false; width]; height];
	let mut explored = HashSet::new();
	let mut unexplored = Vec::new();
	unexplored.push((p, dir));
	while let Some((p, dir)) = unexplored.pop() {
		if !explored.insert((p, dir)) {
			// Already explored
			continue;
		}
		let field_entry = field[p.1][p.0];
		//println!("exploring p={p:?} dir={dir:?} field_entry={field_entry}");
		energized[p.1][p.0] = true;
		let new_dirs = match field_entry {
			'.' => [Some(dir), None],
			'\\' => {
				let new_dir = match dir {
					Dir::Up => Dir::Left,
					Dir::Down => Dir::Right,
					Dir::Left => Dir::Up,
					Dir::Right => Dir::Down,
				};
				[Some(new_dir), None]
			},
			'/' => {
				let new_dir = match dir {
					Dir::Up => Dir::Right,
					Dir::Down => Dir::Left,
					Dir::Left => Dir::Down,
					Dir::Right => Dir::Up,
				};
				[Some(new_dir), None]
			},
			'|' => {
				match dir {
					Dir::Up | Dir::Down => [Some(dir), None],
					Dir::Left | Dir::Right => [Some(Dir::Up), Some(Dir::Down)],
				}
			},
			'-' => {
				match dir {
					Dir::Up | Dir::Down => [Some(Dir::Left), Some(Dir::Right)],
					Dir::Left | Dir::Right => [Some(dir), None],
				}
			},
			fld => panic!("Unsupported field '{fld}'!"),
		};
		//println!("  new dirs: {new_dirs:?}");
		let new_pos_dirs_it = new_dirs.into_iter()
			.filter_map(|v| v)
			.filter_map(|new_dir| {
				let new_pos = new_dir.apply(p, w_h)?;
				Some((new_pos, new_dir))
			});
		for new_pos_dir in new_pos_dirs_it {
			unexplored.push(new_pos_dir);
		}
		//println!("  unexplored: {unexplored:?}");
	}

	let count = energized.iter()
		.map(|l| l.iter())
		.flatten()
		.filter(|b| **b)
		.count();
	count as u32
}
