use super::*;

const EXAMPLE_INPUT :&str = "\
F10
N3
F7
R90
F11
";

#[test]
fn test_1() {
	let mut cfg = ShipConfig::new();
	cfg.go("F10");
	assert_eq!((10, 0), (cfg.x, cfg.y));
	cfg.go("N3");
	assert_eq!((10, 3), (cfg.x, cfg.y));
	cfg.go("F7");
	assert_eq!((17, 3), (cfg.x, cfg.y));
	cfg.go("R90");
	assert_eq!((17, 3), (cfg.x, cfg.y));
	cfg.go("F11");
	assert_eq!((17, -8), (cfg.x, cfg.y));

	assert_eq!(25, cfg.manhattan());
}

#[test]
fn test_2() {
	let mut cfg = ShipConfig::new();
	cfg.apply(EXAMPLE_INPUT);
	assert_eq!(17, cfg.x);
	assert_eq!(-8, cfg.y);
	assert_eq!(25, cfg.manhattan());
}

#[test]
fn test_3() {
	let mut cfg = ShipConfigAdv::new();
	cfg.apply(EXAMPLE_INPUT);
	assert_eq!(214, cfg.x);
	assert_eq!(-72, cfg.y);
	assert_eq!(286, cfg.manhattan());
}
