use super::*;


const EXAMPLE_INPUT_1 :&str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[test]
fn test_1() {
	let f = parse(EXAMPLE_INPUT_1);
	assert_eq!(41, positions_visited(&f));
	assert_eq!(6, possible_obstacles_for_loop(&f));
}
