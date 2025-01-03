use super::*;


const EXAMPLE_INPUT_1 :&str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[test]
fn test_1() {
	let ru = parse(EXAMPLE_INPUT_1);
	assert_eq!(143, updates_sum(&ru));
}

#[test]
fn test_2() {
	let mut ru = parse(EXAMPLE_INPUT_1);
	assert_eq!(123, updates_sum_ordered(&mut ru));
}
