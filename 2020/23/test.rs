use super::*;

#[test]
fn test_1() {
	let nums = parse("389125467");
	assert_eq!(get_labels_after_n(&nums, 10), "92658374");
	assert_eq!(get_labels_after_100(&nums), "67384529");
}

#[test]
fn test_2() {
	let nums = parse("389125467");
	assert_eq!(get_labels_after_ten_million(&nums), 149245887792);
}


#[test]
fn test_get_labels() {
	let cups = Cups {
		cups: vec![5, 8, 3, 7, 4, 1, 9, 2, 6],
		biggest_cup_num: 9,
		current: 0,
	};
	assert_eq!("92658374", cups.get_labels());
}
