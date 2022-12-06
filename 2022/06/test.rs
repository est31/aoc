use super::*;

#[test]
fn test_1() {
	assert_eq!(start_of_packed_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
	assert_eq!(start_of_packed_end("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
	assert_eq!(start_of_packed_end("nppdvjthqldpwncqszvftbrmjlhg"), 6);
}
