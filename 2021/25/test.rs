use super::*;

const EXAMPLE_1 :&str = "\
...>>>>>...
";

const EXAMPLE_1_1 :&str = "\
...>>>>.>..
";

const EXAMPLE_1_2 :&str = "\
...>>>.>.>.
";

#[test]
fn test_1() {
	let mut sc = Scene::parse(EXAMPLE_1);
	assert_eq!(format!("{sc}"), EXAMPLE_1);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_1_1);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_1_2);
}

const EXAMPLE_2 :&str = "\
..........
.>v....v..
.......>..
..........
";


const EXAMPLE_2_1 :&str = "\
..........
.>........
..v....v>.
..........
";

#[test]
fn test_2() {
	let mut sc = Scene::parse(EXAMPLE_2);
	assert_eq!(format!("{sc}"), EXAMPLE_2);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_2_1);
}

const EXAMPLE_3 :&str = "\
...>...
.......
......>
v.....>
......>
.......
..vvv..
";

const EXAMPLE_3_1 :&str = "\
..vv>..
.......
>......
v.....>
>......
.......
....v..
";

const EXAMPLE_3_2 :&str = "\
....v>.
..vv...
.>.....
......>
v>.....
.......
.......
";

const EXAMPLE_3_3 :&str = "\
......>
..v.v..
..>v...
>......
..>....
v......
.......
";

const EXAMPLE_3_4 :&str = "\
>......
..v....
..>.v..
.>.v...
...>...
.......
v......
";

#[test]
fn test_3() {
	let mut sc = Scene::parse(EXAMPLE_3);
	assert_eq!(format!("{sc}"), EXAMPLE_3);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_3_1);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_3_2);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_3_3);
	sc.step();
	assert_eq!(format!("{sc}"), EXAMPLE_3_4);
}

const EXAMPLE_4 :&str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

const EXAMPLE_4_1 :&str = "\
....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v
";

const EXAMPLE_4_2 :&str = "\
>.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv.
";

const EXAMPLE_4_3 :&str = "\
v>v.v>.>v.
v...>>.v.v
>vv>.>v>..
>>v>v.>.v>
..>....v..
.>.>v>v..v
..v..v>vv>
v.v..>>v..
.v>....v..
";

const EXAMPLE_4_4 :&str = "\
v>..v.>>..
v.v.>.>.v.
>vv.>>.v>v
>>.>..v>.>
..v>v...v.
..>>.>vv..
>.v.vv>v.v
.....>>vv.
vvv>...v..
";

const EXAMPLE_4_5 :&str = "\
vv>...>v>.
v.v.v>.>v.
>.v.>.>.>v
>v>.>..v>>
..v>v.v...
..>.>>vvv.
.>...v>v..
..v.v>>v.v
v.v.>...v.
";

const EXAMPLE_4_10 :&str = "\
..>..>>vv.
v.....>>.v
..v.v>>>v>
v>.>v.>>>.
..v>v.vv.v
.v.>>>.v..
v.v..>v>..
..v...>v.>
.vv..v>vv.
";

const EXAMPLE_4_20 :&str = "\
v>.....>>.
>vv>.....v
.>v>v.vv>>
v>>>v.>v.>
....vv>v..
.v.>>>vvv.
..v..>>vv.
v.v...>>.v
..v.....v>
";

const EXAMPLE_4_30 :&str = "\
.vv.v..>>>
v>...v...>
>.v>.>vv.>
>v>.>.>v.>
.>..v.vv..
..v>..>>v.
....v>..>v
v.v...>vv>
v.v...>vvv
";

const EXAMPLE_4_40 :&str = "\
>>v>v..v..
..>>v..vv.
..>>>v.>.v
..>>>>vvv>
v.....>...
v.v...>v>>
>vv.....v>
.>v...v.>v
vvv.v..v.>
";

const EXAMPLE_4_50 :&str = "\
..>>v>vv.v
..v.>>vv..
v.>>v>>v..
..>>>>>vv.
vvv....>vv
..v....>>>
v>.......>
.vv>....v>
.>v.vv.v..
";

const EXAMPLE_4_55 :&str = "\
..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv...>..>
>vv.....>.
.>v.vv.v..
";

const EXAMPLE_4_56 :&str = "\
..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv....>.>
>vv......>
.>v.vv.v..
";

#[test]
fn test_4() {
	let mut sc = Scene::parse(EXAMPLE_4);
	let mut steps = 0;
	while sc.step() {
		steps += 1;
		match steps {
			1 => assert_eq!(sc.to_string(), EXAMPLE_4_1),
			2 => assert_eq!(sc.to_string(), EXAMPLE_4_2),
			3 => assert_eq!(sc.to_string(), EXAMPLE_4_3),
			4 => assert_eq!(sc.to_string(), EXAMPLE_4_4),
			5 => assert_eq!(sc.to_string(), EXAMPLE_4_5),
			10 => assert_eq!(sc.to_string(), EXAMPLE_4_10),
			20 => assert_eq!(sc.to_string(), EXAMPLE_4_20),
			30 => assert_eq!(sc.to_string(), EXAMPLE_4_30),
			40 => assert_eq!(sc.to_string(), EXAMPLE_4_40),
			50 => assert_eq!(sc.to_string(), EXAMPLE_4_50),
			55 => assert_eq!(sc.to_string(), EXAMPLE_4_55),
			56 => assert_eq!(sc.to_string(), EXAMPLE_4_56),
			_ => (),
		}
		if steps > 1000 {
			panic!();
		}
	}
	steps += 1;
	assert_eq!(steps, 58);
}
