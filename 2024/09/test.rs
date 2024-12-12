use super::*;


const EXAMPLE_INPUT_1 :&str = "\
2333133121414131402\
";

#[test]
fn test_1() {
	let eqs = parse(EXAMPLE_INPUT_1);
	assert_eq!(1928, defrag_checksum(&eqs));
}
