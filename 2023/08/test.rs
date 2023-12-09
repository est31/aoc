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

const EXAMPLE_INPUT_3 :&str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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

#[test]
fn test_3() {
	let network = parse(EXAMPLE_INPUT_3);
	println!("{:?}", network.1);
	assert_eq!(steps_required_ghosts(&network), 6);
}
