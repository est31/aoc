use super::*;

const EXAMPLE_INPUT_1 :&str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

#[test]
fn test_1() {
	let mem = parse_sized(EXAMPLE_INPUT_1, 7);
	assert_eq!(22, mem.min_steps_after(12).unwrap());
}

#[test]
fn test_2() {
	let mem = parse_sized(EXAMPLE_INPUT_1, 7);
	assert_eq!((6, 1), mem.first_byte_to_block());
}

