const EXAMPLE_INPUT :&str = "16,1,2,0,4,2,7,1,2,14";

use super::*;

#[test]
fn test_l1() {
	let positions = parse_positions(&EXAMPLE_INPUT);
	assert_eq!(cost_for_center_l1(&positions, 2), 37);
	assert_eq!(cost_for_center_l1(&positions, 1), 41);
	assert_eq!(cost_for_center_l1(&positions, 3), 39);
	assert_eq!(cost_for_center_l1(&positions, 10), 71);
	let center = find_center_l1(&positions);
	println!("cost at center {}: {}", center, cost_for_center_l1(&positions, center));
	assert_eq!(center, 2);
}

#[test]
fn test_l2() {
	let positions = parse_positions(&EXAMPLE_INPUT);
	assert_eq!(cost_for_center_l2(&positions, 5), 168);
	assert_eq!(cost_for_center_l2(&positions, 2), 206);
	let center = find_center_l2(&positions);
	println!("cost at center {}: {}", center, cost_for_center_l2(&positions, center));
	assert_eq!(center, 5);
}
