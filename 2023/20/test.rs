use super::*;

const EXAMPLE_INPUT_1 :&str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

const EXAMPLE_INPUT_2 :&str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[test]
fn test_1() {
	let pl = parse(EXAMPLE_INPUT_1);
	println!("{pl:?}");
	assert_eq!(low_high(&pl), (8000, 4000));
	assert_eq!(low_high_product(&pl), 32000000);
}

#[test]
fn test_2() {
	let pl = parse(EXAMPLE_INPUT_2);
	println!("{pl:?}");
	assert_eq!(low_high(&pl), (4250, 2750));
	assert_eq!(low_high_product(&pl), 11687500);
}
