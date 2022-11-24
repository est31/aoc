use super::*;

#[test]
fn test_1() {
	let nums = parse("389125467");
	assert_eq!(get_labels_after_n(&nums, 10), "92658374");
	assert_eq!(get_labels_after_100(&nums), "67384529");
}

#[test]
fn test_randoms() {
	// Some random orderings we generated,
	// and validated with the part 1 implementation,
	// to make sure the part 2 one doesn't regress.
	let nums = parse("856173294");
	assert_eq!(get_labels_after_n(&nums, 10), "63857492");
	assert_eq!(get_labels_after_100(&nums), "64823759");
	assert_eq!(get_labels_after_n(&nums, 200), "87526934");
	assert_eq!(get_labels_after_n(&nums, 300), "28345697");
	assert_eq!(get_labels_after_n(&nums, 400), "59863472");
	assert_eq!(get_labels_after_n(&nums, 500), "37468529");
	let nums = parse("798563412");
	assert_eq!(get_labels_after_n(&nums, 10), "24359876");
	assert_eq!(get_labels_after_100(&nums), "76543829");
	let nums = parse("954387261");
	assert_eq!(get_labels_after_n(&nums, 10), "67452389");
	assert_eq!(get_labels_after_100(&nums), "28549376");
	let nums = parse("279851346");
	assert_eq!(get_labels_after_n(&nums, 10), "69437852");
	assert_eq!(get_labels_after_100(&nums), "46297358");
	let nums = parse("396521784");
	assert_eq!(get_labels_after_n(&nums, 10), "75896432");
	assert_eq!(get_labels_after_100(&nums), "75896432");
	let nums = parse("968312745");
	assert_eq!(get_labels_after_n(&nums, 10), "93785642");
	assert_eq!(get_labels_after_100(&nums), "48362597");
	let nums = parse("816423975");
	assert_eq!(get_labels_after_n(&nums, 10), "73852964");
	assert_eq!(get_labels_after_100(&nums), "25369478");
	let nums = parse("186549327");
	assert_eq!(get_labels_after_n(&nums, 10), "29685347");
	assert_eq!(get_labels_after_100(&nums), "53794628");
	let nums = parse("351694278");
	assert_eq!(get_labels_after_n(&nums, 10), "75698324");
	assert_eq!(get_labels_after_100(&nums), "67824953");
	let nums = parse("924581637");
	assert_eq!(get_labels_after_n(&nums, 10), "74358296");
	assert_eq!(get_labels_after_100(&nums), "46892735");
}

#[test]
fn test_2() {
	let nums = parse("389125467");
	let lbls = get_labels_after_ten_million(&nums);
	assert_eq!(lbls, (934001, 159792));
	assert_eq!(lbls.0 as u64 * lbls.1 as u64, 149245887792);
}


#[test]
fn test_get_labels() {
	let cups = Cups::new(vec![5, 8, 3, 7, 4, 1, 9, 2, 6]);
	assert_eq!("92658374", cups.get_labels());
}
