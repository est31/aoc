use super::*;

const EXAMPLE_INPUT :&str = "\
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

#[test]
fn test_1() {
	let (map, cmds) = parse(EXAMPLE_INPUT);
	let p = final_password(&map, &cmds);
	assert_eq!(p, 6032);
}
