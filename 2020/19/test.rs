use super::*;

const EXAMPLE_INPUT_1 :&str = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"
";

const EXAMPLE_INPUT_2 :&str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";

#[test]
fn test_1() {
	let (rules, _msgs) = parse_rules_msgs(EXAMPLE_INPUT_1);
	assert!(is_valid(&rules, "aab"));
	assert!(is_valid(&rules, "aba"));
	assert!(!is_valid(&rules, "baa"));
	assert!(!is_valid(&rules, "bab"));
	assert!(!is_valid(&rules, "bbb"));
	assert!(!is_valid(&rules, "aaa"));
}

#[test]
fn test_2() {
	let (rules, msgs) = parse_rules_msgs(EXAMPLE_INPUT_2);
	//assert!(is_valid(&rules, ));
}
