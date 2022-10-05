use super::*;

const EXAMPLE_INPUT_1 :&str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
";

#[test]
fn test_1_simple() {
	let cmds = parse_commands(EXAMPLE_INPUT_1);
	assert_eq!(run_commands_simple(&cmds[..1]).len(), 27);
	assert_eq!(run_commands_simple(&cmds[..2]).len(), 27 + 19);
	assert_eq!(run_commands_simple(&cmds[..3]).len(), 27 + 19 - 8);
	assert_eq!(run_commands_simple(&cmds).len(), 39);
}

#[test]
fn test_1() {
	let cmds = parse_commands(EXAMPLE_INPUT_1);
	assert_eq!(run_commands(&cmds[..1]), 27);
	assert_eq!(run_commands(&cmds[..2]), 27 + 19);
	assert_eq!(run_commands(&cmds[..3]), 27 + 19 - 8);
	assert_eq!(run_commands(&cmds), 39);
}

const EXAMPLE_INPUT_1_MODIFIED :&str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
";

#[test]
fn test_1_modified() {
	let cm = parse_commands(EXAMPLE_INPUT_1_MODIFIED);
	let rcs = run_commands_simple;
	assert_eq!(run_commands(&cm[..1]) as usize, rcs(&cm[..1]).len());
	assert_eq!(run_commands(&cm[..2]) as usize, rcs(&cm[..2]).len());
	assert_eq!(run_commands(&cm[..3]) as usize, rcs(&cm[..3]).len());
	assert_eq!(run_commands(&cm) as usize, rcs(&cm).len());
}

const INPUT_0 :&str = "\
on x=10..12,y=10..12,z=10..12
on x=10..12,y=10..12,z=10..14
off x=10..12,y=10..12,z=10..13
";

#[test]
fn test_0() {
	let cm = parse_commands(INPUT_0);
	let rcs = run_commands_simple;
	assert_eq!(run_commands(&cm[..1]) as usize, rcs(&cm[..1]).len());
	assert_eq!(run_commands(&cm[..2]) as usize, rcs(&cm[..2]).len());
	assert_eq!(run_commands(&cm[..3]) as usize, rcs(&cm[..3]).len());
	assert_eq!(run_commands(&cm) as usize, rcs(&cm).len());
}

fn cube_list() -> Vec<Cube> {
	let mut cube_list = Vec::new();
	let l = 2;
	let m = 2;
	for x_min in -l..=l {
		for x_max in x_min..=l {
			for y_min in -l..=l {
				for y_max in y_min..=l {
					for z_min in -l..=l {
						for z_max in z_min..=l {
							let xr = (x_min * m)..=(x_max * m);
							let yr = (y_min * m)..=(y_max * m);
							let zr = (z_min * m)..=(z_max * m);
							cube_list.push([xr, yr, zr]);
						}
					}
				}
			}
		}
	}
	cube_list
}

#[test]
fn test_simple_normal_same_true_true() {
	test_simple_normal_same::<true, true>();
}

#[test]
fn test_simple_normal_same_true_false() {
	test_simple_normal_same::<true, false>();
}

#[test]
fn test_simple_normal_same_false_true() {
	test_simple_normal_same::<false, true>();
}

#[test]
fn test_simple_normal_same_false_false() {
	test_simple_normal_same::<false, false>();
}

fn test_simple_normal_same<const C1 :bool, const C2 :bool>() {
	let cube_list = cube_list();
	let m = 2;
	let cube_2 = [-m..=(2 * m), -m..=(2 * m), -m..=(2 * m)];
	for cube in cube_list.iter() {
		let cmds = [(C1, cube.clone()), (C2, cube_2.clone())];
		println!("{:?}", cmds);
		assert_eq!(run_commands(&cmds), run_commands_simple(&cmds).len() as u64);
	}
}

const EXAMPLE_INPUT_2 :&str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

#[test]
fn test_2() {
	let cmds = parse_commands(EXAMPLE_INPUT_2);
	assert_eq!(run_commands(&cmds), 590784);
}
