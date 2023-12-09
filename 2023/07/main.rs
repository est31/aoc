use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let hands_bids = parse(INPUT);
	println!("total winnings: {}", total_winnings(&hands_bids, false));
	println!("total winnings with J: {}", total_winnings(&hands_bids, true));
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

fn card_strength(card :u8, joker_low :bool) -> u8 {
	match card {
		b'A' => 0,
		b'K' => 1,
		b'Q' => 2,
		b'J' => if joker_low { 13 } else { 3 },
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

fn hand_strength(hand :&[u8; 5], joker_low :bool) -> [u8; 5] {
	hand.map(|card| card_strength(card, joker_low))
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

fn hand_type(hand :&[u8; 5], joker_mode :bool) -> HandType {
	let mut occurences = HashMap::new();
	for card in hand.iter() {
		*occurences.entry(*card).or_default() += 1;
	}
	let jokers_present = *occurences.get(&b'J').unwrap_or(&0);
	if !joker_mode || jokers_present == 0 {
		return hand_type_inner(hand, &occurences);
	}
	if jokers_present == 5 {
		return HandType::FiveKind;
	}
	fn ht(hand :&[u8; 5], occurences :&mut HashMap<u8, u8>) -> HandType {
		{
			let joker_occ_mut = occurences.get_mut(&b'J').unwrap();
			if *joker_occ_mut == 0 {
				return hand_type_inner(hand, occurences);
			} else {
				*joker_occ_mut -= 1;
			}
		}
		let ret = occurences.iter()
			.filter(|(card, _occ)| **card != b'J')
			.map(|(card, _occ)| {
				let mut occurences = occurences.clone();
				*occurences.get_mut(card).unwrap() += 1;
				ht(hand, &mut occurences)
			})
			.min()
			.unwrap();
		*occurences.get_mut(&b'J').unwrap() += 1;
		ret
	}
	ht(hand, &mut occurences)
}

fn hand_type_inner(hand :&[u8; 5], occurences :&HashMap<u8, u8>) -> HandType {
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

fn total_winnings(hands_bids :&[([u8; 5], u32)], joker_mode :bool) -> u32 {
	let mut hands_bids = hands_bids.iter()
		.map(|(hand, bid)| (hand, bid, hand_type(hand, joker_mode), hand_strength(hand, joker_mode)))
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
