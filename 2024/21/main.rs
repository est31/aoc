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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Pos {
	Up,
	A,
	Left,
	Down,
	Right,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
	pos_numeric :PosNum,
	pos_robot_1 :Pos,
	pos_robot_2 :Pos,
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

fn shortest_press_seq(code :&[PosNum]) -> Vec<Pos> {
	todo!()
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

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

impl State {
}
