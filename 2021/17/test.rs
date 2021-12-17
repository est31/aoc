use super::*;

const EXAMPLE_INPUT :&str = "target area: x=20..30, y=-10..-5";

#[test]
fn test() -> Result<()> {
	let target = parse_target_area(EXAMPLE_INPUT)?;
	assert_eq!(find_highest_possible_y(&target), Some(45));
	assert_eq!(find_number_of_successes(&target), 112);
	Ok(())
}
