const EXAMPLE_INPUT :&str = "3,4,3,1,2";

use super::*;

#[test]
fn test() {
	let mut fishes = Lanternfishes::from_str(&EXAMPLE_INPUT);
	fishes.steps(18);
	assert_eq!(fishes.total_count(), 26);
	fishes.steps(80 - 18);
	assert_eq!(fishes.total_count(), 5934);
	fishes.steps(256 - 80);
	assert_eq!(fishes.total_count(), 26984457539);
}
