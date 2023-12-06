use super::*;

const EXAMPLE_INPUT :&str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test_bf() {
	assert_eq!(number_of_beating_hold_times_bf(7, 9), 4);
	assert_eq!(number_of_beating_hold_times_bf(15, 40), 8);
	assert_eq!(number_of_beating_hold_times_bf(30, 200), 9);
	assert_eq!(number_of_beating_hold_times_bf(71530, 940200), 71503);
}

#[test]
fn test() {
	let times_max_dist = parse(EXAMPLE_INPUT);
	println!("{times_max_dist:?}");
	assert_eq!(number_of_beating_hold_times(7, 9), 4);
	assert_eq!(number_of_beating_hold_times(15, 40), 8);
	assert_eq!(number_of_beating_hold_times(30, 200), 9);
	assert_eq!(numbers_product(&times_max_dist), 288);
	assert_eq!(number_of_beating_hold_times(71530, 940200), 71503);
}
