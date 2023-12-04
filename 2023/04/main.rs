use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cards = parse(INPUT);
	let worth = get_worth(&cards);
	println!("worth: {worth}");
	let cards_num = get_cards_num(&cards);
	println!("cards num: {cards_num}");
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

fn get_matches_it(cards :&[(Vec<u8>, Vec<u8>)]) -> impl Iterator<Item = usize> + '_ {
	cards.iter()
		.map(|(winning, present)| {
			let winning_hs = winning.iter()
				.map(|n| *n)
				.collect::<HashSet<_>>();
			let matches = present.iter()
				.filter(|n| winning_hs.contains(n))
				.count();
			matches
		})
}

fn get_worth(cards :&[(Vec<u8>, Vec<u8>)]) -> u32 {
	get_matches_it(cards)
		.map(|matches| {
			if matches == 0 {
				0
			} else {
				1 << (matches - 1)
			}
		})
		.sum()
}

fn get_cards_num(cards :&[(Vec<u8>, Vec<u8>)]) -> u32 {
	get_mult_cards_num(cards).1
}

fn get_mult_cards_num(cards :&[(Vec<u8>, Vec<u8>)]) -> (Vec<u32>, u32) {
	let mut multipliers = vec![1; cards.len()];
	let mut sum = 0;
	for (i, matches) in get_matches_it(cards).enumerate() {
		let mult = multipliers[i];
		sum += mult;
		if i < cards.len() - 1 {
			let high = cards.len().min(i + matches + 1);
			for v in multipliers[(i + 1)..high].iter_mut() {
				*v += mult;
			}
		}
	}
	(multipliers, sum)
}
