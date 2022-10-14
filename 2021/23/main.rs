use std::collections::{BTreeMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let scene = Scene::parse(INPUT);
	let mut search = SceneSearch::new(scene);
	let energy = search.search();
	println!("Energy required to organize: {energy}");
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Field {
	Empty(char),
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl Field {
	fn ch(&self) -> char {
		match self {
			Field::Empty(ch) => *ch,
			Field::Amber => 'A',
			Field::Bronze => 'B',
			Field::Copper => 'C',
			Field::Desert => 'D',
		}
	}
	fn from_ch(ch: char) -> Option<Self> {
		Some(match ch {
			'.' | '#' | ' ' => Field::Empty(ch),
			'A' => Field::Amber,
			'B' => Field::Bronze,
			'C' => Field::Copper,
			'D' => Field::Desert,
			_ => return None,
		})
	}
	fn end_pos_col(&self) -> Option<usize> {
		Some(match self {
			Field::Empty(_) => return None,
			Field::Amber => 2,
			Field::Bronze => 4,
			Field::Copper => 6,
			Field::Desert => 8,
		})
	}
	fn cost_mul(&self) -> Option<u32> {
		Some(match self {
			Field::Empty(_) => return None,
			Field::Amber => 1,
			Field::Bronze => 10,
			Field::Copper => 100,
			Field::Desert => 1000,
		})
	}
	fn is_empty(&self) -> bool {
		matches!(self, Field::Empty(_))
	}
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Scene {
	fields :[[Field; 11]; 3],
	imperfect_amphipods :Vec<(usize, usize)>,
}

impl Scene {
	fn parse(input :&str) -> Self {
		let mut fields = [[Field::Empty('.'); 11]; 3];
		let mut lines = input.lines();
		assert_eq!(lines.next(), Some("#############"));
		let mut imperfect_amphipods = Vec::new();
		for (line, fl) in lines.zip(fields.iter_mut()) {
			let mut ch_it = line.chars();
			ch_it.next();
			for (ch, field) in ch_it.zip(fl.iter_mut()) {
				let Some(fl) = Field::from_ch(ch) else {
					panic!("None of expected characters '{ch}'");
				};
				*field = fl;
			}
		}
		for (linei, fl) in fields.iter().enumerate() {
			for (coli, field) in fl.iter().enumerate() {
				let Some(epc) = field.end_pos_col() else {
					continue;
				};
				if epc == coli {
					if linei == 2 || (linei == 1 && fields[2][coli] == *field) {
						continue;
					}
				}
				//println!("Imperfect {field:?} at {linei}");
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
	fn moves_for(&self, (linei, coli) :(usize, usize)) -> Vec<(u32, (usize, usize))> {
		let kind = self.fields[linei][coli];
		let end_col = kind.end_pos_col().unwrap();
		let cost_mul = kind.cost_mul().unwrap();
		match linei {
			0 => {
				// Move from the hallway into the end destination (if possible)
				// Whether we can descend in the destination, and if yes,
				// the descend distance.
				let land_distance = {
					let f1 = self.fields[1][end_col];
					let f2 = self.fields[2][end_col];
					if !f1.is_empty() {
						return Vec::new();
					} else {
						if f2.is_empty() {
							2
						} else if f2 == kind {
							1
						} else {
							return Vec::new();
						}
					}
				};
				if self.hallway_is_free(coli, end_col) {
					let cost = ((end_col as isize) - (coli as isize)).abs() as u32;
					let cost = cost + land_distance;
					let end_line = land_distance as usize;
					vec![(cost * cost_mul, (end_line, end_col))]
				} else {
					Vec::new()
				}
			},
			1 | 2 => {
				// Move from the start destination to somewhere (legal) into the hallway
				// Whether we can ascend to the hallway, and if yes, the cost
				let ascent_cost = if linei == 2 {
					if !self.fields[1][coli].is_empty() {
						return Vec::new();
					}
					2
				} else {
					1
				};
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
			_ => unreachable!(),
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
		write!(f, " ")?;
		for field in &fields[2][..10] {
			write!(f, "{}", field.ch())?;
		}
		writeln!(f, "")?;
		writeln!(f, "  #########")?;
		Ok(())
	}
}

struct SceneSearch {
	scenes_encountered :HashSet<Scene>,
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
		if self.scenes_encountered.contains(&sc) {
			return;
		}
		let entries = self.scenes_with_cost.entry(cost).or_default();
		entries.insert(sc);
	}
	fn step(&mut self) -> Option<u32> {
		let (cost, scenes_to_develop) = self.scenes_with_cost.pop_first()?;
		self.scenes_encountered.extend(scenes_to_develop.iter().cloned());
		for sc in scenes_to_develop {
			if sc.is_perfect() {
				return Some(cost);
			}
			for ipa in sc.imperfect_amphipods.clone() {
				for (acost, mv) in sc.moves_for(ipa) {
					let mut nsc = sc.clone();
					nsc.perform_move_for(ipa, mv);
					let ncost = acost + cost;
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
	fn search_for_steps(&mut self, steps :u32) -> Option<u32> {
		for _ in 0..steps {
			if let Some(s) = self.step() {
				return Some(s);
			}
		}
		None
	}
}
