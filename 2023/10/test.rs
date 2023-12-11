use super::*;

const EXAMPLE_INPUT_1 :&str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

const EXAMPLE_INPUT_2 :&str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[test]
fn test_1() {
	let field = parse(EXAMPLE_INPUT_1);
	assert_eq!(farthest(&field), 4);

	let field = parse(EXAMPLE_INPUT_2);
	assert_eq!(farthest(&field), 8);
}
