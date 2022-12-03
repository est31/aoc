use super::*;

const EXAMPLE_INPUT :&str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

#[test]
fn test_1() {
	let lines = parse(EXAMPLE_INPUT);
	assert_eq!(lines[0].0, "vJrwpWtwJgWr");
	assert_eq!(lines[0].1, "hcsFMMfFFhFp");
	assert_eq!(lines[1].0, "jqHRNqRjqzjGDLGL");
	assert_eq!(lines[1].1, "rsFMfFZSrLrFZsSL");
	assert_eq!(lines[2].0, "PmmdzqPrV");
	assert_eq!(lines[2].1, "vPwwTWBwg");

	assert_eq!(priorities_of_dupes(&lines[..1]), 16);
	assert_eq!(priorities_of_dupes(&lines[..2]), 16 + 38);
	assert_eq!(priorities_of_dupes(&lines[..3]), 16 + 38 + 42);

	let prios = priorities_of_dupes(&lines);
	assert_eq!(prios, 157);
}

#[test]
fn test_2() {
	let lines = parse(EXAMPLE_INPUT);
	let g_prios = group_priorities(&lines);
	assert_eq!(g_prios, 70);
}
