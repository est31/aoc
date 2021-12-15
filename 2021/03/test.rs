const EXAMPLE_INPUT :&str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

use super::*;

#[test]
fn test() -> Result<()> {
	assert_eq!(gamma_espilon(EXAMPLE_INPUT)?, (22, 9));
	assert_eq!(ox_co2(EXAMPLE_INPUT)?, (23, 10));
	Ok(())
}
