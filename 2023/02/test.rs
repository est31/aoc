use super::*;

const EXAMPLE_INPUT :&str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[test]
fn test() {
	let games = parse(EXAMPLE_INPUT);
	//println!("games: {games:?}");
	assert_eq!(8, possible_games_id_sum(&games));
	assert_eq!(48, sum_of_min_powers(&games[0..=0]));
	assert_eq!(12, sum_of_min_powers(&games[1..=1]));
	assert_eq!(1560, sum_of_min_powers(&games[2..=2]));
	assert_eq!(630, sum_of_min_powers(&games[3..=3]));
	assert_eq!(36, sum_of_min_powers(&games[4..=4]));
	assert_eq!(2286, sum_of_min_powers(&games));
}
