use std::collections::{HashSet, HashMap};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cards = parse(INPUT);
	let score = winner_score(&cards.0, &cards.1);
	println!("Winner score: {score}");
	let score_rec = winner_score_recursive(&cards.0, &cards.1);
	println!("Winner score recursive: {score_rec}");
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

fn winner_recursive(results :&mut HashMap<(Vec<u16>, Vec<u16>), (bool, u16)>, p1 :&[u16], p2 :&[u16]) -> (bool, u16) {
	let p_orig = (p1.to_vec(), p2.to_vec());
	if let Some(res) = results.get(&p_orig) {
		return *res;
	}

	let mut p1 = p1.to_vec();
	let mut p2 = p2.to_vec();
	let mut encountered = HashSet::new();
	while p1.len() != 0 && p2.len() != 0 {
		if encountered.contains(&(p1.clone(), p2.clone())) {
			let res = (true, score(&p1));
			results.insert(p_orig, res);
			return res;
		} else {
			encountered.insert((p1.clone(), p2.clone()));
		}
		let p1_card = p1.remove(0);
		let p2_card = p2.remove(0);
		let p1_won = if p1_card as usize <= p1.len() && p2_card as usize <= p2.len() {
			let p1 = &p1[..p1_card as usize];
			let p2 = &p2[..p2_card as usize];
			winner_recursive(results, p1, p2).0
		} else {
			p1_card > p2_card
		};
		let (winner, cards) = if p1_won {
			(&mut p1, (p1_card, p2_card))
		} else {
			(&mut p2, (p2_card, p1_card))
		};
		winner.push(cards.0);
		winner.push(cards.1);
	}
	let (wb, winner) = if p1.len() == 0 {
		(false, p2)
	} else {
		(true, p1)
	};
	let res = (wb, score(&winner));
	results.insert(p_orig, res);
	res
}

fn winner_score_recursive(p1 :&[u16], p2 :&[u16]) -> u16 {
	let mut results = HashMap::new();
	winner_recursive(&mut results, p1, p2).1
}
