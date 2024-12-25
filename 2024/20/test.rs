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

	// Test given manhattan sphere conforms with:
	// * given dist from center is accurate
	// * given dist from center <= radius
	// * all points unique, no doubles
	fn tm(c :Pos, radius :usize, positions :&[(Pos, usize)]) -> usize {
		let pos_set = positions.iter().map(|v| v.0).collect::<HashSet<_>>();
		assert_eq!(pos_set.len(), positions.len());
		for p in positions {
			let mut sum = 0;
			sum += (c.0 as isize - p.0.0 as isize).abs();
			sum += (c.1 as isize - p.0.1 as isize).abs();
			assert_eq!(sum as usize, p.1);
			assert!(radius as isize >= sum);
		}
		positions.len()
	}
	fn tmc(c :Pos, radius :usize, positions :&[(Pos, usize)]) -> usize {
		// OEIS A001844
		// Lemma 1 in https://doi.org/10.3390/e26040317
		let wanted_num = 2 * radius * (radius + 1);
		assert_eq!(positions.len(), wanted_num, "r={radius}, c={c:?}");
		tm(c, radius, positions)
	}
	assert_eq!(tmc(c, 1, &neighs_manhattan(c, 1000, 1000, 1)), 4);
	assert_eq!(tmc(c, 2, &neighs_manhattan(c, 1000, 1000, 2)), 12);

	assert_eq!(tmc(c, 3, &neighs_manhattan(c, 1000, 1000, 3)), 24);

	let r = 3;
	tm(c, r, &neighs_manhattan(c, 1000, 1000, r));
	assert_eq!(tmc(c, 4, &neighs_manhattan(c, 1000, 1000, 4)), 40);
	assert_eq!(tmc(c, 5, &neighs_manhattan(c, 1000, 1000, 5)), 60);
	assert_eq!(tmc(c, 6, &neighs_manhattan(c, 1000, 1000, 6)), 84);
	for r in 1..25 {
		let nc = (30, 50);
		tmc(nc, r, &neighs_manhattan(nc, 1000, 1000, r));
	}
	// Partially cut off
	assert_eq!(tm(c, 3, &neighs_manhattan(c, 101, 101, 3)), 24 - 9);
}
