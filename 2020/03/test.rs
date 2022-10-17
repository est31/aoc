use super::*;

const EXAMPLE_INPUT :&str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

#[test]
fn test() {
	let scene = parse(EXAMPLE_INPUT);
	assert_eq!(count_trees(&scene), 7);
	assert_eq!(count_trees_product(&scene), 336);
}
