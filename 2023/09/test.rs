use super::*;

const EXAMPLE_INPUT :&str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn test() {
	let lines = parse(EXAMPLE_INPUT);
	assert_eq!(extrapolate_lines(&lines, false), 114);
	assert_eq!(extrapolate_lines(&lines, true), 2);
}
