use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cards = parse(INPUT);
	let worth = get_worth(&cards);
	println!("worth: {worth}");
}

fn parse(input :&str) -> Vec<(Vec<u8>, Vec<u8>)> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut cmps = l.split([':', '|']);
			_ = cmps.next();
			let first = cmps.next().unwrap().trim();
			let second = cmps.next().unwrap().trim();
			let first_ints = first.split_ascii_whitespace()
				.map(|c| u8::from_str(c).unwrap())
				.collect::<Vec<_>>();
			let second_ints = second.split_ascii_whitespace()
				.map(|c| u8::from_str(c).unwrap())
				.collect::<Vec<_>>();
			(first_ints, second_ints)
		})
		.collect::<Vec<_>>()
}

fn get_worth(cards :&[(Vec<u8>, Vec<u8>)]) -> u32 {
	cards.iter()
		.map(|(winning, present)| {
			let winning_hs = winning.iter()
				.map(|n| *n)
				.collect::<HashSet<_>>();
			let sh = present.iter()
				.filter(|n| winning_hs.contains(n))
				.count();
			if sh == 0 {
				0
			} else {
				1 << (sh - 1)
			}
		})
		.sum()
}
