use super::*;

const EXAMPLE_INPUT_1 :&str = "\
1
10
100
2024
";

const EXAMPLE_INPUT_2 :&str = "\
1
2
3
2024
";

#[test]
fn test_1() {
	let nums = parse(EXAMPLE_INPUT_1);
	assert_eq!(secret_num_2000_sum(&nums), 37327623);
}

#[test]
fn test_advance() {
	assert_eq!(advance(123), 15887950);
	assert_eq!(advance(15887950), 16495136);
	assert_eq!(advance(16495136), 527345);
	assert_eq!(advance(527345), 704524);
	assert_eq!(advance(704524), 1553684);
}

#[test]
fn test_2() {
	let nums = parse(EXAMPLE_INPUT_2);
	assert_eq!(bananas_for(&[123], &[-1, -1, 0, 2]), 6);
	assert_eq!(bananas_for(&nums, &[-2, 1, -1, 3]), 23);
	assert_eq!(most_bananas_simple(&nums), 23);
}
