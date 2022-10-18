use std::collections::{BTreeMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let scene = Scene::parse(INPUT);
	let mut search = SceneSearch::new(scene);
	let energy = search.search();
	println!("Energy required to organize: {energy}");

	let scene_uf = Scene::parse_with_unfolded(INPUT, true);
	let mut search_uf = SceneSearch::new(scene_uf);
	let energy_uf = search_uf.search();
	println!("Energy required to organize for unfolded: {energy_uf}");
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Field {
	Outside,
	Wall,
	Empty,
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl Field {
	fn ch(&self) -> char {
		match self {
			Field::Outside => ' ',
			Field::Wall => '#',
			Field::Empty => '.',
			Field::Amber => 'A',
			Field::Bronze => 'B',
			Field::Copper => 'C',
			Field::Desert => 'D',
		}
	}
	fn from_ch(ch: char) -> Option<Self> {
		Some(match ch {
			' ' => Field::Outside,
			'#' => Field::Wall,
			'.' => Field::Empty,
			'A' => Field::Amber,
			'B' => Field::Bronze,
			'C' => Field::Copper,
			'D' => Field::Desert,
			_ => return None,
		})
	}
	fn end_pos_col(&self) -> Option<usize> {
		Some(match self {
			Field::Outside | Field::Wall | Field::Empty => return None,
			Field::Amber => 2,
			Field::Bronze => 4,
			Field::Copper => 6,
			Field::Desert => 8,
		})
	}
	fn cost_mul(&self) -> Option<u32> {
		Some(match self {
			Field::Outside | Field::Wall | Field::Empty => return None,
			Field::Amber => 1,
			Field::Bronze => 10,
			Field::Copper => 100,
			Field::Desert => 1000,
		})
	}
	fn is_empty(&self) -> bool {
		matches!(self, Field::Empty)
	}
}

fn parse_line(line :&str) -> [Field; 11] {
	let mut fields = [Field::Empty; 11];
	let mut ch_it = line.chars();
	ch_it.next();
	for (ch, field) in ch_it.zip(fields.iter_mut()) {
		let Some(fl) = Field::from_ch(ch) else {
			panic!("None of expected characters '{ch}'");
		};
		*field = fl;
	}
	fields
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Scene {
	fields :Vec<[Field; 11]>,
	imperfect_amphipods :Vec<(usize, usize)>,
}

impl Scene {
	fn parse(input :&str) -> Self {
		Self::parse_with_unfolded(input, false)
	}
	fn parse_with_unfolded(input :&str, unfolded :bool) -> Self {
		let mut fields = Vec::new();
		let mut lines = input.lines();
		assert_eq!(lines.next(), Some("#############"));
		let mut imperfect_amphipods = Vec::new();
		for (linei, line) in lines.enumerate().take(3) {
			if unfolded && linei == 2 {
				fields.push(parse_line("  #D#C#B#A#"));
				fields.push(parse_line("  #D#B#A#C#"));
			}
			fields.push(parse_line(line));
		}
		for (linei, fl) in fields.iter().enumerate() {
			for (coli, field) in fl.iter().enumerate() {
				let Some(epc) = field.end_pos_col() else {
					continue;
				};
				if epc == coli {
					let Some(lines_below) = fields.get((linei + 1)..) else { continue };
					if lines_below.iter().all(|l| l[coli] == *field) {
						continue;
					}
				}
				// println!("Imperfect {field:?} at {linei},{coli}");
				imperfect_amphipods.push((linei, coli));
			}
		}
		Self {
			fields,
			imperfect_amphipods,
		}
	}
	fn is_perfect(&self) -> bool {
		self.imperfect_amphipods.is_empty()
	}
	fn can_move_home(&self, coli :usize, kind :Field, end_col :usize, cost_mul :u32) -> Option<(u32, (usize, usize))> {
		// Move from the hallway into the end destination (if possible)

		// Whether we can descend in the destination, and if yes,
		// the descend distance.
		let land_distance = {
			let mut descend_dist = None;
			let can_descend = self.fields.iter()
				.enumerate()
				.all(|(i, fl)| {
					let field = fl[end_col];
					if descend_dist.is_none() {
						if field.is_empty() {
							true
						} else if field == kind {
							descend_dist = Some(i as u32 - 1);
							true
						} else {
							false
						}
					} else {
						field == kind
					}
				});
			// TODO simplify this with let chains once available
			if !can_descend {
				return None;
			} else if let Some(dist) = descend_dist {
				// Check whether nothing is empty. This shouldn't
				// really occur, as can_descend should have been false.
				assert!(dist != 0, "Nothing is empty for descent of 0,{coli} at {end_col} in scene:\n{self}");
				dist
			} else {
				// Everything is empty
				self.fields.len() as u32 - 1
			}
		};
		if self.hallway_is_free(coli, end_col) {
			let cost = ((end_col as isize) - (coli as isize)).abs() as u32;
			let cost = cost + land_distance;
			let end_line = land_distance as usize;
			Some((cost * cost_mul, (end_line, end_col)))
		} else {
			None
		}
	}
	fn moves_for(&self, (linei, coli) :(usize, usize)) -> Vec<(u32, (usize, usize))> {
		let kind = self.fields[linei][coli];
		let end_col = kind.end_pos_col().unwrap();
		let cost_mul = kind.cost_mul().unwrap();
		match linei {
			0 => {
				if let Some(cost) = self.can_move_home(coli, kind, end_col, cost_mul) {
					vec![cost]
				} else {
					Vec::new()
				}
			},
			_ => {
				// Move from the start destination to somewhere (legal) into the hallway
				let ascent_cost = linei as u32;

				// Whether we can ascend to the hallway.
				// We only have to check the one field direcly above us,
				// As we never store scenes with gaps in the side rooms.
				if !self.fields[linei - 1][coli].is_empty() {
					return Vec::new();
				}

				// If we can move home immediately from here,
				// we know that the direct route is the shortest,
				// and waiting in the hallway won't bring any improvement
				// for anybody. So do the movement just right away,
				// which saves costs.
				if let Some((cost, dst)) = self.can_move_home(coli, kind, end_col, cost_mul) {
					return vec![(cost + cost_mul * ascent_cost, dst)];
				}

				// Legal columns in the halway to stop
				let legal_hallway_cols = [0, 1, 3, 5, 7, 9, 10];
				legal_hallway_cols.into_iter()
					.filter(|to_col| self.hallway_is_free(coli, *to_col))
					.map(|to_col| {
						let cost = ((to_col as isize) - (coli as isize)).abs() as u32;
						let cost = ascent_cost + cost;
						(cost * cost_mul, (0, to_col))
					})
					.collect::<Vec<_>>()
			},
		}
	}
	fn perform_move_for(&mut self, from :(usize, usize), to :(usize, usize)) {
		let field = self.fields[from.0][from.1];
		self.fields[from.0][from.1] = self.fields[to.0][to.1];
		self.fields[to.0][to.1] = field;
		if let Some(idx) = self.imperfect_amphipods.iter().position(|v| v == &from) {
			// This is a simpler check as we only move to the dest column *if* we actually do a descent to the final position
			if Some(to.1) == field.end_pos_col() {
				self.imperfect_amphipods.remove(idx);
			} else {
				self.imperfect_amphipods[idx] = to;
			}
		}
	}
	fn hallway_is_free(&self, from_col :usize, to_col :usize) -> bool {
		if from_col == to_col {
			return true;
		}
		let st = ((to_col as isize) - (from_col as isize)).signum();
		let mut col = from_col as isize;
		col += st;
		while self.fields[0][col as usize].is_empty() {
			if col == to_col as isize {
				return true;
			}
			col += st;
		}
		false
	}
	fn hallway_blocking_each_other(&self) -> bool {
		for (f_col, f) in self.fields[0].iter().enumerate() {
			let Some(f_ep) = f.end_pos_col() else {
				continue;
			};
			if f_ep < f_col {
				continue;
			}
			// Array lookup is safe because end position is always in range
			for b in self.fields[0][f_col + 1.. f_ep].iter() {
				// TODO use let chains here
				if let Some(b_ep) = b.end_pos_col() {
					if b_ep < f_col {
						//println!("Found blocking hallway:\n{self}");
						return true;
					}
				}
			}
		}
		false
	}
}

impl std::fmt::Display for Scene {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let fields = &self.fields;
		writeln!(f, "#############")?;
		write!(f, "#")?;
		for field in fields[0].iter() {
			write!(f, "{}", field.ch())?;
		}
		writeln!(f, "#")?;
		write!(f, "#")?;
		for field in &fields[1] {
			write!(f, "{}", field.ch())?;
		}
		writeln!(f, "#")?;
		for fl in &fields[2..] {
			write!(f, " ")?;
			for field in &fl[..10] {
				write!(f, "{}", field.ch())?;
			}
			writeln!(f, "")?;
		}
		writeln!(f, "  #########")?;
		Ok(())
	}
}

struct SceneSearch {
	scenes_encountered :HashSet<Vec<[Field; 11]>>,
	scenes_with_cost :BTreeMap<u32, HashSet<Scene>>,
}

impl SceneSearch {
	fn new(sc: Scene) -> Self {
		let mut ret = Self {
			scenes_encountered : HashSet::new(),
			scenes_with_cost : BTreeMap::new(),
		};
		ret.add_scene_with_cost(sc, 0);
		ret
	}
	fn add_scene_with_cost(&mut self, sc: Scene, cost: u32) {
		if self.scenes_encountered.contains(&sc.fields) {
			return;
		}
		let entries = self.scenes_with_cost.entry(cost).or_default();
		entries.insert(sc);
	}
	fn step(&mut self) -> Option<u32> {
		let (cost, scenes_to_develop) = self.scenes_with_cost.pop_first()?;
		self.scenes_encountered.extend(scenes_to_develop.iter().map(|sc| sc.fields.clone()));
		for sc in scenes_to_develop {
			if sc.is_perfect() {
				return Some(cost);
			}
			for ipa in sc.imperfect_amphipods.clone() {
				for (acost, mv) in sc.moves_for(ipa) {
					let mut nsc = sc.clone();
					nsc.perform_move_for(ipa, mv);
					let ncost = acost + cost;
					if nsc.hallway_blocking_each_other() {
						continue;
					}
					//println!("Adding scene with cost {ncost} and move {ipa:?} -> {mv:?}:\n{nsc}");
					self.add_scene_with_cost(nsc, ncost);
				}
			}
		}
		None
	}
	fn search(&mut self) -> u32 {
		loop {
			if let Some(s) = self.step() {
				return s;
			}
		}
	}
	#[cfg(test)]
	fn search_for_steps(&mut self, steps :u32) -> Option<u32> {
		for _ in 0..steps {
			if let Some(s) = self.step() {
				return Some(s);
			}
		}
		None
	}
}
