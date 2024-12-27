use super::*;

const EXAMPLE_INPUT_1 :&str = "\
1
10
100
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
