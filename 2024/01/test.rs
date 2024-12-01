use super::*;

const EXAMPLE_INPUT_1 :&str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn test_1_and_2() {
	let (left, right) = parse_vecs(EXAMPLE_INPUT_1);
	assert_eq!(11, sum_of_diffs(&left, &right));
	assert_eq!(31, similarity_score(&left, &right));
}
