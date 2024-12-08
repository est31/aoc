use super::*;


const EXAMPLE_INPUT_1 :&str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[test]
fn test_1() {
	let eqs = parse(EXAMPLE_INPUT_1);
	assert_eq!(3749, total_calibration_res(&eqs));
	assert_eq!(11387, total_calibration_concat(&eqs));
}

#[test]
fn test_concat() {
	assert_eq!(156, concat(15, 6));
	assert_eq!(12345, concat(12, 345));
}
