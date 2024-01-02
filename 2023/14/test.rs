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

const EXAMPLE_INPUT_CYCLE_1 :&str = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";


const EXAMPLE_INPUT_CYCLE_2 :&str = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";

#[test]
fn test_1() {
	let (round_rocks, cube_rocks, h_w) = parse(EXAMPLE_INPUT);
	assert_eq!(total_load_tilted(&round_rocks, &cube_rocks, h_w.0), 136);
	assert_eq!(total_load_circles(&round_rocks, &cube_rocks, h_w), 64);
}

#[test]
fn test_1_2() {
	let (round_rocks, cube_rocks, h_w) = parse(EXAMPLE_INPUT);
	let mut round_rocks = round_rocks.clone();

	advance_n(&mut round_rocks, &cube_rocks, h_w, 1);
	let (round_rocks_ccl, _cube_rocks, _h_w) = parse(EXAMPLE_INPUT_CYCLE_1);
	assert_eq!(round_rocks_ccl, round_rocks);

	advance_n(&mut round_rocks, &cube_rocks, h_w, 1);
	let (round_rocks_ccl, _cube_rocks, _h_w) = parse(EXAMPLE_INPUT_CYCLE_2);
	assert_eq!(round_rocks_ccl, round_rocks);
}
