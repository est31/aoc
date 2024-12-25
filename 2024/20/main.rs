use std::collections::{BTreeSet, HashMap, HashSet, hash_map::Entry};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mp = parse(INPUT);
	println!("cheats saving >= 100 picos: {}", mp.count_cheats_100());
	println!("long cheats saving >= 100 picos: {}", mp.count_long_cheats_100());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Field {
	Start,
	End,
	Wall,
	Empty,
}

type Pos = (usize, usize);

#[derive(Clone, PartialEq, Eq)]
struct Map {
	fields :Vec<Vec<Field>>,
	start_pos :Pos,
	end_pos :Pos,
	height :usize,
	width :usize,
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
	let height = fields.len();
	let width = fields[0].len();
	Map {
		fields,
		start_pos: start_pos.unwrap(),
		end_pos: end_pos.unwrap(),
		height,
		width,
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

#[inline(always)]
fn neighs(p: (usize, usize), height :usize, width :usize) -> Vec<(usize, usize)> {
	let mut neighs = Vec::with_capacity(4);
	if p.0 > 0 {
		neighs.push((p.0 - 1, p.1));
	}
	if p.1 > 0 {
		neighs.push((p.0, p.1 - 1));
	}
	if p.0 + 1 < height {
		neighs.push((p.0 + 1, p.1));
	}
	if p.1 + 1 < width {
		neighs.push((p.0, p.1 + 1));
	}
	neighs
}

#[inline(always)]
fn neighs_manhattan(p: Pos, height :usize, width :usize, radius :usize) -> Vec<(Pos, usize)> {
	let mut neighs = Vec::new();
	if p.0 >= width || p.1 >= height {
		panic!("pos {p:?} out of bounds");
	}
	for y in (p.0.saturating_sub(radius))..=((width - 1).min(p.0 + radius)) {
		let d = (p.0 as isize - y as isize).abs() as usize;
		let lim = radius - d;
		for x in p.1.saturating_sub(lim)..=((height - 1).min(p.1 + lim)) {
			let xd = (p.1 as isize - x as isize).abs() as usize;
			if (y, x) == p { continue }
			neighs.push(((y, x), d + xd));
		}
	}
	neighs
}

#[cfg(test)]
fn count_cheats_saving(cost_no_cheat :u32, cheats_db :&HashMap<(Pos, Pos), u32>, saving :u32) -> u32 {
	cheats_db.iter()
		.filter(|(_, len)| cost_no_cheat - *len == saving)
		.count() as u32
}

impl Map {
	fn search(&self) -> u32 {
		self.shortest_tree()[&self.end_pos].0
	}
	fn shortest_tree(&self) -> HashMap<Pos, (u32, Pos)> {
		let mut handled = HashMap::new();
		let mut unhandled = BTreeSet::new();
		let fake_st_prev = self.start_pos;
		unhandled.insert(((0, self.start_pos), fake_st_prev));
		while let Some(((cost, pos), prev)) = unhandled.pop_first() {
			if self.fields[pos.1][pos.0] == Field::Wall {
				continue;
			}
			match handled.entry(pos) {
				Entry::Vacant(v) => {
					v.insert((cost, prev));
				},
				Entry::Occupied(_) => {
					continue;
				},
			}
			if pos == self.end_pos {
				return handled;
			}
			let new_prev = pos;
			for neigh in neighs(pos, self.height, self.width) {
				unhandled.insert(((cost + 1, neigh), new_prev));
			}
		}
		panic!("unreachable")
	}
	fn shortest_path(&self) -> Vec<Pos> {
		let tr = self.shortest_tree();

		let mut r = vec![self.end_pos];
		let mut last = self.end_pos;
		while last != self.start_pos {
			last = tr[&last].1;
			r.push(last);
		}
		r.reverse();
		r
	}
	fn make_cheats_db(&self) -> HashMap<(Pos, Pos), u32> {
		self.make_cheats_db_dist(2)
	}
	fn make_cheats_db_long(&self) -> HashMap<(Pos, Pos), u32> {
		self.make_cheats_db_dist(20)
	}
	fn make_cheats_db_dist(&self, max_cheat_dist :usize) -> HashMap<(Pos, Pos), u32> {
		let path = self.shortest_path();
		dprint!("path len no cheats: {}\n", path.len() - 1);
		let inverse_tree = {
			let mut mp = self.clone();
			std::mem::swap(&mut mp.end_pos, &mut mp.start_pos);
			mp.shortest_tree()
		};

		let mut db = HashMap::new();
		for (start_cost, p) in path.iter().cloned().enumerate() {
			for (p_nn, dist) in neighs_manhattan(p, self.height, self.width, max_cheat_dist) {
				let Some(inv_entry) = inverse_tree.get(&p_nn) else { continue };
				if p_nn == p { continue; }
				let rest_cost = inv_entry.0;
				let cost = start_cost as u32 + rest_cost + dist as u32;
				if cost < path.len() as u32 - 1 {
					dprint!("add to db {p:?}->{p_nn:?}: {start_cost} + {rest_cost} + {dist} = {cost}\n");
					db.insert((p, p_nn), cost);
				}
			}
		}
		db
	}
	fn count_cheats_100(&self) -> u32 {
		let cost_no_cheat = self.search();
		let db = self.make_cheats_db();
		db.iter()
			.filter(|(_, len)| cost_no_cheat - *len >= 100)
			.count() as u32
	}
	fn count_long_cheats_100(&self) -> u32 {
		let cost_no_cheat = self.search();
		let db = self.make_cheats_db_long();
		db.iter()
			.filter(|(_, len)| cost_no_cheat - *len >= 100)
			.count() as u32
	}
}
