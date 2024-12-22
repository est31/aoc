use std::collections::{BTreeSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mp = parse(INPUT);
	println!("lowest score: {}", mp.lowest_score());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {
	fn turns(&self) -> [Dir; 2] {
		use Dir::*;
		match *self {
			Up => [Left, Right],
			Down => [Left, Right],
			Left => [Up, Down],
			Right => [Up, Down],
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Field {
	Start,
	End,
	Wall,
	Empty,
}

impl Field {
	fn ch(&self) -> char {
		match *self {
			Field::Start => 'S',
			Field::End => 'E',
			Field::Wall => '#',
			Field::Empty => '.',
		}
	}
}

fn coord_in_dir(p: (usize, usize), dir :Dir) -> (usize, usize) {
	match dir {
		Dir::Up => (p.0, p.1 - 1),
		Dir::Down => (p.0, p.1 + 1),
		Dir::Left => (p.0 - 1, p.1),
		Dir::Right => (p.0 + 1, p.1),
	}
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
	fields :Vec<Vec<Field>>,
	start_pos :(usize, usize),
	end_pos :(usize, usize),
}

fn parse(s :&str) -> Map {
	let mut lines = s.trim().lines().map(str::trim);
	let mut fields = Vec::new();
	while let Some(l) = lines.next() {
		if l.is_empty() {
			break;
		}
		let line = l.chars()
			.map(|ch| match ch {
				'S' => Field::Start,
				'E' => Field::End,
				'#' => Field::Wall,
				'.' => Field::Empty,
				_ => panic!("Unexpected char '{ch}'"),
			})
			.collect::<Vec<_>>();
		fields.push(line);
	}
	let mut start_pos = None;
	let mut end_pos = None;
	for (y, l) in fields.iter().enumerate() {
		for (x, fld) in l.iter().enumerate() {
			if *fld == Field::Start {
				start_pos = Some((x, y));
			}
			if *fld == Field::End {
				end_pos = Some((x, y));
			}
		}
	}
	Map {
		fields,
		start_pos: start_pos.unwrap(),
		end_pos: end_pos.unwrap(),
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

impl Map {
	fn lowest_score(&self) -> u32 {
		let mut handled = HashMap::new();
		let mut unhandled = BTreeSet::new();
		unhandled.insert((0, self.start_pos, Dir::Right));
		while let Some((cost, pos, dir)) = unhandled.pop_first() {
			if self.fields[pos.1][pos.0] == Field::Wall {
				continue;
			}
			if handled.contains_key(&(pos, dir)) {
				continue;
			}
			handled.insert((pos, dir), cost);
			if pos == self.end_pos {
				break;
			}
			let in_dir = coord_in_dir(pos, dir);
			let turns = dir.turns();
			unhandled.insert((cost + 1, in_dir, dir));
			for new_dir in turns {
				unhandled.insert((cost + 1000, pos, new_dir));
			}
		}
		let min = [Dir::Up, Dir::Down, Dir::Left, Dir::Right].into_iter()
			.filter_map(|d| handled.get(&(self.end_pos, d)))
			.min();
		*min.expect("no path from start to end found")
	}
}
