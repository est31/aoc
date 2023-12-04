use super::*;

const EXAMPLE_INPUT :&str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn test_1() {
	let cards = parse(EXAMPLE_INPUT);
	assert_eq!(get_worth(&cards[0..=0]), 8);
	assert_eq!(get_worth(&cards[1..=1]), 2);
	assert_eq!(get_worth(&cards[2..=2]), 2);
	assert_eq!(get_worth(&cards[3..=3]), 1);
	assert_eq!(get_worth(&cards[4..=4]), 0);
	assert_eq!(get_worth(&cards[5..=5]), 0);
	let worth = get_worth(&cards);
	assert_eq!(worth, 13);
}

#[test]
fn test_2() {
	let cards = parse(EXAMPLE_INPUT);
	assert_eq!(&get_mult_cards_num(&cards).0, &[1, 2, 4, 8, 14, 1]);
	assert_eq!(get_cards_num(&cards), 30);
}
