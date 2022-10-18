use super::*;

const EXAMPLE_INPUT :&str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

#[test]
fn test_1() {
	let cnts = group_counts(EXAMPLE_INPUT)
		.map(|(cnt, _ey)| cnt)
		.collect::<Vec<_>>();
	assert_eq!(cnts, [3, 3, 3, 1, 1]);
	assert_eq!(sum_of_counts(EXAMPLE_INPUT), 11);
}

#[test]
fn test_2() {
	let eys = group_counts(EXAMPLE_INPUT)
		.map(|(_cnt, ey)| ey)
		.collect::<Vec<_>>();
	assert_eq!(eys, [3, 0, 1, 1, 1]);
	assert_eq!(sum_of_everyone_yes(EXAMPLE_INPUT), 6);
}
