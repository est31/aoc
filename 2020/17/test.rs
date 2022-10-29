use super::*;

const EXAMPLE_INPUT_1 :&str = "\
.#.
..#
###
";

#[test]
fn test_1() {
	let sc = parse_slice(EXAMPLE_INPUT_1);
	print(&sc);
	let sc_after = six_steps(&sc);
	assert_eq!(sc_after.len(), 112);
}
