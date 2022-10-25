use super::*;

const EXAMPLE_INPUT :&str = "\
939
7,13,x,x,59,x,31,19
";

#[test]
fn test_1() {
	let p = parse(EXAMPLE_INPUT);
	assert_eq!(939, p.0);
	assert_eq!(&[7, 13, 0, 0, 59, 0, 31, 19], p.1.as_slice());
	let (id, wt) = min_bus_time(p.0, &p.1);
	assert_eq!(59, id);
	assert_eq!(5, wt);
}

#[test]
fn test_2() {
	let p = parse(EXAMPLE_INPUT);
	assert_eq!(1068781, min_bus_cascade(&p.1));
	let p = parse("0\n17,x,13,19");
	assert_eq!(3417, min_bus_cascade(&p.1));
	let p = parse("0\n67,7,59,61");
	assert_eq!(754018, min_bus_cascade(&p.1));
	let p = parse("0\n67,x,7,59,61");
	assert_eq!(779210, min_bus_cascade(&p.1));
	let p = parse("0\n67,7,x,59,61");
	assert_eq!(1261476, min_bus_cascade(&p.1));
	let p = parse("0\n1789,37,47,1889");
	assert_eq!(1202161486, min_bus_cascade(&p.1));
}
