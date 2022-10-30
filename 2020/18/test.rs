use super::*;

#[test]
fn test_1() {
	assert_eq!(eval_lines("1 + (2 * 3) + (4 * (5 + 6))"), 51);
	assert_eq!(eval_lines("2 * 3 + (4 * 5)"), 26);
	assert_eq!(eval_lines("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
	assert_eq!(eval_lines("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
	assert_eq!(eval_lines("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}

#[test]
fn test_2() {
	assert_eq!(eval_lines_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
	assert_eq!(eval_lines_2("2 * 3 + (4 * 5)"), 46);
	assert_eq!(eval_lines_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
	assert_eq!(eval_lines_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
	assert_eq!(eval_lines_2("((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2 + 4) * 2"), 23340);
	assert_eq!(eval_lines_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
}

#[test]
fn test_2_last() {
	assert_eq!(eval_lines_2("(6 * 9) * (15 * (8 + 6))"), 11340);
	assert_eq!(eval_lines_2("(6 * 9) * (210 + 6)"), 11664);
	assert_eq!(eval_lines_2("(6 * 9) * ((15 * 14) + 6)"), 11664);
	assert_eq!(eval_lines_2("(6 * 9) * ((15 * (8 + 6)) + 6)"), 11664);
	assert_eq!(eval_lines_2("(((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6))"), 11664);
	assert_eq!(eval_lines_2("(((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2 + 4"), 11670);
	assert_eq!(eval_lines_2("((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2 + 4) * 2"), 23340);
	assert_eq!(eval_lines_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
}
