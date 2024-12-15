use super::*;

const EXAMPLE_INPUT_1 :&str = "\
0123
1234
8765
9876
";

const EXAMPLE_INPUT_2 :&str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";

const EXAMPLE_INPUT_3 :&str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

const EXAMPLE_INPUT_4 :&str = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";

const EXAMPLE_INPUT_5 :&str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";


const EXAMPLE_INPUT_6 :&str = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";


#[test]
fn test_1() {
	let eqs = parse(EXAMPLE_INPUT_1);
	assert_eq!(1, trailhead_sum(&eqs));
}

#[test]
fn test_2() {
	let eqs = parse(EXAMPLE_INPUT_2);
	assert_eq!(2, trailhead_sum(&eqs));
}

#[test]
fn test_3() {
	let eqs = parse(EXAMPLE_INPUT_3);
	assert_eq!(4, trailhead_sum(&eqs));
}

#[test]
fn test_4() {
	let eqs = parse(EXAMPLE_INPUT_4);
	assert_eq!(1+2, trailhead_sum(&eqs));
}

#[test]
fn test_5() {
	let eqs = parse(EXAMPLE_INPUT_5);
	assert_eq!(36, trailhead_sum(&eqs));
	assert_eq!(81, trailhead_ratings(&eqs));
}

#[test]
fn test_6() {
	let eqs = parse(EXAMPLE_INPUT_6);
	assert_eq!(3, trailhead_ratings(&eqs));
}
