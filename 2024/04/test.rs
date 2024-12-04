use super::*;


const EXAMPLE_INPUT_0 :&str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....
";

const EXAMPLE_INPUT_1 :&str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn test_0() {
	assert_eq!(4, count_xmas(EXAMPLE_INPUT_0));
}


#[test]
fn test_1() {
	assert_eq!(18, count_xmas(EXAMPLE_INPUT_1));
}
