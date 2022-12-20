use super::*;

const EXAMPLE_INPUT :&str = "\
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
";

#[test]
fn test_1() {
	let bps = parse(EXAMPLE_INPUT);
	println!("{bps:?}");

	let sum = quality_level_sum(&bps[..1]);
	assert_eq!(sum, 9);

	let sum = quality_level_sum(&bps);
	assert_eq!(sum, 33);
}

#[test]
fn test_1_20() {
	let bps = parse(EXAMPLE_INPUT);
	let bp = &bps[0];
	println!("{bp:?}");

	// "Minute 9"
	let st = State {
		bp,
		resources : [3, 12, 0, 0],
		robots : [1, 3, 0, 0],
		building : None,
		time_rem : 15,
	};
	let gto = geodes_to_open_st(st);
	assert_eq!(gto, 9);

	// "Minute 16"
	let st = State {
		bp,
		resources : [2, 9, 6, 0],
		robots : [1, 4, 2, 0],
		building : None,
		time_rem : 8,
	};
	let gto = geodes_to_open_st(st);
	assert_eq!(gto, 9);

	// "Minute 19"
	let st = State {
		bp,
		resources : [3, 21, 5, 1],
		robots : [1, 4, 2, 1],
		building : None,
		time_rem : 5,
	};
	let gto = geodes_to_open_st(st);
	assert_eq!(gto, 9);
}

#[test]
fn test_2() {
	let bps = parse(EXAMPLE_INPUT);

	assert_eq!(geodes_to_open(bps[0], 32), 56);
	assert_eq!(geodes_to_open(bps[1], 32), 62);
}
