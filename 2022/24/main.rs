use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	let l = find_shortest_path(&mut field.clone());
	println!("Shortest path len: {l}");
	let lbf = find_shortest_path_back_forth(&mut field.clone());
	println!("Shortest path len: {lbf}");
}

#[derive(Copy, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn from_ch(ch: char) -> Option<Direction> {
		Some(match ch {
			'^' => Direction::Up,
			'v' => Direction::Down,
			'<' => Direction::Left,
			'>' => Direction::Right,
			_ => return None,
		})
	}
}

#[derive(Copy, Clone)]
enum FieldEntry {
	Free,
	Blizzard(Direction),
	Multiple(u8),
}

impl FieldEntry {
	fn ch(&self) -> char {
		match self {
			FieldEntry::Free => '.',
			FieldEntry::Blizzard(Direction::Up) => '^',
			FieldEntry::Blizzard(Direction::Down) => 'v',
			FieldEntry::Blizzard(Direction::Left) => '<',
			FieldEntry::Blizzard(Direction::Right) => '>',
			FieldEntry::Multiple(v) => {
				assert!(*v < 10);
				(*v + b'0').into()
			},
		}
	}
	fn is_free(&self) -> bool {
		matches!(self, FieldEntry::Free)
	}
}

#[derive(Clone)]
struct FieldEntries(Vec<Vec<FieldEntry>>);

impl std::fmt::Display for FieldEntries {
	fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "#.")?;
		for _ in 0..self.0[0].len() {
			write!(f, "#")?;
		}
		writeln!(f, "")?;
		for line in self.0.iter() {
			write!(f, "#")?;
			for fld in line.iter() {
				write!(f, "{}", fld.ch())?;
			}
			writeln!(f, "#")?;
		}
		for _ in 0..self.0[0].len() {
			write!(f, "#")?;
		}
		write!(f, ".#")?;
		writeln!(f, "")?;
		Ok(())
	}
}

#[derive(Clone)]
struct Field {
	width: u16,
	height: u16,
	blizzards: Vec<(u16, u16, Direction)>,
	fields: FieldEntries,
}

impl Field {
	fn step(&mut self) {
		for (x, y, bl_dir) in self.blizzards.iter_mut() {
			match bl_dir {
				Direction::Up => {
					if *y == 0 {
						*y = self.height - 1;
					} else {
						*y -= 1;
					}
				},
				Direction::Down => {
					if *y == self.height - 1 {
						*y = 0;
					} else {
						*y += 1;
					}
				},
				Direction::Left => {
					if *x == 0 {
						*x = self.width - 1;
					} else {
						*x -= 1;
					}
				},
				Direction::Right => {
					if *x == self.width - 1 {
						*x = 0;
					} else {
						*x += 1;
					}
				},
			}
		}
		self.update_fields();
	}
	fn update_fields(&mut self) {
		// First, clear them
		for row in self.fields.0.iter_mut() {
			for field in row.iter_mut() {
				*field = FieldEntry::Free;
			}
		}
		// Then, fill them
		for (x, y, bl_dir) in self.blizzards.iter() {
			let fld = &mut self.fields.0[*y as usize][*x as usize];
			*fld = match *fld {
				FieldEntry::Free => FieldEntry::Blizzard(*bl_dir),
				FieldEntry::Blizzard(_bl_dir) => FieldEntry::Multiple(2),
				FieldEntry::Multiple(v) => FieldEntry::Multiple(v + 1),
			};
		}
	}
}

fn parse(input :&str) -> Field {
	let mut lines = input.lines()
		.filter(|l| !l.is_empty());
	let first_line = lines.next().unwrap();
	let width = first_line.len() as u16 - 2;
	let height = lines.clone().count() as u16 - 1;
	let blizzards = lines.enumerate()
		.map(|(i, l)| {
			l.chars()
				.enumerate()
				.filter(|(_j, ch)| *ch != '#' && *ch != '.')
				.map(move |(j, ch)| {
					let dir = Direction::from_ch(ch).unwrap();
					(j as u16 - 1, i as u16, dir)
				})
		})
		.flatten()
		.collect::<Vec<(u16, u16, Direction)>>();
	let fields = FieldEntries(vec![vec![FieldEntry::Free; width.into()]; height.into()]);
	let mut res = Field {
		height,
		width,
		blizzards,
		fields,
	};
	res.update_fields();
	res
}

fn positions_to_move_to(pos :(u16, u16), field :&Field) -> Vec<(u16, u16)> {
	let mut res = Vec::with_capacity(5);
	if pos.0 > 0 {
		res.push((pos.0 - 1, pos.1));
	}
	if pos.0 < field.width - 1 {
		res.push((pos.0 + 1, pos.1));
	}
	if pos.1 > 0 {
		res.push((pos.0, pos.1 - 1));
	}
	if pos.1 < field.height - 1 {
		res.push((pos.0, pos.1 + 1));
	}
	res.push(pos);
	res
}

fn find_shortest_path(field :&mut Field) -> u32 {
	let start = (0, 0);
	let goal = (field.width - 1, field.height - 1);
	find_shortest_path_generic(start, field, goal) + 1
}

fn find_shortest_path_back_forth(field :&mut Field) -> u32 {
	let start = (0, 0);
	let goal = (field.width - 1, field.height - 1);

	let p1 = find_shortest_path_generic(start, field, goal);
	let p2 = find_shortest_path_generic(goal, field, start);
	let p3 = find_shortest_path_generic(start, field, goal);

	p1 + p2 + p3 + 3
}

fn find_shortest_path_generic(start :(u16, u16), field :&mut Field, goal :(u16, u16)) -> u32 {
	let mut possible_positions = HashSet::new();
	let mut step_num = 0;
	loop {
		// We can always just wait some number of steps in the start
		if field.fields.0[start.1 as usize][start.0 as usize].is_free() {
			possible_positions.insert(start);
		}
		field.step();
		//println!("step {step_num}. {} positions: {:?}\n{}", possible_positions.len(), possible_positions, field.fields);
		let old_positions = std::mem::take(&mut possible_positions);
		for pos in old_positions {
			let mt = positions_to_move_to(pos, field);
			//println!("  -> {}", mt.len());
			for new_pos in mt {
				let content = field.fields.0[new_pos.1 as usize][new_pos.0 as usize];
				if !content.is_free() {
					//println!("     {:?} is not free", new_pos);
					continue;
				}
				if pos == goal {
					return step_num;
				}
				possible_positions.insert(new_pos);
			}
		}
		step_num += 1;
	}
}
