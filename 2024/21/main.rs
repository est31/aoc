use std::collections::HashMap;
use std::cmp::Ordering;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cds = parse(INPUT);
	println!("sum of complexities 4: {}", sum_complexities(&cds));
	println!("sum of complexities 27: {}", sum_complexities_27(&cds));
}

trait Coord: Copy + Clone + std::fmt::Display + PartialEq + Eq {
	const EMPTY_POS: (i8, i8);
	const A: Self;
	fn coord(&self) -> (i8, i8);

}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum PosNum {
	Digit(u8),
	A,
}

impl Coord for PosNum {
	const EMPTY_POS: (i8, i8) = (0, 3);
	const A :Self = PosNum::A;
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
			PosNum::Digit(8) => (1, 0),
			PosNum::Digit(9) => (2, 0),
			PosNum::Digit(_) => panic!("Invalid digit"),
		}
	}
}

impl std::fmt::Display for PosNum {
	fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			PosNum::A => write!(f, "A"),
			PosNum::Digit(n) => write!(f, "{n}"),
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
	const POSSIBLE :&[Pos] = &[
		Pos::Up,
		Pos::A,
		Pos::Left,
		Pos::Down,
		Pos::Right,
	];
}

impl Coord for Pos {
	const EMPTY_POS :(i8, i8) = (0, 0);
	const A :Self = Pos::A;
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
		if false
			{ print!($($args),*); }
	};
}

fn dcmds(code :&[Pos]) -> String {
	code.iter().map(|s| format!("{s}")).collect::<String>()
}

fn add_transition<T :Coord>(cmds :&mut Vec<Pos>, cd_from :T, cd_to :T) {
	let coord_from = cd_from.coord();
	let coord_to = cd_to.coord();
	dprint!("    {cd_from} TO {cd_to}: {coord_from:?}->{coord_to:?}");
	let (x_cmd, x_cnt) = match coord_from.0.cmp(&coord_to.0) {
		Ordering::Less => {
			(Some(Pos::Right), (coord_to.0 - coord_from.0) as usize)
		}
		Ordering::Equal => {
			(None, 0)
		}
		Ordering::Greater => {
			(Some(Pos::Left), (coord_from.0 - coord_to.0) as usize)
		}
	};
	let (y_cmd, y_cnt) = match coord_from.1.cmp(&coord_to.1) {
		Ordering::Less => {
			(Some(Pos::Down), (coord_to.1 - coord_from.1) as usize)
		}
		Ordering::Equal => (None, 0),
		Ordering::Greater => {
			(Some(Pos::Up), (coord_from.1 - coord_to.1) as usize)
		}
	};
	dprint!("; x_cmd: {x_cmd:?}{x_cnt}; y_cmds: {y_cmd:?}{y_cnt}\n");
	let add = |vec :&mut Vec<_>, v :Option<Pos>, cnt :usize| {
		if let Some(v) = v {
			vec.extend_from_slice(&vec![v; cnt]);
		}
	};
	/*
	Try to go left first, unless it can't be done:
	the < button is very far from the other buttons, and one needs to
	press < to get to it from A. If one does v<A for example, one needs to
	do v<A<A>>^A, and for <vA one needs to do v<<A>A>^A.
	------------------------
	.                      v
	.      v       <       A
	. v <  A   <   A >>  ^ A
	v<A<A>>Av<<A>>^AvAA<^A>A
	------------------------
	------------------------
	.                  v
	.        <   v     A
	. v <<   A > A > ^ A
	<vA<AA>>^AvA^AvA<A>A
	------------------------
	For the right button, we want to put it last: if it's together
	with up, like in ^>A or >^A, 2 keypads down the number for ^>A
	is still the same as >^A, but if you go 3 pads down,
	the sequence ^>A is shorter by 1.
	--------------------
	.     >        ^   A
	.  v  A   <  ^ A > A
	.<vA>^Av<<A>^A>AvA^A
	--------------------
	--------------------
	.       ^      >   A
	.   <   A  v > A ^ A
	.v<<A>>^A<vA>A^A<A>A
	--------------------
	-------------------------------------------------
	.                >                    ^         A
	.        v       A         <      ^   A     >   A
	.   < v  A  >  ^ A  v <<   A >  ^ A > A  v  A ^ A
	.v<<A>A<^Av>A<^A>A<vA<AA>>^AvA<^A>AvA^A<vA>^A<A>A
	-------------------------------------------------
	-------------------------------------------------
	.                 ^                >           A
	.         <       A        v   >   A       ^   A
	.  v <<   A >>  ^ A   < v  A > A ^ A   <   A > A
	.v<A<AA^>>AvAA<^A>Av<<A>A^>AvA^A<A>Av<<A>>^AvA^A
	-------------------------------------------------
	*/
	// Go left first if possible
	if x_cmd == Some(Pos::Left) && (coord_to.0, coord_from.1) != T::EMPTY_POS {
		add(cmds, x_cmd, x_cnt);
		add(cmds, y_cmd, y_cnt);
	}
	// Go right last if possible
	else if x_cmd == Some(Pos::Right) &&  (coord_from.0, coord_to.1) != T::EMPTY_POS {
		add(cmds, y_cmd, y_cnt);
		add(cmds, x_cmd, x_cnt);
	}
	// If neither works, do what's safe.
	else if (coord_to.0, coord_from.1) != T::EMPTY_POS {
		add(cmds, x_cmd, x_cnt);
		add(cmds, y_cmd, y_cnt);
	} else {
		add(cmds, y_cmd, y_cnt);
		add(cmds, x_cmd, x_cnt);
	}
	cmds.push(Pos::A);
}

