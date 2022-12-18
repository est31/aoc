use super::*;

const EXAMPLE_INPUT :&str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let a = surface_area(&cmds);
	assert_eq!(a, 64);
}

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT);
	println!("{cmds:?}");
	let a = outside_surface_area(&cmds);
	assert_eq!(a, 58);
}
