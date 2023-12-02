use super::*;

const EXAMPLE_INPUT :&str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[test]
fn test_1() {
	let values = parse(EXAMPLE_INPUT);
	assert_eq!(142, sum(&values));
}
