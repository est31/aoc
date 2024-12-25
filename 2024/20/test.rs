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
	assert_eq!(cost_no_cheat, mp.shortest_path().len() as u32 - 1);
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

#[test]
fn test_neighs_manhattan() {
	let c = (10, 100);
	fn tm(c :Pos, max_dist :usize, positions :&[(Pos, usize)]) -> usize {
		let pos_set = positions.iter().collect::<HashSet<_>>();
		assert_eq!(pos_set.len(), positions.len());
		for p in positions {
			let mut sum = 0;
			sum += (c.0 as isize - p.0.0 as isize).abs();
			sum += (c.1 as isize - p.0.1 as isize).abs();
			assert_eq!(sum as usize, p.1);
			assert!(max_dist as isize >= sum);
		}
		positions.len()
	}
	assert_eq!(tm(c, 1, &neighs_manhattan(c, 1000, 1000, 1)), 4);
	assert_eq!(tm(c, 2, &neighs_manhattan(c, 1000, 1000, 2)), 12);
	assert_eq!(tm(c, 3, &neighs_manhattan(c, 1000, 1000, 3)), 24);
}
