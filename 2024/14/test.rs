use super::*;

const EXAMPLE_INPUT_1 :&str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[test]
fn test_1() {
	let pvs = parse(EXAMPLE_INPUT_1);
	assert_eq!(12, safety_factor_100_wh(&pvs, 11, 7));
}
