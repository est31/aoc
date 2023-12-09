use super::*;

const EXAMPLE_INPUT :&str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn test() {
	let hands_bids = parse(EXAMPLE_INPUT);
	println!("{hands_bids:?}");
	assert_eq!(total_winnings(&hands_bids), 6440);
}
