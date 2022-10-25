use super::*;

const EXAMPLE_INPUT :&str = "\
939
7,13,x,x,59,x,31,19
";

#[test]
fn test_1() {
	let p = parse(EXAMPLE_INPUT);
	assert_eq!(939, p.0);
	assert_eq!(&[7, 13, 0, 0, 59, 31, 19], p.1.as_slice());
	let (id, wt) = min_bus_time(p.0, &p.1);
	assert_eq!(59, id);
	assert_eq!(5, wt);
}
