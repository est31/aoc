use super::*;

const EXAMPLE_INPUT_1 :&str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

#[test]
fn test_1() {
	assert_eq!(161, sum_of_muls(EXAMPLE_INPUT_1));
}
