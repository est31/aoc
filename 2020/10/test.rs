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
	assert_eq!(p, (7, 5));

	let p = jolts_combinations(&numbers);
	assert_eq!(p, 8);
	let p = jolts_combinations_slow(&numbers);
	assert_eq!(p, 8);
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
	assert_eq!(p, (22, 10));

	let p = jolts_combinations(&numbers);
	assert_eq!(p, 19208);
	let p = jolts_combinations_slow(&numbers);
	assert_eq!(p, 19208);
}

#[test]
fn test_3() {
	let numbers = (0..=30).collect::<Vec<_>>();
	// https://oeis.org/A000073
	let tribonacci = [
		1, 1, 2, 4, 7, 13, 24, 44, 81,
		149, 274, 504, 927, 1705, 3136,
		5768, 10609, 19513, 35890, 66012,
		121415, 223317, 410744, 755476,
		1389537, 2555757, 4700770,
		8646064, 15902591, 29249425,
	];
	for i in 0..20 {
		let nums = &numbers[..=i];
		let p_sl = jolts_combinations_slow(nums);
		let p = jolts_combinations(nums);
		println!("{p} {p_sl}");
		assert_eq!(p, p_sl);
		assert_eq!(p, tribonacci[i]);
	}
	for i in 20..30 {
		let nums = &numbers[..=i];
		let p = jolts_combinations(nums);
		println!("{p}");
		assert_eq!(p, tribonacci[i]);
	}
}
