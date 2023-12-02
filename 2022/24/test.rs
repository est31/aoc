use super::*;

const EXAMPLE_INPUT_1 :&str ="\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
";

const EXAMPLE_INPUT_1_1 :&str ="\
#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#
";

const EXAMPLE_INPUT_1_2 :&str ="\
#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#
";

const EXAMPLE_INPUT_1_3 :&str ="\
#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#
";

const EXAMPLE_INPUT_1_4 :&str ="\
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#
";

const EXAMPLE_INPUT_2 :&str ="\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

const EXAMPLE_INPUT_2_1 :&str ="\
#.######
#.>3.<.#
#<..<<.#
#>2.22.#
#>v..^<#
######.#
";

const EXAMPLE_INPUT_2_2 :&str ="\
#.######
#.2>2..#
#.^22^<#
#.>2.^>#
#.>..<.#
######.#
";

const EXAMPLE_INPUT_2_3 :&str ="\
#.######
#<^<22.#
#.2<.2.#
#><2>..#
#..><..#
######.#
";

const EXAMPLE_INPUT_2_4 :&str ="\
#.######
#.<..22#
#<<.<..#
#<2.>>.#
#.^22^.#
######.#
";

const EXAMPLE_INPUT_2_10 :&str ="\
#.######
#.2..>2#
#<2v2^.#
#<>.>2.#
#..<>..#
######.#
";

#[test]
fn test_1_steps() {
	let mut field = parse(EXAMPLE_INPUT_1);
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_1);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_1_1);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_1_2);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_1_3);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_1_4);
}

#[test]
fn test_2_steps() {
	let mut field = parse(EXAMPLE_INPUT_2);
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2_1);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2_2);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2_3);
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2_4);
	field.step();
	field.step();
	field.step();
	field.step();
	field.step();
	field.step();
	assert_eq!(format!("{}", field.fields), EXAMPLE_INPUT_2_10);
}

#[test]
fn test_2() {
	let mut field = parse(EXAMPLE_INPUT_2);
	println!("\n-------------\nStarting field: w={} h={}", field.width, field.height);
	println!("{}", field.fields);
	let sp = find_shortest_path(&mut field);
	println!("{}", sp);
	assert_eq!(sp, 18);
}
