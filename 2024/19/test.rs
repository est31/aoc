use super::*;

const EXAMPLE_INPUT_1 :&str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[test]
fn test_1() {
	let twl = parse(EXAMPLE_INPUT_1);
	assert_eq!(6, twl.num_possible());
}

#[test]
fn test_2() {
	let twl = parse(EXAMPLE_INPUT_1);
	assert_eq!(16, twl.sum_possible());
}
