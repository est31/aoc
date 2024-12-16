use super::*;

const EXAMPLE_INPUT_1 :&str = "\
125 17
";

#[test]
fn test_split() {
	assert_eq!((1, 0), split(10));
	assert_eq!((10, 0), split(1000));
	assert_eq!((12, 34), split(1234));
	assert_eq!((784, 329), split(784329));
}

#[test]
fn test_1() {
	let stn = parse(EXAMPLE_INPUT_1);
	assert_eq!(3, stone_count_n(&stn, 1));
	assert_eq!(22, stone_count_n(&stn, 6));
	assert_eq!(55312, stone_count_25(&stn));
}
