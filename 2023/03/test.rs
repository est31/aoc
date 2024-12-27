use super::*;

const EXAMPLE_INPUT :&str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn test() {
	let pn_gr = pn_gr(EXAMPLE_INPUT);
	assert_eq!(&[35, 467, 592, 598, 617, 633, 664, 755], pn_gr.0.as_slice());
	assert_eq!(4361, pn_gr.0.iter().sum::<u32>());
	assert_eq!(&[16345, 451490], pn_gr.1.as_slice());
	assert_eq!(467835, pn_gr.1.iter().sum::<u32>());
}

const INPUT_EXTRACT :&str = "\
89.......641.......
..............697..
.............&.....
....673.%....../...
";

#[test]
fn test_extract() {
	let pn_gr = pn_gr(INPUT_EXTRACT);
	assert_eq!(&[697], pn_gr.0.as_slice());
}
