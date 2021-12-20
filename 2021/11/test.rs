use super::*;

const EXAMPLE_INPUT :&str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

#[test]
fn test() {
	let mut octopuses = parse_octopuses(EXAMPLE_INPUT);
	assert_eq!(octopuses_steps(&mut octopuses, 10), 204);
	assert_eq!(octopuses_steps(&mut octopuses, 100 - 10), 1656 - 204);
}

#[test]
fn test_total_blink() {
	let mut octopuses = parse_octopuses(EXAMPLE_INPUT);
	assert_eq!(octopuses_first_total_blink(&mut octopuses), 195);
}
