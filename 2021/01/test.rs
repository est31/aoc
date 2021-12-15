const EXAMPLE_INPUT :&str = "\
199
200
208
210
200
207
240
269
260
263";

use super::*;

#[test]
fn test() -> Result<()> {
	assert_eq!(count_increases(EXAMPLE_INPUT)?, 7);
	assert_eq!(count_increases_conv(EXAMPLE_INPUT)?, 5);
	Ok(())
}
