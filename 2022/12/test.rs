use super::*;

const EXAMPLE_INPUT :&str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

#[test]
fn test_1() {
	let (field, start, end) = parse(EXAMPLE_INPUT);
	let steps = steps_to_goal(&field, Some(start), end);
	assert_eq!(steps, 31);
	let steps_any = steps_to_goal(&field, None, end);
	assert_eq!(steps_any, 29);
}
