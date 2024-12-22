use super::*;

const EXAMPLE_INPUT_1 :&str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const EXAMPLE_INPUT_2 :&str = "\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
";

const EXAMPLE_INPUT_3 :&str = "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

const EXAMPLE_INPUT_4 :&str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const EXAMPLE_INPUT_5 :&str = "\
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
";

const EXAMPLE_INPUT_6 :&str = "\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
";

const EXAMPLE_INPUT_7 :&str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";


#[test]
fn test_1_1() {
	let cmp = parse(EXAMPLE_INPUT_1);
	assert_eq!("4,6,3,5,6,3,5,2,1,0", cmp.output());
}

#[test]
fn test_1_2() {
	let mut cmp = parse(EXAMPLE_INPUT_2);
	assert!(cmp.output_mut().is_empty());
	assert_eq!(cmp.register_b, 1);
}

#[test]
fn test_1_3() {
	let cmp = parse(EXAMPLE_INPUT_3);
	assert_eq!("0,1,2", cmp.output());
}

#[test]
fn test_1_4() {
	let mut cmp = parse(EXAMPLE_INPUT_4);
	assert_eq!(vec![4,2,5,6,7,7,7,7,3,1,0], cmp.output_mut());
	assert_eq!(cmp.register_a, 0);
}

#[test]
fn test_1_5() {
	let mut cmp = parse(EXAMPLE_INPUT_5);
	assert!(cmp.output_mut().is_empty());
	assert_eq!(cmp.register_b, 26);
}

#[test]
fn test_1_6() {
	let mut cmp = parse(EXAMPLE_INPUT_6);
	assert!(cmp.output_mut().is_empty());
	assert_eq!(cmp.register_b, 44354);
}

#[test]
fn test_2_0() {
	let cmp = parse(EXAMPLE_INPUT_7);
	assert_eq!(117440, cmp.lowest_a_for_quine());
}
