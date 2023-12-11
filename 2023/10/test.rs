use super::*;

const EXAMPLE_INPUT_1 :&str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

const EXAMPLE_INPUT_2 :&str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

const EXAMPLE_INPUT_3 :&str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

const EXAMPLE_INPUT_4 :&str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

const EXAMPLE_INPUT_5 :&str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[test]
fn test_1() {
	let field = parse(EXAMPLE_INPUT_1);
	assert_eq!(farthest(&field), 4);

	let field = parse(EXAMPLE_INPUT_2);
	assert_eq!(farthest(&field), 8);
}

#[test]
fn test_2() {
	let field = parse(EXAMPLE_INPUT_3);
	assert_eq!(enclosed_by_loop(&field), 4);

	let field = parse(EXAMPLE_INPUT_4);
	assert_eq!(enclosed_by_loop(&field), 8);

	let field = parse(EXAMPLE_INPUT_5);
	assert_eq!(enclosed_by_loop(&field), 10);
}
