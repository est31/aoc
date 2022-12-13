use super::*;

const EXAMPLE_INPUT :&str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[test]
fn test_1() {
	let lists = parse(EXAMPLE_INPUT);
	assert_eq!(well_ordered_sum(&lists[0..][..1]), 1);
	assert_eq!(well_ordered_sum(&lists[1..][..1]), 1);
	assert_eq!(well_ordered_sum(&lists[2..][..1]), 0);
	assert_eq!(well_ordered_sum(&lists[3..][..1]), 1);
	assert_eq!(well_ordered_sum(&lists[4..][..1]), 0);
	assert_eq!(well_ordered_sum(&lists[5..][..1]), 1);
	assert_eq!(well_ordered_sum(&lists[6..][..1]), 0);
	assert_eq!(well_ordered_sum(&lists[7..][..1]), 0);
	let sum = well_ordered_sum(&lists);
	assert_eq!(sum, 13);
}

#[test]
fn test_2() {
	let lists = parse(EXAMPLE_INPUT);
	let dk = decoder_key(&lists);
	assert_eq!(dk, 140);
}
