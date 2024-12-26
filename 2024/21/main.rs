use std::cmp::Ordering;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cds = parse(INPUT);
	println!("sum of complexities: {}", sum_complexities(&cds));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum PosNum {
	Digit(u8),
	A,
}

impl PosNum {
	fn coord(&self) -> (i8, i8) {
		match self {
			PosNum::Digit(0) => (1, 3),
			PosNum::A => (2, 3),
			PosNum::Digit(1) => (0, 2),
			PosNum::Digit(2) => (1, 2),
			PosNum::Digit(3) => (2, 2),
			PosNum::Digit(4) => (0, 1),
			PosNum::Digit(5) => (1, 1),
			PosNum::Digit(6) => (2, 1),
			PosNum::Digit(7) => (0, 0),
			PosNum::Digit(8) => (0, 0),
			PosNum::Digit(9) => (0, 0),
			PosNum::Digit(_) => panic!("Invalid digit"),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Pos {
	Up,
	A,
	Left,
	Down,
	Right,
}

impl Pos {
	fn coord(&self) -> (i8, i8) {
		match self {
			Pos::Up => (1, 0),
			Pos::A => (2, 0),
			Pos::Left => (0, 1),
			Pos::Down => (1, 1),
			Pos::Right => (2, 1),
		}
	}
}

impl std::fmt::Display for Pos {
	fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Pos::Up => write!(f, "^"),
			Pos::A => write!(f, "A"),
			Pos::Left => write!(f, "<"),
			Pos::Down => write!(f, "v"),
			Pos::Right => write!(f, ">"),
		}
	}
}

fn parse(s :&str) -> Vec<Vec<PosNum>> {
	s.trim()
		.lines()
		.map(str::trim)
		.map(|l| {
			l.chars()
				.map(|c| match c {
					'0'..='9' => PosNum::Digit((c as u8 - b'0') as u8),
					'A' => PosNum::A,
					_ => panic!("Didn't expect {c}"),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

macro_rules! dprint {
	($($args:expr),*) => {
		//if false
			{ print!($($args),*); }
	};
}

fn shortest_for_pos_num(code :&[PosNum]) -> Vec<Pos> {
	let mut cmds = Vec::new();
	for wnd in code.windows(2) {
		let cd_from = wnd[0];
		let cd_to = wnd[1];
		let coord_from = cd_from.coord();
		let coord_to = cd_to.coord();
		let x_cmds = match coord_from.0.cmp(&coord_to.0) {
			Ordering::Less => {
				vec![Pos::Right; (coord_to.0 - coord_from.0) as usize]
			}
			Ordering::Equal => {
				vec![]
			}
			Ordering::Greater => {
				vec![Pos::Left; (coord_from.0 - coord_to.0) as usize]
			}
		};
		match coord_from.1.cmp(&coord_to.1) {
			Ordering::Less => {
				cmds.extend_from_slice(&x_cmds);
				cmds.extend_from_slice(&vec![Pos::Down; (coord_to.1 - coord_from.1) as usize]);
			}
			Ordering::Equal => (),
			Ordering::Greater => {
				cmds.extend_from_slice(&vec![Pos::Up; (coord_from.1 - coord_to.1) as usize]);
				cmds.extend_from_slice(&x_cmds);
			}
		};
		cmds.push(Pos::A);
	}
	dprint!("numpad cmds: {}\n", cmds.iter().map(|s| format!("{s}")).collect::<String>());
	cmds
}

fn shortest_remote_one(code :&[Pos]) -> Vec<Pos> {
	let mut cmds = Vec::new();
	for wnd in code.windows(2) {
		let cd_from = wnd[0];
		let cd_to = wnd[1];
		let coord_from = cd_from.coord();
		let coord_to = cd_to.coord();
		let x_cmds = match coord_from.0.cmp(&coord_to.0) {
			Ordering::Less => {
				vec![Pos::Right; (coord_to.0 - coord_from.0) as usize]
			}
			Ordering::Equal => {
				vec![]
			}
			Ordering::Greater => {
				vec![Pos::Left; (coord_from.0 - coord_to.0) as usize]
			}
		};
		match coord_from.1.cmp(&coord_to.1) {
			Ordering::Less => {
				cmds.extend_from_slice(&vec![Pos::Down; (coord_to.1 - coord_from.1) as usize]);
				cmds.extend_from_slice(&x_cmds);
			}
			Ordering::Equal => (),
			Ordering::Greater => {
				cmds.extend_from_slice(&x_cmds);
				cmds.extend_from_slice(&vec![Pos::Up; (coord_from.1 - coord_to.1) as usize]);
			}
		};
		cmds.push(Pos::A);
	}
	dprint!("robot cmds: {}\n", cmds.iter().map(|s| format!("{s}")).collect::<String>());
	cmds
}

fn shortest_press_seq(code :&[PosNum]) -> Vec<Pos> {
	let code_0 = shortest_for_pos_num(code);
	let code_1 = shortest_remote_one(&code_0);
	let code_2 = shortest_remote_one(&code_1);
	let final_code = shortest_remote_one(&code_2);
	final_code
}

fn code_to_num(code :&[PosNum]) -> u32 {
	let mut r = 0;
	for v in code.iter() {
		let PosNum::Digit(v) = v else { continue };
		r *= 10;
		r += *v as u32;
	}
	r
}

fn complexity(code :&[PosNum]) -> u32 {
	shortest_press_seq(code).len() as u32 * code_to_num(code)
}

fn sum_complexities(list :&[Vec<PosNum>]) -> u32 {
	list.iter()
		.map(|c| complexity(c))
		.sum()
}
