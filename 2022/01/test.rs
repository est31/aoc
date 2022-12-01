use super::*;

const EXAMPLE_INPUT :&str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

#[test]
fn test_1() {
	let groups = parse(EXAMPLE_INPUT);
	let mgs = max_group_sum(&groups);
	assert_eq!(mgs, 24000);
}
