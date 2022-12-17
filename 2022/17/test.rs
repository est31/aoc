use super::*;

const EXAMPLE_INPUT :&str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let h = tower_height(&cmds);
	assert_eq!(h, 3068);
}
