use super::*;

const EXAMPLE_INPUT :&str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

#[test]
fn test_1() {
	let components = parse(EXAMPLE_INPUT);
	assert_eq!(sum_hashes(&components), 1320);
	assert_eq!(focusing_power(&components), 145);
}
