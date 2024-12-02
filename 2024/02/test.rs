use super::*;

const EXAMPLE_INPUT_1 :&str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn test_1_and_2() {
	let reports = parse_reports(EXAMPLE_INPUT_1);
	assert_eq!(1, number_of_safe(&[reports[0].clone()]));
	assert_eq!(0, number_of_safe(&[reports[1].clone()]));
	assert_eq!(0, number_of_safe(&[reports[2].clone()]));
	assert_eq!(0, number_of_safe(&[reports[3].clone()]));
	assert_eq!(0, number_of_safe(&[reports[4].clone()]));
	assert_eq!(1, number_of_safe(&[reports[5].clone()]));

	assert_eq!(2, number_of_safe(&reports));
}
