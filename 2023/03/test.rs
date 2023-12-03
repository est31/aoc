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
	let pn = part_numbers(EXAMPLE_INPUT);
	assert_eq!(&[35, 467, 592, 598, 617, 633, 664, 755], pn.as_slice());
	assert_eq!(4361, pn.iter().sum::<u32>());
}