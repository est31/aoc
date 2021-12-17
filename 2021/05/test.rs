const EXAMPLE_INPUT :&str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

use super::*;

#[test]
fn test() {
	assert_eq!(find_crossings(&EXAMPLE_INPUT, true), 5);
	assert_eq!(find_crossings(&EXAMPLE_INPUT, false), 12);
}