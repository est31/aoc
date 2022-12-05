use super::*;

const EXAMPLE_INPUT :&str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

#[test]
fn test_1() {
	let (stacks, cmds) = parse(EXAMPLE_INPUT);
	assert_eq!(stacks.len(), 3);
	assert_eq!(cmds.len(), 4);
	assert_eq!(stacks[0][..], ['Z', 'N']);
	assert_eq!(stacks[1][..], ['M', 'C', 'D']);
	assert_eq!(stacks[2][..], ['P']);
	let top = exec_and_top(&stacks, &cmds);
	assert_eq!(top, "CMZ");
	let top_9001 = exec_and_top_9001(&stacks, &cmds);
	assert_eq!(top_9001, "MCD");
}
