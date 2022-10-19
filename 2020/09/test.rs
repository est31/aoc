use super::*;

const EXAMPLE_INPUT :&str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

#[test]
fn test_1() {
	let numbers = parse(EXAMPLE_INPUT);
	let v = first_not_sum(5, &numbers);
	assert_eq!(v, 127);
	let v = contiguous_smallest_largest(v, &numbers);
	assert_eq!(v, (15, 47));
}
