use super::*;

#[test]
fn test_1() {
	assert_eq!(eval_line("2 * 3 + (4 * 5)"), 26);
	assert_eq!(eval_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
	assert_eq!(eval_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
	assert_eq!(eval_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}
