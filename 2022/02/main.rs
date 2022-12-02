const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let moves = parse(INPUT);
	let tsc = total_score(&moves);
	println!("total score: {tsc}");
	let tsc2 = total_score_2(&moves);
	println!("total score 2: {tsc2}");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Shape {
	Rock,
	Paper,
	Scissors,
}

impl Shape {
	fn from_str(s :&str) -> Self {
		match s {
			"A" | "X" => Shape::Rock,
			"B" | "Y" => Shape::Paper,
			"C" | "Z" => Shape::Scissors,
			_ => panic!("Invalid move: '{s}'"),
		}
	}
	fn score(&self) -> u16 {
		match *self {
			Shape::Rock => 1,
			Shape::Paper => 2,
			Shape::Scissors => 3,
		}
	}
}

fn parse(input :&str) -> Vec<(Shape, &str)> {
	input.lines()
		.map(|l| {
			let mut words = l.split_whitespace();
			let first = words.next().unwrap();
			let second = words.next().unwrap();
			(Shape::from_str(first), second)
		})
		.collect::<Vec<_>>()
}

fn total_score(moves :&[(Shape, &str)]) -> u16 {
	moves.iter()
		.map(|(f, s)| {
			let s = Shape::from_str(s);
			let won = match (f, s) {
				(Shape::Rock, Shape::Paper) => Some(true),
				(Shape::Rock, Shape::Scissors) => Some(false),
				(Shape::Paper, Shape::Rock) => Some(false),
				(Shape::Paper, Shape::Scissors) => Some(true),
				(Shape::Scissors, Shape::Rock) => Some(true),
				(Shape::Scissors, Shape::Paper) => Some(false),
				// Both are the same: draw
				_ => None,
			};
			let won_score = match won {
				Some(true) => 6,
				Some(false) => 0,
				None => 3,
			};
			s.score() + won_score
		})
		.sum()
}

fn total_score_2(moves :&[(Shape, &str)]) -> u16 {
	moves.iter()
		.map(|(f, s)| {
			let (won_score, must_win) = match *s {
				"X" => (0, false),
				"Y" => return f.score() + 3,
				"Z" => (6, true),
				_ => panic!("invalid word: '{s}'"),
			};
			let chosen = match (f, must_win) {
				(Shape::Rock, false) => Shape::Scissors,
				(Shape::Rock, true) => Shape::Paper,
				(Shape::Paper, false) => Shape::Rock,
				(Shape::Paper, true) => Shape::Scissors,
				(Shape::Scissors, false) => Shape::Paper,
				(Shape::Scissors, true) => Shape::Rock,
			};
			chosen.score() + won_score
		})
		.sum()
}
