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
	assert_eq!(do_flips(&cmds), 10);
}

