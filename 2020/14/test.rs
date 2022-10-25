use super::*;

const EXAMPLE_INPUT :&str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT);
	let sum = apply_cmds(&cmds);
	assert_eq!(sum, 165);
}
