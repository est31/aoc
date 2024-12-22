use std::collections::{BTreeSet, HashMap, HashSet, hash_map::Entry};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mp = parse(INPUT);
	println!("lowest score: {}", mp.lowest_score());
	println!("shortest count: {}", mp.tiles_shortest_count());
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
	fn search(&self) -> (u32, u32) {
		let mut handled = HashMap::new();
		let mut unhandled = BTreeSet::new();
		let fake_st_prev = (self.start_pos, Dir::Right);
		unhandled.insert(((0, self.start_pos, Dir::Right), fake_st_prev));
		let mut stop_at_cost = None;
		while let Some(((cost, pos, dir), prev)) = unhandled.pop_first() {
			if self.fields[pos.1][pos.0] == Field::Wall {
				continue;
			}
			match handled.entry((pos, dir)) {
				Entry::Vacant(v) => {
					v.insert((cost, vec![prev]));
				},
				Entry::Occupied(mut o) => {
					let o = o.get_mut();
					if o.0 == cost {
						o.1.push(prev);
					}
					continue;
				},
			}
			if let Some(stop_at) = &stop_at_cost {
				if cost > *stop_at {
					break;
				}
			} else {
				if pos == self.end_pos {
					stop_at_cost = Some(cost);
				}
			}
			let new_prev = (pos, dir);
			let in_dir = coord_in_dir(pos, dir);
			let turns = dir.turns();
			unhandled.insert(((cost + 1, in_dir, dir), new_prev));
			for new_dir in turns {
				unhandled.insert(((cost + 1000, pos, new_dir), new_prev));
			}
		}

		let possible_ends = [Dir::Up, Dir::Down, Dir::Left, Dir::Right].into_iter()
			.filter_map(|d| handled.get(&(self.end_pos, d)))
			.collect::<Vec<_>>();
		dprint!("possible ends: {possible_ends:?}\n");
		let min = possible_ends.iter().map(|(cost, _from)| cost).min();
		let lowest_score = *min.expect("no path from start to end found");
		// Determine lowest score
		let ends_with_score = possible_ends.iter()
			.filter(|(score, _from)| {
				*score == lowest_score
			})
			.map(|(_score, from)| {
				from.iter().map(|v| *v)
			})
			.flatten()
			.collect::<Vec<_>>();

		let mut shortest_p = HashSet::<(usize, usize)>::new();
		let mut handled_reverse = HashSet::new();
		let mut to_visit = ends_with_score;
		while let Some(p) = to_visit.pop() {
			if handled_reverse.contains(&p) {
				continue;
			}
			handled_reverse.insert(p);
			dprint!("  visiting: {p:?}\n");
			dprint!("  -> adding\n");
			shortest_p.insert(p.0);
			to_visit.extend_from_slice(&handled[&p].1);
		}

		let shortest_count = shortest_p.len() as u32 + 1;
		(lowest_score, shortest_count)
	}
	fn lowest_score(&self) -> u32 {
		self.search().0
	}
	fn tiles_shortest_count(&self) -> u32 {
		self.search().1
	}
}
