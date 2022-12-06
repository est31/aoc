use super::*;

#[test]
fn test_1() {
	assert_eq!(start_of_packet_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
	assert_eq!(start_of_packet_end("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
	assert_eq!(start_of_packet_end("nppdvjthqldpwncqszvftbrmjlhg"), 6);
}

#[test]
fn test_2() {
	assert_eq!(start_of_msg_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
	assert_eq!(start_of_msg_end("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
	assert_eq!(start_of_msg_end("nppdvjthqldpwncqszvftbrmjlhg"), 23);
}
