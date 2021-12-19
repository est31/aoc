use super::*;

const EXAMPLE_INPUT :&str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[test]
fn test_syntax() {
	assert_eq!(compute_syntax_score("{([(<{}[<>[]}>{[]{[(<()>"), 1197);
	assert_eq!(compute_syntax_score(EXAMPLE_INPUT), 26397);
}

#[test]
fn test_middle() {
	//assert_eq!(compute_middle_score("[({(<(())[]>[[{[]{<()<>>"), 288957);
	assert_eq!(compute_middle_score("[(()[<>])]({[<{<<[]>>("), 5566);
	assert_eq!(compute_middle_score(EXAMPLE_INPUT), 288957);
}
