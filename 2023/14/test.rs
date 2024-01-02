use super::*;

const EXAMPLE_INPUT :&str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

#[test]
fn test_1() {
	let (round_rocks, cube_rocks, hg) = parse(EXAMPLE_INPUT);
	assert_eq!(total_load_tilted(&round_rocks, &cube_rocks, hg), 136);
}
