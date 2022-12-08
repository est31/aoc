use super::*;

const EXAMPLE_INPUT :&str = "\
30373
25512
65332
33549
35390
";

#[test]
fn test_1() {
	let grid = parse(EXAMPLE_INPUT);
	assert_eq!(outside_visible(&grid), 21);
}

#[test]
fn test_2() {
	let grid = parse(EXAMPLE_INPUT);
	assert_eq!(viewing_dist_asc(grid[0].iter().copied()), [0, 1, 2, 3, 1]);
	assert_eq!(viewing_dist_asc(grid[1].iter().copied()), [0, 1, 1, 1, 2]);
	assert_eq!(viewing_dist_asc(grid[2].iter().copied()), [0, 1, 1, 1, 1]);
	assert_eq!(viewing_dist_asc(grid[3].iter().copied()), [0, 1, 2, 1, 4]);
	assert_eq!(scenic_score_maximum(&grid), 8);
}
