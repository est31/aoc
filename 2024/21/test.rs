use super::*;

const EXAMPLE_INPUT_1 :&str = "\
029A
980A
179A
456A
379A
";

#[test]
fn test_1() {
	let cds = parse(EXAMPLE_INPUT_1);
	assert_eq!(code_to_num(&cds[0]), 29);
	assert_eq!(code_to_num(&cds[1]), 980);
	assert_eq!(code_to_num(&cds[2]), 179);
	assert_eq!(code_to_num(&cds[3]), 456);
	assert_eq!(code_to_num(&cds[4]), 379);
	assert_eq!(shortest_press_seq(&cds[0]).len(), 68);
	assert_eq!(shortest_press_seq(&cds[1]).len(), 60);
	assert_eq!(shortest_press_seq(&cds[2]).len(), 68);
	assert_eq!(shortest_press_seq(&cds[3]).len(), 64);
	assert_eq!(shortest_press_seq(&cds[4]).len(), 64);
	assert_eq!(sum_complexities(&cds), 126384);
}
