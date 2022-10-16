use super::*;

const EXAMPLE_INPUT :&str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

#[test]
fn test_1() {
	let lines = parse(EXAMPLE_INPUT);
	assert_eq!(lines.len(), 3);
	assert!(check_conforms_1(&lines[0]));
	assert!(!check_conforms_1(&lines[1]));
	assert!(check_conforms_1(&lines[2]));
	assert_eq!(count_conformances_1(&lines), 2);
}

#[test]
fn test_2() {
	let lines = parse(EXAMPLE_INPUT);
	assert_eq!(lines.len(), 3);
	assert!(check_conforms_2(&lines[0]));
	assert!(!check_conforms_2(&lines[1]));
	assert!(!check_conforms_2(&lines[2]));
	assert_eq!(count_conformances_2(&lines), 1);
}
