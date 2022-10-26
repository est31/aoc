use super::*;

const EXAMPLE_INPUT_1 :&str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

const EXAMPLE_INPUT_2 :&str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT_1);
	let sum = apply_cmds(&cmds);
	assert_eq!(sum, 165);
}

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT_2);
	let sum = apply_cmds_v2_slow(&cmds);
	assert_eq!(sum, 208);
}
