use super::*;

const EXAMPLE_INPUT :&str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let cap = sand_holding_cap(&cmds, false);
	assert_eq!(cap, 24);
}

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let cap_floor = sand_holding_cap(&cmds, true);
	assert_eq!(cap_floor, 93);
}
