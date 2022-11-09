use super::*;

const EXAMPLE_INPUT_1 :&str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

#[test]
fn test_1() {
	let (p1, p2) = parse(EXAMPLE_INPUT_1);
	assert_eq!(winner_score(&p1, &p2), 306);
}

#[test]
fn test_score() {
	assert_eq!(score(&[3, 2, 10, 6, 8, 5, 9, 4, 7, 1]), 306);
}
