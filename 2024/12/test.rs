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

#[test]
fn test_1() {
	let grd = parse(EXAMPLE_INPUT_1);
	assert_eq!(140, prices(&grd));
}

#[test]
fn test_2() {
	let grd = parse(EXAMPLE_INPUT_2);
	assert_eq!(772, prices(&grd));
}

#[test]
fn test_3() {
	let grd = parse(EXAMPLE_INPUT_3);
	assert_eq!(1930, prices(&grd));
}
