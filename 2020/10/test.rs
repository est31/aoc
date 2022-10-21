use super::*;

const EXAMPLE_INPUT_1 :&str = "\
16
10
15
5
1
11
7
19
6
12
4
";

#[test]
fn test_1() {
	let numbers = parse(EXAMPLE_INPUT_1);
	assert_eq!(device_jolts(&numbers), 22);
	let p = jolts_diff_count(&numbers);
	assert_eq!(p, (8, 7, 5));
}

const EXAMPLE_INPUT_2 :&str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

#[test]
fn test_2() {
	let numbers = parse(EXAMPLE_INPUT_2);
	assert_eq!(device_jolts(&numbers), 52);
	let p = jolts_diff_count(&numbers);
	assert_eq!(p, (19208, 22, 10));
}
