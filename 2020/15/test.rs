use super::*;

#[test]
fn test_1() {
	let nums = parse("0,3,6");
	assert_eq!(play_game_n(&nums, 4), 0);
	assert_eq!(play_game_n(&nums, 5), 3);
	assert_eq!(play_game_n(&nums, 6), 3);
	assert_eq!(play_game_n(&nums, 7), 1);
	assert_eq!(play_game_n(&nums, 8), 0);
	assert_eq!(play_game_n(&nums, 9), 4);
	assert_eq!(play_game_n(&nums, 10), 0);

	assert_eq!(play_game(&parse("0,3,6")), 436);
}

#[test]
fn test_2() {
	assert_eq!(play_game(&parse("1,3,2")), 1);
	assert_eq!(play_game(&parse("2,1,3")), 10);
	assert_eq!(play_game(&parse("1,2,3")), 27);
	assert_eq!(play_game(&parse("2,3,1")), 78);
	assert_eq!(play_game(&parse("3,2,1")), 438);
	assert_eq!(play_game(&parse("3,1,2")), 1836);
}
