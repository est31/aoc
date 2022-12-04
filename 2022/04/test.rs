use super::*;

const EXAMPLE_INPUT :&str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

#[test]
fn test_1() {
	let lines = parse(EXAMPLE_INPUT);
	let cont = contained_pairs(&lines);
	assert_eq!(cont, 2);
	let ovlp = overlap_pairs(&lines);
	assert_eq!(ovlp, 4);
}
