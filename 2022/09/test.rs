use super::*;

const EXAMPLE_INPUT_1 :&str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT_1);
	assert_eq!(visited_positions(&cmds, 1), 13);
}

const EXAMPLE_INPUT_2 :&str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT_1);
	assert_eq!(visited_positions(&cmds, 9), 1);
	let cmds = parse(EXAMPLE_INPUT_2);
	assert_eq!(visited_positions(&cmds, 9), 36);
}
