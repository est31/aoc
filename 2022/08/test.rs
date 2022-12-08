use super::*;

const EXAMPLE_INPUT :&str = "\
30373
25512
65332
33549
35390
";

#[test]
fn test_1() {
	let grid = parse(EXAMPLE_INPUT);
	assert_eq!(outside_visible(&grid), 21);
}
