const EXAMPLE_INPUT :&str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

use super::*;

#[test]
fn test() -> Result<()> {
	assert_eq!(final_position(EXAMPLE_INPUT)?, (15, 10));
	assert_eq!(final_position_ext(EXAMPLE_INPUT)?, (15, 60));
	Ok(())
}
