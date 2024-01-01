use super::*;

const EXAMPLE_INPUT :&str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

#[test]
fn test_1() {
	let patterns = parse(EXAMPLE_INPUT);
	assert_eq!(summarize_notes(&patterns), 405);
	assert_eq!(summarize_notes_smudge(&patterns), 400);
}
