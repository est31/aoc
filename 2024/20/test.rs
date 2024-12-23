use super::*;

const EXAMPLE_INPUT_1 :&str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[test]
fn test_1() {
	let mp = parse(EXAMPLE_INPUT_1);
	let cost_no_cheat = mp.search();
	let db = mp.make_cheats_db();
	assert_eq!(14, count_cheats_saving(cost_no_cheat, &db, 2));
	assert_eq!(14, count_cheats_saving(cost_no_cheat, &db, 4));
	assert_eq!(2, count_cheats_saving(cost_no_cheat, &db, 6));
	assert_eq!(4, count_cheats_saving(cost_no_cheat, &db, 8));
	assert_eq!(2, count_cheats_saving(cost_no_cheat, &db, 10));
	assert_eq!(3, count_cheats_saving(cost_no_cheat, &db, 12));
	assert_eq!(1, count_cheats_saving(cost_no_cheat, &db, 20));
	assert_eq!(1, count_cheats_saving(cost_no_cheat, &db, 36));
	assert_eq!(1, count_cheats_saving(cost_no_cheat, &db, 38));
	assert_eq!(1, count_cheats_saving(cost_no_cheat, &db, 40));
	assert_eq!(1, count_cheats_saving(cost_no_cheat, &db, 64));
}

