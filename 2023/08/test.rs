use super::*;

const EXAMPLE_INPUT_1 :&str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const EXAMPLE_INPUT_2 :&str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[test]
fn test_1() {
	let network = parse(EXAMPLE_INPUT_1);
	println!("{:?}", network.1);
	assert_eq!(steps_required(&network), 2);
}

#[test]
fn test_2() {
	let network = parse(EXAMPLE_INPUT_2);
	println!("{:?}", network.1);
	assert_eq!(steps_required(&network), 6);
}
