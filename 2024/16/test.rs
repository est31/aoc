use super::*;

const EXAMPLE_INPUT_1 :&str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const EXAMPLE_INPUT_2 :&str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[test]
fn test_1_1() {
	let mp = parse(EXAMPLE_INPUT_1);
	assert_eq!(7036, mp.lowest_score());
}

#[test]
fn test_1_2() {
	let mp = parse(EXAMPLE_INPUT_2);
	assert_eq!(11048, mp.lowest_score());
}

#[test]
fn test_2_1() {
	let mp = parse(EXAMPLE_INPUT_1);
	assert_eq!(45, mp.tiles_shortest_count());
}

#[test]
fn test_2_2() {
	let mp = parse(EXAMPLE_INPUT_2);
	assert_eq!(64, mp.tiles_shortest_count());
}
