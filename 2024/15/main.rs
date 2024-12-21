use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mp = parse(INPUT);
	println!("gps coords: {}", mp.sum_gps_coords());
	println!("gps coords widened: {}", mp.sum_gps_coords_widened());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cmd {
	Up,
	Down,
	Left,
	Right,
}

impl Cmd {
	fn opposite(&self) -> Cmd {
		use Cmd::*;
		match *self {
			Up => Down,
			Down => Up,
			Left => Right,
			Right => Left,
		}
	}
	fn is_vert(&self) -> bool {
		matches!(self, Cmd::Up | Cmd::Down)
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Field {
	Box_,
	BoxLeft,
	BoxRight,
	Wall,
	Empty,
	Robot,
}

impl Field {
	fn widened(&self) -> [Field; 2] {
		use Field::*;
		match *self {
			Box_ => [BoxLeft, BoxRight],
			Wall => [Wall, Wall],
			Empty => [Empty, Empty],
			Robot => [Robot, Empty],
			BoxLeft | BoxRight => panic!("can't widen two times"),
		}
	}
	fn ch(&self) -> char {
		match *self {
			Field::Box_ => 'O',
			Field::BoxLeft => '[',
			Field::BoxRight => ']',
			Field::Wall => '#',
			Field::Empty => '.',
			Field::Robot => '@',
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
	fields :Vec<Vec<Field>>,
	commands :Vec<Cmd>,
	robot_pos :(usize, usize),
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
				'@' => Field::Robot,
				'O' => Field::Box_,
				'#' => Field::Wall,
				'.' => Field::Empty,
				_ => panic!("Unexpected char '{ch}'"),
			})
			.collect::<Vec<_>>();
		fields.push(line);
	}
	let mut robot_pos = None;
	for (y, l) in fields.iter().enumerate() {
		for (x, fld) in l.iter().enumerate() {
			if *fld == Field::Robot {
				robot_pos = Some((x, y));
			}
		}
	}
	let robot_pos = robot_pos.unwrap();
	let commands = lines
		.map(|l| l.chars()
			.map(|ch| match ch {
				'^' => Cmd::Up,
				'v' => Cmd::Down,
				'<' => Cmd::Left,
				'>' => Cmd::Right,
				_ => panic!("Unexpected char '{ch}'"),
			})
		)
		.flatten()
		.collect::<Vec<_>>();
	Map {
		fields,
		commands,
		robot_pos,
	}
}

fn coord_in_dir(p: (usize, usize), dir :Cmd) -> (usize, usize) {
	match dir {
		Cmd::Up => (p.0, p.1 - 1),
		Cmd::Down => (p.0, p.1 + 1),
		Cmd::Left => (p.0 - 1, p.1),
		Cmd::Right => (p.0 + 1, p.1),
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

impl Map {
	fn apply_cmd(&mut self, cmd :Cmd) {
		if !cmd.is_vert() {
			let mut sp = self.robot_pos;
			dprint!("sp start: {sp:?}\n");
			while !matches!(self.fields[sp.1][sp.0], Field::Wall | Field::Empty) {
				dprint!("    sp: {sp:?}\n");
				sp = coord_in_dir(sp, cmd);
			}
			if self.fields[sp.1][sp.0] == Field::Wall {
				// Don't do anything
				return;
			}
			assert_eq!(self.fields[sp.1][sp.0], Field::Empty);

			// Move in the opposite direction
			let opp = cmd.opposite();
			while sp != self.robot_pos {
				let np = coord_in_dir(sp, opp);
				self.fields[sp.1][sp.0] = self.fields[np.1][np.0];
				sp = np;
			}
			self.fields[sp.1][sp.0] = Field::Empty;
			self.robot_pos = coord_in_dir(self.robot_pos, cmd);
		} else {
			// Vertical direction: we need to put special care on boxes.

			// Walk in normal dir
			let mut heads = [self.robot_pos].into_iter()
				.collect::<HashSet<_>>();
			let mut list_of_heads = Vec::new();

			// See if we can move the potentially increasing set of heads
			while !heads.is_empty() {
				dprint!("heads: {heads:?}\n");
				let mut new_heads = HashSet::new();
				let mut hd_cl = heads.clone();
				for hd in heads.iter() {
					let at_hd = self.fields[hd.1][hd.0];
					match at_hd {
						// We can't continue, there is a wall
						Field::Wall => return,
						// Nothing, we end here
						Field::Empty => (),
						Field::Robot | Field::Box_ => {
							new_heads.insert(coord_in_dir(*hd, cmd));
						},
						Field::BoxLeft => {
							let rhd = (hd.0 + 1, hd.1);
							hd_cl.insert(rhd);
							assert_eq!(self.fields[rhd.1][rhd.0], Field::BoxRight);
							new_heads.insert(coord_in_dir(*hd, cmd));
							new_heads.insert(coord_in_dir(rhd, cmd));
						},
						Field::BoxRight => {
							let lhd = (hd.0 - 1, hd.1);
							hd_cl.insert(lhd);
							assert_eq!(self.fields[lhd.1][lhd.0], Field::BoxLeft);
							new_heads.insert(coord_in_dir(*hd, cmd));
							new_heads.insert(coord_in_dir(lhd, cmd));
						},
					}
				}
				list_of_heads.push(hd_cl);
				heads = new_heads;
			}
			// Now, we know that we can move the heads as otherwise we'd have returned
			let opp = cmd.opposite();
			list_of_heads.reverse();
			list_of_heads.push(HashSet::new());
			for heads_and_next in list_of_heads.windows(2) {
				let heads = &heads_and_next[0];
				let heads_next = &heads_and_next[1];
				dprint!("  heads: {heads:?}\n");
				for hd in heads {
					let np = coord_in_dir(*hd, opp);
					let to_put = if heads_next.contains(&np) {
						self.fields[np.1][np.0]
					} else {
						Field::Empty
					};
					dprint!("    upd: {hd:?}({:?}) <- {:?}\n",  self.fields[hd.1][hd.0], to_put);
					self.fields[hd.1][hd.0] = to_put;
				}
			}
			self.robot_pos = coord_in_dir(self.robot_pos, cmd);
		}
	}
	#[allow(unused)]
	fn print(&self) {
		for (_y, l) in self.fields.iter().enumerate() {
			for (_x, fld) in l.iter().enumerate() {
				print!("{}", fld.ch());
			}
			println!();
		}
		println!();
	}
	fn gps_coord_boxes(&self) -> u32 {
		let mut sum = 0;

		for (y, l) in self.fields.iter().enumerate() {
			for (x, fld) in l.iter().enumerate() {
				if !matches!(fld, Field::Box_ | Field::BoxLeft) { continue }
				let gps_coord = (y as u32 * 100) + x as u32;
				dprint!("sum({sum}) += {gps_coord}\n");
				sum += gps_coord;
			}
		}
		sum
	}
	fn sum_gps_coords_x_m1(&self) -> u32 {
		let mut cl = self.clone();
		for (_i, cmd) in self.commands.iter().enumerate() {
			//println!("Command {_i:03}: {cmd:?}");
			cl.apply_cmd(*cmd);
			//cl.print();
		}
		cl.gps_coord_boxes()
	}
	fn sum_gps_coords(&self) -> u32 {
		self.sum_gps_coords_x_m1()
	}
	fn sum_gps_coords_widened(&self) -> u32 {
		let wd = self.widen();
		wd.sum_gps_coords_x_m1()
	}
	fn widen(&self) -> Map {
		let fields = self.fields.iter()
			.map(|l|
				l.iter()
					.map(|fld| fld.widened().into_iter())
					.flatten()
					.collect::<Vec<_>>()
			)
			.collect::<Vec<_>>();
		Map {
			fields,
			commands: self.commands.clone(),
			robot_pos: (self.robot_pos.0 * 2, self.robot_pos.1),
		}
	}
}
