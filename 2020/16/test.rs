use super::*;

const EXAMPLE_INPUT_1 :&str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

#[test]
fn test_1() {
	let ticket = parse(EXAMPLE_INPUT_1);
	let er = scanning_error_rate(&ticket);
	assert_eq!(er, 71);
}
