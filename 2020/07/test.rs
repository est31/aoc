use super::*;

const EXAMPLE_INPUT :&str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

#[test]
fn test_1() {
	let lines = parse(EXAMPLE_INPUT);
	let shiny_gold_id = get_shiny_gold_id(&lines.0).unwrap();
	let roots = search_roots(shiny_gold_id, &lines.1);
	assert_eq!(roots, 4);
	let roots = search_children(shiny_gold_id, &lines.1);
	assert_eq!(children, 32);
}

const EXAMPLE_INPUT :&str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

#[test]
fn test_2() {
	let lines = parse(EXAMPLE_INPUT);
	let shiny_gold_id = get_shiny_gold_id(&lines.0).unwrap();
	let roots = search_children(shiny_gold_id, &lines.1);
	assert_eq!(children, 126);
}
