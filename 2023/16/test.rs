use super::*;

const EXAMPLE_INPUT :&str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

#[test]
fn test_1() {
	let field = parse(EXAMPLE_INPUT);
	assert_eq!(energized_count(&field), 46);
	assert_eq!(energized_count_from_anywhere(&field), 51);
}
