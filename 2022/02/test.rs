use super::*;

const EXAMPLE_INPUT :&str = "\
A Y
B X
C Z
";

#[test]
fn test_1() {
	let moves = parse(EXAMPLE_INPUT);
	let tsc = total_score(&moves);
	assert_eq!(tsc, 15);
	let tsc2 = total_score_2(&moves);
	assert_eq!(tsc2, 12);
}
