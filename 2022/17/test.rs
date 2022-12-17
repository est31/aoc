use super::*;

const EXAMPLE_INPUT :&str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let h = tower_height(&cmds, LIMIT_1);
	assert_eq!(h, 3068);
}

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let h = tower_height(&cmds, LIMIT_2);
	assert_eq!(h, 1514285714288);
}
