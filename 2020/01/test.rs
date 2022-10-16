use super::*;

const EXAMPLE_INPUT :&str = "\
1721
979
366
299
675
1456
";

#[test]
fn test_1() {
	let lines = parse(EXAMPLE_INPUT);
	let (a, b) = two_sum(&lines);
	assert_eq!(a * b, 514579);
	let (a, b, c) = three_sum(&lines);
	assert_eq!(a * b * c, 241861950);
}
