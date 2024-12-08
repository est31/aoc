use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let f = parse(INPUT);
	println!("positions visited: {}", positions_visited(&f));
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn turn_right(&self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Right => Direction::Down,
		}
	}
}

#[derive(Clone)]
struct Field {
	height: usize,
	width: usize,
	field: Vec<Vec<bool>>,
	pos: (usize, usize),
	dir: Direction,
}

impl Field {
	fn pos_directed_to(&self) -> Option<(usize, usize)> {
		let py = self.pos.0;
		let px = self.pos.1;
		Some(match self.dir {
			Direction::Up if py == 0 => return None,
			Direction::Left if px == 0 => return None,
			Direction::Right if px >= self.width - 1 => return None,
			Direction::Down if py >= self.height - 1 => return None,
			Direction::Up => (py - 1, px),
			Direction::Left => (py, px - 1),
			Direction::Right => (py, px + 1),
			Direction::Down => (py + 1, px),
		})
	}
	fn step(&mut self) -> bool {
		loop {
			let Some(dir_to) = self.pos_directed_to() else {
				return false;
			};
			if self.field[dir_to.0][dir_to.1] {
				self.dir = self.dir.turn_right();
			} else {
				self.pos = dir_to;
				return true;
			}
		}
	}
}

fn parse(s: &str) -> Field {
	let mut init = None;
	let field = s.lines()
		.enumerate()
		.map(|(py, l)| {
			l.chars()
				.enumerate()
				.map(|(px, ch)| {
					if let Some(dir) = match ch {
						'^' => Some(Direction::Up),
						'>' => Some(Direction::Right),
						'v' => Some(Direction::Down),
						'<' => Some(Direction::Left),
						_ => None,
					} {
						init = Some(((py, px), dir));
					}
					ch == '#'
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let height = field.len();
	let width = if height == 0 {
		0
	} else {
		field[0].len()
	};
	Field {
		height,
		width,
		field,
		pos: init.unwrap().0,
		dir: init.unwrap().1,
	}
}

fn positions_visited(f: &Field) -> u32 {
	let mut visited = HashSet::new();
	let mut fld = f.clone();
	while fld.step() {
		visited.insert(fld.pos);
	}
	visited.len() as u32
}
