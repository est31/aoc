use super::*;

const EXAMPLE_INPUT :&str = "\
Player 1 starting position: 4
Player 2 starting position: 8
";

#[test]
fn test() {
	let sps = obtain_starting_positions(EXAMPLE_INPUT);
	assert_eq!(sps, (4,8));

	assert_eq!(find_end_state(sps), (993, (1000, 745)));
	assert_eq!(play_game(sps), 739785);
}

#[test]
fn test_dirac() {
	let sps = obtain_starting_positions(EXAMPLE_INPUT);
	assert_eq!(sps, (4,8));

	assert_eq!(dirac_universes(sps), (444356092776315, 341960390180808));

	assert_eq!(play_dirac_game(sps), 444356092776315);
}
