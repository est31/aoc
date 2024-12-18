use super::*;

const EXAMPLE_INPUT_1 :&str = "\
AAAA
BBCD
BBCC
EEEC
";

const EXAMPLE_INPUT_2 :&str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

const EXAMPLE_INPUT_3 :&str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const EXAMPLE_INPUT_4 :&str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

const EXAMPLE_INPUT_5 :&str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

#[test]
fn test_1_1() {
	let grd = parse(EXAMPLE_INPUT_1);
	assert_eq!(140, prices(&grd));
}

#[test]
fn test_1_2() {
	let grd = parse(EXAMPLE_INPUT_2);
	assert_eq!(772, prices(&grd));
}

#[test]
fn test_1_3() {
	let grd = parse(EXAMPLE_INPUT_3);
	assert_eq!(1930, prices(&grd));
}

#[test]
fn test_2_1() {
	let grd = parse(EXAMPLE_INPUT_1);
	assert_eq!(80, prices_sides(&grd));
}

#[test]
fn test_2_3() {
	let grd = parse(EXAMPLE_INPUT_3);
	assert_eq!(1206, prices_sides(&grd));
}

#[test]
fn test_2_4() {
	let grd = parse(EXAMPLE_INPUT_4);
	assert_eq!(236, prices_sides(&grd));
}

#[test]
fn test_2_5() {
	let grd = parse(EXAMPLE_INPUT_5);
	assert_eq!(368, prices_sides(&grd));
}
