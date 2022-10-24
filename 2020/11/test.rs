use super::*;

const EXAMPLE_INPUT :&str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

const EXAMPLE_INPUT_1 :&str = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";

const EXAMPLE_INPUT_2 :&str = "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";

const EXAMPLE_INPUT_3 :&str = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
";

const EXAMPLE_INPUT_4 :&str = "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
";

const EXAMPLE_INPUT_5 :&str = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";

#[test]
fn test_1() {
	let start_fields = parse(EXAMPLE_INPUT);
	let s = fields_to_string(&start_fields);
	assert_eq!(EXAMPLE_INPUT, s);

	let fields = step(&start_fields).1;
	assert_eq!(EXAMPLE_INPUT_1, fields_to_string(&fields));

	assert_eq!(2, count_occupied_adjacent(&fields, 0, 0));
	assert_eq!(4, count_occupied_adjacent(&fields, 0, 2));
	assert_eq!(4, count_occupied_adjacent(&fields, 0, 3));

	let fields = step(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2, fields_to_string(&fields));

	let fields = step(&fields).1;
	assert_eq!(EXAMPLE_INPUT_3, fields_to_string(&fields));

	let fields = step(&fields).1;
	assert_eq!(EXAMPLE_INPUT_4, fields_to_string(&fields));

	let fields = step(&fields).1;
	assert_eq!(EXAMPLE_INPUT_5, fields_to_string(&fields));

	let (changes, fields) = step(&fields);
	assert_eq!(EXAMPLE_INPUT_5, fields_to_string(&fields));
	assert_eq!(false, changes);

	assert_eq!(37, step_until_no_change(&start_fields, step));
}

const EXAMPLE_INPUT_2_2 :&str = "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
";

const EXAMPLE_INPUT_2_3 :&str = "\
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
";

const EXAMPLE_INPUT_2_4 :&str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
";

const EXAMPLE_INPUT_2_5 :&str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
";

const EXAMPLE_INPUT_2_6 :&str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
";

#[test]
fn test_2() {
	let start_fields = parse(EXAMPLE_INPUT);
	let s = fields_to_string(&start_fields);
	assert_eq!(EXAMPLE_INPUT, s);

	let fields = step_p2(&start_fields).1;
	assert_eq!(EXAMPLE_INPUT_1, fields_to_string(&fields));

	let fields = step_p2(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2_2, fields_to_string(&fields));

	let fields = step_p2(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2_3, fields_to_string(&fields));

	let fields = step_p2(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2_4, fields_to_string(&fields));

	let fields = step_p2(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2_5, fields_to_string(&fields));

	let fields = step_p2(&fields).1;
	assert_eq!(EXAMPLE_INPUT_2_6, fields_to_string(&fields));

	let (changes, fields) = step_p2(&fields);
	assert_eq!(EXAMPLE_INPUT_2_6, fields_to_string(&fields));
	assert_eq!(false, changes);

	assert_eq!(26, step_until_no_change(&start_fields, step_p2));
}
