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
	let mut twl = parse(EXAMPLE_INPUT_1);
	let desired = std::mem::take(&mut twl.desired);

	twl.desired = vec![desired[0].clone()];
	assert_eq!(2, twl.sum_possible());

	twl.desired = vec![desired[1].clone()];
	assert_eq!(1, twl.sum_possible());

	twl.desired = vec![desired[2].clone()];
	assert_eq!(4, twl.sum_possible());

	twl.desired = vec![desired[3].clone()];
	assert_eq!(6, twl.sum_possible());

	twl.desired = vec![desired[4].clone()];
	assert_eq!(1, twl.sum_possible());

	twl.desired = vec![desired[5].clone()];
	assert_eq!(2, twl.sum_possible());

	twl.desired = desired;
	assert_eq!(16, twl.sum_possible());
}
