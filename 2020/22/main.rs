use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cards = parse(INPUT);
	let score = winner_score(&cards.0, &cards.1);
	println!("Winner score: {score}");
}

fn parse(input :&str) -> (Vec<u16>, Vec<u16>) {
	let mut lines = input.lines();

	lines.next();

	let mut first = Vec::new();
	while let Some(l) = lines.next() {
		if l.is_empty() {
			assert_eq!(Some("Player 2:"), lines.next());
			break;
		}
		let n = u16::from_str(l).unwrap();
		first.push(n);
	}

	let mut second = Vec::new();
	while let Some(l) = lines.next() {
		let n = u16::from_str(l).unwrap();
		second.push(n);
	}
	(first, second)
}

fn score(p :&[u16]) -> u16 {
	p.iter()
		.rev()
		.enumerate()
		.map(|(i, v)| (i as u16 + 1) * v)
		.sum()
}

fn winner_score(p1 :&[u16], p2 :&[u16]) -> u16 {
	let mut p1 = p1.to_vec();
	let mut p2 = p2.to_vec();
	while p1.len() != 0 && p2.len() != 0 {
		let p1_card = p1.remove(0);
		let p2_card = p2.remove(0);
		if p1_card > p2_card {
			p1.push(p1_card);
			p1.push(p2_card);
		} else {
			p2.push(p2_card);
			p2.push(p1_card);
		}
	}
	let winner = if p1.len() == 0 {
		p2
	} else {
		p1
	};
	score(&winner)
}
