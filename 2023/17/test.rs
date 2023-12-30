use super::*;

const EXAMPLE_INPUT_1 :&str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

const EXAMPLE_INPUT_2 :&str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";

#[test]
fn test_1() {
	let field = parse(EXAMPLE_INPUT_1);
	assert_eq!(heat_loss(&field), 102);
	assert_eq!(heat_loss_ultra(&field), 94);
}

#[test]
fn test_2() {
	let field = parse(EXAMPLE_INPUT_2);
	assert_eq!(heat_loss_ultra(&field), 71);
}
