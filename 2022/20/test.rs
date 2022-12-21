use super::*;

const EXAMPLE_INPUT :&str = "\
1
2
-3
3
-2
0
4
";

#[test]
fn test_1() {
	let nums = parse(EXAMPLE_INPUT);
	assert_eq!(nums, &[1, 2, -3, 3, -2, 0, 4]);

	assert_eq!(mix_n(&nums, 1), &[0, 4, 2, 1, -3, 3, -2]);
	assert_eq!(mix_n(&nums, 2), &[0, 4, 1, -3, 2, 3, -2]);
	assert_eq!(mix_n(&nums, 3), &[0, 4, 1, 2, 3, -2, -3]);
	assert_eq!(mix_n(&nums, 4), &[0, 3, 4, 1, 2, -2, -3]);
	assert_eq!(mix_n(&nums, 5), &[0, 3, 4, -2, 1, 2, -3]);
	assert_eq!(mix_n(&nums, 6), &[0, 3, 4, -2, 1, 2, -3]);
	assert_eq!(mix_n(&nums, 7), &[0, 3, -2, 1, 2, -3, 4]);

	assert_eq!(mix(&nums), &[0, 3, -2, 1, 2, -3, 4]);

	let s = grove_coords_sum(&nums);
	assert_eq!(s, 3);
}

#[test]
fn test_2() {
	let nums = parse(EXAMPLE_INPUT);
	let dec = mul_by_dec_key(&nums);
	assert_eq!(dec, &[811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612]);

	assert_eq!(
		mix_n(&dec, dec.len()),
		&[0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153],
	);
	assert_eq!(
		mix_n(&dec, dec.len() * 2),
		&[0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153],
	);
	// ...
	assert_eq!(
		mix_n(&dec, dec.len() * 10),
		&[0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153],
	);

	let s = grove_coords_sum_dec(&nums);
	assert_eq!(s, 1623178306);
}
