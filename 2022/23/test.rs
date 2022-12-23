use super::*;

const EXAMPLE_INPUT_1 :&str ="\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

const EXAMPLE_INPUT_1_0 :&str = "\
.....#...
...#...#.
.#..#.#..
.....#..#
..#.#.##.
#..#.#...
#.#.#.##.
.........
..#..#...
";

const EXAMPLE_INPUT_1_1 :&str = "\
......#....
...#.....#.
..#..#.#...
......#...#
..#..#.#...
#...#.#.#..
...........
.#.#.#.##..
...#..#....
";

const EXAMPLE_INPUT_1_2 :&str = "\
......#....
....#....#.
.#..#...#..
......#...#
..#..#.#...
#..#.....#.
......##...
.##.#....#.
..#........
......#....
";

const EXAMPLE_INPUT_1_3 :&str = "\
......#....
.....#....#
.#...##....
..#.....#.#
........#..
#...###..#.
.#......#..
...##....#.
...#.......
......#....
";

const EXAMPLE_INPUT_1_9 :&str = "\
......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..
";


fn step(field :&HashSet<(i16, i16)>, si :usize) -> HashSet<(i16, i16)> {
	super::step(field, si).0
}

#[test]
fn test_1() {
	let field = parse(EXAMPLE_INPUT_1);
	println!("\n-------------\nStarting field:");
	print_field(&field);
	let e = empty_ground_tiles(&field);
	assert_eq!(e, 110);
}

#[test]
fn test_1_1() {
	let field = parse(EXAMPLE_INPUT_1);
	println!("\n-------------\nStarting field:");
	print_field(&field);

	let fld = step(&field, 0);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 0:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_1_0, field_str);

	let fld = step(&fld, 1);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 1:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_1_1, field_str);

	let fld = step(&fld, 2);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 2:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_1_2, field_str);

	let fld = step(&fld, 3);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 3:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_1_3, field_str);


	let fld = step(&fld, 4);
	let fld = step(&fld, 5);
	let fld = step(&fld, 6);
	let fld = step(&fld, 7);
	let fld = step(&fld, 8);

	let fld = step(&fld, 9);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 9:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_1_9, field_str);
}

const EXAMPLE_INPUT_2 :&str ="\
.....
..##.
..#..
.....
..##.
.....
";

const EXAMPLE_INPUT_2_0 :&str ="\
##
..
#.
.#
#.
";

const EXAMPLE_INPUT_2_1 :&str ="\
.##.
#...
...#
....
.#..
";

const EXAMPLE_INPUT_2_2 :&str ="\
..#..
....#
#....
....#
.....
..#..
";

#[test]
fn test_1_2() {
	let field = parse(EXAMPLE_INPUT_2);
	println!("\n-------------\nStarting field:");
	print_field(&field);

	let fld = step(&field, 0);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 0:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_2_0, field_str);

	let fld = step(&fld, 1);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 1:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_2_1, field_str);

	let fld = step(&fld, 2);
	let field_str = field_to_str(&fld);
	println!("\n-------------\nAfter step 2:\n{field_str}");
	assert_eq!(EXAMPLE_INPUT_2_2, field_str);
}

#[test]
fn test_2() {
	let field = parse(EXAMPLE_INPUT_1);
	let r = rounds_until_stop(&field);
	assert_eq!(r, 20);

	let field = parse(EXAMPLE_INPUT_2);
	let r = rounds_until_stop(&field);
	assert_eq!(r, 4);
}
