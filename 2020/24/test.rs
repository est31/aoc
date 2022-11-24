use super::*;

#[test]
fn test_simple() {
	let line = parse_line("esenee");
	assert_eq!(line, [Dir::East, Dir::SouthEast, Dir::NorthEast, Dir::East]);

	assert_eq!(coords_for_cmds(&parse_line("nwwswee")), (0, 0));
}

const EXAMPLE_INPUT_1 :&str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

#[test]
fn test_1() {
	let cmds = parse(EXAMPLE_INPUT_1);
	assert_eq!(do_flips(&cmds).len(), 10);
}

#[test]
fn test_2() {
	let cmds = parse(EXAMPLE_INPUT_1);
	let flipped = do_flips(&cmds);
	assert_eq!(flipped.len(), 10);
	assert_eq!(n_days(1, flipped.clone()).len(), 15);
	assert_eq!(n_days(2, flipped.clone()).len(), 12);
	assert_eq!(n_days(3, flipped.clone()).len(), 25);
	assert_eq!(n_days(4, flipped.clone()).len(), 14);
	assert_eq!(n_days(5, flipped.clone()).len(), 23);
	assert_eq!(n_days(6, flipped.clone()).len(), 28);
	assert_eq!(n_days(7, flipped.clone()).len(), 41);
	assert_eq!(n_days(8, flipped.clone()).len(), 37);
	assert_eq!(n_days(9, flipped.clone()).len(), 49);
	assert_eq!(n_days(10, flipped.clone()).len(), 37);
	// ...
	assert_eq!(n_days(20, flipped.clone()).len(), 132);
	assert_eq!(n_days(30, flipped.clone()).len(), 259);
	assert_eq!(n_days(40, flipped.clone()).len(), 406);
	assert_eq!(n_days(50, flipped.clone()).len(), 566);
	assert_eq!(n_days(60, flipped.clone()).len(), 788);
	assert_eq!(n_days(70, flipped.clone()).len(), 1106);
	assert_eq!(n_days(80, flipped.clone()).len(), 1373);
	assert_eq!(n_days(90, flipped.clone()).len(), 1844);
	assert_eq!(n_days(100, flipped.clone()).len(), 2208);
}
