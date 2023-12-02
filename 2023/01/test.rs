use super::*;

const EXAMPLE_INPUT_1 :&str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";


const EXAMPLE_INPUT_2 :&str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

#[test]
fn test_1() {
	let values = parse_1(EXAMPLE_INPUT_1);
	assert_eq!(142, sum(&values));
}

#[test]
fn test_2() {
	let values = parse_2(EXAMPLE_INPUT_2);
	assert_eq!([29, 83, 13, 24, 42, 14, 76], values.as_slice());
	assert_eq!(281, sum(&values));
}
