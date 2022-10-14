use super::*;

const EXAMPLE_INPUT :&str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

#[test]
fn test_1() {
	let scene = Scene::parse(EXAMPLE_INPUT);
	assert_eq!(EXAMPLE_INPUT, scene.to_string());
	assert!(!scene.is_perfect());

	let mut search = SceneSearch::new(scene);
	assert_eq!(Some(12521), search.search_for_steps(20_000));
}

const EXAMPLE_INPUT_UNFOLDED :&str = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
";

#[test]
fn test_1_unfolded() {
	let scene = Scene::parse_with_unfolded(EXAMPLE_INPUT, true);
	assert_eq!(EXAMPLE_INPUT_UNFOLDED, scene.to_string());
	assert!(!scene.is_perfect());

	let mut search = SceneSearch::new(scene);
	assert_eq!(Some(44169), search.search_for_steps(20_000));
}

const END_STATE :&str = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
";

#[test]
fn test_end_state() {
	let scene = Scene::parse(END_STATE);
	assert_eq!(END_STATE, scene.to_string());
	assert!(scene.is_perfect());

	let mut search = SceneSearch::new(scene);
	assert_eq!(Some(0), search.search_for_steps(3));
}

const BURIED_IMPERFECT :&str = "\
#############
#...........#
###A#B#C#D###
  #B#A#C#D#
  #########
";

#[test]
fn test_buried_imperfect() {
	let scene = Scene::parse(BURIED_IMPERFECT);
	assert_eq!(BURIED_IMPERFECT, scene.to_string());
	assert!(!scene.is_perfect());
	assert_eq!(scene.imperfect_amphipods.len(), 4);

	let mut search = SceneSearch::new(scene);
	assert!(search.search_for_steps(300).is_some());
}

const EXAMPLE_INPUT_FINAL_M1 :&str = "\
#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########
";

#[test]
fn test_1_m1() {
	let scene = Scene::parse(EXAMPLE_INPUT_FINAL_M1);
	assert_eq!(EXAMPLE_INPUT_FINAL_M1, scene.to_string());
	assert!(!scene.is_perfect());

	let mut search = SceneSearch::new(scene);
	assert_eq!(Some(8), search.search_for_steps(2));
}


const EXAMPLE_INPUT_FINAL_M2 :&str = "\
#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#
  #########
";

#[test]
fn test_1_m2() {
	let scene = Scene::parse(EXAMPLE_INPUT_FINAL_M2);
	assert_eq!(EXAMPLE_INPUT_FINAL_M2, scene.to_string());
	assert!(!scene.is_perfect());

	let mut search = SceneSearch::new(scene);
	assert_eq!(Some(7008), search.search_for_steps(100));
}
