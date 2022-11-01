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
	assert!(is_valid(&rules, &msgs[0]));
	assert!(!is_valid(&rules, &msgs[1]));
	assert!(is_valid(&rules, &msgs[2]));
	assert!(!is_valid(&rules, &msgs[3]));
	assert!(!is_valid(&rules, &msgs[4]));
	assert_eq!(find_valid_count(&rules, &msgs), 2);
}

const EXAMPLE_INPUT_3 :&str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

#[test]
fn test_3() {
	let (mut rules, msgs) = parse_rules_msgs(EXAMPLE_INPUT_3);
	assert!(is_valid(&rules, "bbabbbbaabaabba"));
	assert!(is_valid(&rules, "ababaaaaaabaaab"));
	assert!(is_valid(&rules, "ababaaaaabbbaba"));
	assert_eq!(find_valid_count(&rules, &msgs), 3);

	transform_rules(&mut rules);
	assert!(is_valid(&rules, "bbabbbbaabaabba"));
	assert!(is_valid(&rules, "babbbbaabbbbbabbbbbbaabaaabaaa"));
	assert!(is_valid(&rules, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
	assert!(is_valid(&rules, "bbbbbbbaaaabbbbaaabbabaaa"));
	assert!(is_valid(&rules, "bbbababbbbaaaaaaaabbababaaababaabab"));
	assert!(is_valid(&rules, "ababaaaaaabaaab"));
	assert!(is_valid(&rules, "ababaaaaabbbaba"));
	// ...
	assert_eq!(find_valid_count(&rules, &msgs), 12);
}
