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

const EXAMPLE_INPUT_1_X_H :&str = "\
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const EXAMPLE_INPUT_ONLY_RIGHT_ANGLE :&str = "\
S.S.SS
.A.AA.
M.MMMM
";

const EXAMPLE_INPUT_2 :&str = "\
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
";

const EXAMPLE_INPUT_2_H :&str = "\
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
";


const EXAMPLE_INPUT_2_X :&str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

const EXAMPLE_INPUT_3_1 :&str = "\
XMAS
....
....
....
";

const EXAMPLE_INPUT_3_2 :&str = "\
X...
M...
A...
S...
";

const EXAMPLE_INPUT_3_3 :&str = "\
SAMX
....
....
....
";

const EXAMPLE_INPUT_3_4 :&str = "\
S...
A...
M...
X...
";

const EXAMPLE_INPUT_3_5 :&str = "\
X...
.M..
..A.
...S
";

const EXAMPLE_INPUT_3_6 :&str = "\
S...
.A..
..M.
...X
";

const EXAMPLE_INPUT_3_7 :&str = "\
...S
..A.
.M..
X...
";

const EXAMPLE_INPUT_3_8 :&str = "\
...X
..M.
.A..
S...
";

#[test]
fn test_0() {
	assert_eq!(4, count_xmas(EXAMPLE_INPUT_0));
}


#[test]
fn test_1() {
	assert_eq!(18, count_xmas(EXAMPLE_INPUT_1));
	assert_eq!(18, count_xmas(EXAMPLE_INPUT_2));
	assert_eq!(8, count_xmas(EXAMPLE_INPUT_2_H));
}

#[test]
fn test_1_3() {
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_1));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_2));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_3));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_4));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_5));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_6));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_7));
	assert_eq!(1, count_xmas(EXAMPLE_INPUT_3_8));
}

#[test]
fn test_2() {
	assert_eq!(0, count_x_mas(EXAMPLE_INPUT_0));
	assert_eq!(2, count_x_mas(EXAMPLE_INPUT_ONLY_RIGHT_ANGLE));
	assert_eq!(9, count_x_mas(EXAMPLE_INPUT_2_X));
	assert_eq!(4, count_x_mas(EXAMPLE_INPUT_1_X_H));
	assert_eq!(9, count_x_mas(EXAMPLE_INPUT_1));
}
