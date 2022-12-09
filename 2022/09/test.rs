use super::*;

const EXAMPLE_INPUT :&str = "\
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
	let cmds = parse(EXAMPLE_INPUT);
	assert_eq!(visited_positions(&cmds), 13);
}
