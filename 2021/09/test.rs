use super::*;

const EXAMPLE_INPUT :&str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

#[test]
fn test() {
	assert_eq!(low_points_risk_sum(EXAMPLE_INPUT), 15);
	let field = parse_field(EXAMPLE_INPUT);
	let basins = find_basin_sizes(&field);
	println!("{:?}", basins);
	assert_eq!(largest_3_basins_product(EXAMPLE_INPUT), 1134);
}
