use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let hands_bids = parse(INPUT);
	println!("total winnings: {}", total_winnings(&hands_bids));
}

fn parse(input :&str) -> Vec<([u8; 5], u32)> {
	input.lines()
		.map(|l| {
			let l = l.trim();
			let mut it = l.split_whitespace();
			let (c0, c1) = (it.next().unwrap(), it.next().unwrap());
			let hand = c0.chars()
				.map(|ch| ch as u8)
				.collect::<Vec<_>>();
			let hand :[u8; 5] = hand.try_into().unwrap();
			let bid = u32::from_str(c1).unwrap();
			(hand, bid)
		})
		.collect::<Vec<_>>()
}

fn card_strength(card :u8) -> u8 {
	match card {
		b'A' => 0,
		b'K' => 1,
		b'Q' => 2,
		b'J' => 3,
		b'T' => 4,
		b'9' => 5,
		b'8' => 6,
		b'7' => 7,
		b'6' => 8,
		b'5' => 9,
		b'4' => 10,
		b'3' => 11,
		b'2' => 12,
		_ => panic!("invalid card {card}"),
	}
}

fn hand_strength(hand :&[u8; 5]) -> [u8; 5] {
	hand.map(card_strength)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandType {
	FiveKind,
	FourKind,
	FullHouse,
	ThreeKind,
	TwoPair,
	OnePair,
	HighCard,
}

fn hand_type(hand :&[u8; 5]) -> HandType {
	let mut occurences = HashMap::new();
	for card in hand.iter() {
		*occurences.entry(*card).or_default() += 1;
	}
	let mut occurences = occurences.iter()
		.map(|(_card, occ)| occ)
		.collect::<Vec<_>>();
	occurences.sort();
	let mut it = occurences.iter().rev();
	let highest = it.next().unwrap();
	match highest {
		5 => HandType::FiveKind,
		4 => HandType::FourKind,
		_ => match (highest, it.next().unwrap()) {
			(3, 2) => HandType::FullHouse,
			(3, 1) => HandType::ThreeKind,
			(2, 2) => HandType::TwoPair,
			(2, 1) => HandType::OnePair,
			(1, 1) => HandType::HighCard,
			h => panic!("invalid combination for hand {hand:?}: {h:?}"),
		},
	}
}

fn total_winnings(hands_bids :&[([u8; 5], u32)]) -> u32 {
	let mut hands_bids = hands_bids.iter()
		.map(|(hand, bid)| (hand, bid, hand_type(hand), hand_strength(hand)))
		.collect::<Vec<_>>();

	hands_bids.sort_by_key(|(_h, _b, ty, strength)| (*ty, *strength));

	/*for hb in hands_bids.iter() {
		println!("{hb:?}");
	}*/

	let hbl = hands_bids.len() as u32;

	hands_bids.iter()
		.enumerate()
		.map(|(rank, (_h, b, _ty, _strength))| (hbl - rank as u32) * **b as u32)
		.sum()
}
