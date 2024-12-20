const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mp = parse(INPUT);
	println!("safety factor after 100 seconds: {}", mp.sum_gps_coords());
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
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Field {
	Box_,
	Wall,
	Empty,
	Robot,
}

impl Field {
	fn ch(&self) -> char {
		match *self {
			Field::Box_ => 'O',
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
		let mut sp = self.robot_pos;
		while !matches!(self.fields[sp.1][sp.0], Field::Wall | Field::Empty) {
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
				if !matches!(fld, Field::Box_ ) { continue }
				let gps_coord = (y as u32 * 100) + x as u32;
				dprint!("sum({sum}) += {gps_coord}\n");
				sum += gps_coord;
			}
		}
		sum
	}
	fn sum_gps_coords(&self) -> u32 {
		let mut cl = self.clone();
		for cmd in &self.commands {
			//println!("Command: {cmd:?}");
			//cl.print();
			cl.apply_cmd(*cmd);
		}
		cl.gps_coord_boxes()
	}
}
