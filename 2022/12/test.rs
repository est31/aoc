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
	let fld = parse(EXAMPLE_INPUT);
	let steps = steps_to_goal(fld);
	assert_eq!(steps, 31);
}