fn transitions_for<T :Coord>(code :&[T]) -> impl Iterator<Item = (T, T)> + '_ {
	code.iter().scan(T::A, |prev, &cd_to| {
		let prev = std::mem::replace(prev, cd_to);
		Some((prev, cd_to))
	})
}

fn shortest_for<T :Coord>(code :&[T]) -> Vec<Pos> {
	let mut cmds = Vec::new();
	for (cd_from, cd_to) in transitions_for(code) {
		add_transition(&mut cmds, cd_from, cd_to);
	}
	dprint!("robot cmds: {}\n", dcmds(&cmds));
	cmds
}

fn print_codes(code: &[PosNum], code_0 :&[Pos], code_1 :&[Pos], code_2 :&[Pos]) {
	let mut c = code.iter();
	let mut c0 = code_0.iter();
	let mut c1 = code_1.iter();
	let mut c2 = code_2.iter();

	let mut strs = [const {String::new() }; 4];

	while let Some(c2) = c2.next() {
		strs[3] += &format!("{c2}");
		if *c2 == Pos::A {
			let c1 = c1.next().unwrap();
			strs[2] += &format!("{c1}");
			if *c1 == Pos::A {
				let c0 = c0.next().unwrap();
				strs[1] += &format!("{c0}");
				if *c0 == Pos::A {
					let c = c.next().unwrap();
					strs[0] += &format!("{c}");
				} else {
					strs[..1].iter_mut().for_each(|s| *s += " ");
				}
			} else {
				strs[..2].iter_mut().for_each(|s| *s += " ");
			}
		} else {
			strs[..3].iter_mut().for_each(|s| *s += " ");
		}
	}
	for s in strs {
		dprint!("{s}\n");
	}
}

fn shortest_press_seq(code :&[PosNum]) -> Vec<Pos> {
	let code_0 = shortest_for(code);
	let code_1 = shortest_for(&code_0);
	let final_code = shortest_for(&code_1);
	print_codes(code, &code_0, &code_1, &final_code);
	final_code
}

fn shortest_press_seq_len(code :&[PosNum], num_robot_kp :u8) -> u64 {
	let code_0 = shortest_for(code);
	let last_level_id = num_robot_kp - 1;
	let mut hm = HashMap::<((Pos, Pos), u8), u64>::new();
	for l in 0..=last_level_id {
		for cd_from in Pos::POSSIBLE {
			for cd_to in Pos::POSSIBLE {
				let mut cmds = Vec::new();
				add_transition(&mut cmds, *cd_from, *cd_to);
				let cost = transitions_for(&cmds)
					.map(|(cd_from, cd_to)| {
						if l == 0 {
							1
						} else {
							hm[&((cd_from, cd_to), l - 1)]
						}
					})
					.sum();
				hm.insert(((*cd_from, *cd_to), l), cost);
			}
		}
	}
	transitions_for(&code_0).map(|(cd_from, cd_to)| {
		hm[&((cd_from, cd_to), last_level_id)]
	})
	.sum()
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

fn complexity_n(code :&[PosNum], num_robot_kp :u8) -> u64 {
	shortest_press_seq_len(code, num_robot_kp) * code_to_num(code) as u64
}

fn sum_complexities_27(list :&[Vec<PosNum>]) -> u64 {
	list.iter()
		.map(|c| complexity_n(c, 25))
		.sum()
}
