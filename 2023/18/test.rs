use super::*;

const EXAMPLE_INPUT :&str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

#[test]
fn test_1() {
	let dig_plan = parse(EXAMPLE_INPUT);
	println!("{dig_plan:?}");
	assert_eq!(lava_cubes(&dig_plan), 62);
}

const EXAMPLE_INPUT_2 :&str = "\
R 3 (#70c710)
D 4 (#0dc571)
L 3 (#5713f0)
U 4 (#d2c081)
";

#[test]
fn test_2() {
	let dig_plan = parse(EXAMPLE_INPUT_2);
	println!("{dig_plan:?}");
	assert_eq!(lava_cubes(&dig_plan), 4 * 5);
}
