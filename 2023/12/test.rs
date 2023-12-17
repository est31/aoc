use super::*;

const EXAMPLE_INPUT :&str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[test]
fn test_1() {
	let pt_rle = parse(EXAMPLE_INPUT);
	assert_eq!(sum_arrangement_counts(&pt_rle[0..=0]), 1);
	assert_eq!(sum_arrangement_counts(&pt_rle[1..=1]), 4);
	assert_eq!(sum_arrangement_counts(&pt_rle[2..=2]), 1);
	assert_eq!(sum_arrangement_counts(&pt_rle[3..=3]), 1);
	assert_eq!(sum_arrangement_counts(&pt_rle[4..=4]), 4);
	assert_eq!(sum_arrangement_counts(&pt_rle[5..=5]), 10);
	assert_eq!(sum_arrangement_counts(&pt_rle), 21);
}
