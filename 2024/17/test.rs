use super::*;

const EXAMPLE_INPUT_1 :&str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

#[test]
fn test_1_1() {
	let cmp = parse(EXAMPLE_INPUT_1);
	assert_eq!("4,6,3,5,6,3,5,2,1,0", cmp.output());
}
