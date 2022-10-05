use std::str::FromStr;
use std::ops::RangeInclusive;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let commands = parse_commands(INPUT);
	println!("Enabled after commands: {}", run_commands_simple(&commands).len());
}

type Cube = [RangeInclusive<i32>; 3];

fn parse_commands(input :&str) -> Vec<(bool, Cube)> {
	input.lines()
		.map(|l| l.trim())
		.map(|l| {
			let (on, l) = match &l[..2] {
				"on" => (true, &l[3..]),
				"of" => (false, &l[4..]),
				_ => panic!("Invalid line '{}'", l),
			};
			let mut it = l.split(',');
			let rx = parse_range(it.next().unwrap());
			let ry = parse_range(it.next().unwrap());
			let rz = parse_range(it.next().unwrap());
			(on, [rx, ry, rz])
		})
		.collect::<Vec<_>>()
}

fn parse_range(s :&str) -> RangeInclusive<i32> {
	let s = &s[2..];
	let mut it = s.split('.');
	let st = i32::from_str(it.next().unwrap()).unwrap();
	assert_eq!("", it.next().unwrap());
	let end = i32::from_str(it.next().unwrap()).unwrap();
	st..=end
}

fn limit(r :&RangeInclusive<i32>) -> RangeInclusive<i32> {
	(*r.start().max(&-50))..=(*r.end().min(&50))
}

fn run_commands_simple(cmds :&[(bool, Cube)]) -> HashSet<(i32, i32, i32)> {
	let mut enabled = HashSet::new();
	for (cmd, ranges) in cmds {
		for x in limit(&ranges[0]) {
			for y in limit(&ranges[1]) {
				for z in limit(&ranges[2]) {
					if *cmd {
						enabled.insert((x, y, z));
					} else {
						enabled.remove(&(x, y, z));
					}
				}
			}
		}
	}
	enabled
}

fn cubes_overlap(c1 :&Cube, c2 :&Cube) -> bool {
	c1.iter()
		.zip(c2)
		.all(|(r1, r2)| r1.contains(r2.start()) || r1.contains(r2.end())
			|| r2.contains(r1.start()) || r2.contains(r1.end()))
}

/*
fn interval_intersection(r1 :&RangeInclusive<i32>, r2 :&RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
	match (r1.contains(r2.start()), r1.contains(r2.end())) {
		// r2 is a subset of r1.
		// [ ( ) ]
		(true, true) => Some(r2.clone()),

		// r2 starts before r1, but ends in r1.
		// ( [ ) ]
		(false, true) => Some((*r1.start())..=(*r2.end())),

		// r2 starts inside r1, but ends outside of it.
		// [ ( ] )
		(true, false) => Some((*r2.start())..=(*r1.end())),

		// r2 neither starts nor ends in r1.
		// We need to look further.
		(false, false) => match (r2.contains(r1.start()), r2.contains(r1.end())) {
			// r1 is a subset of r2.
			(true, true) => Some(r1.clone()),
			// Disjoint intervals
			(false, false) => None,
			// This would imply that at least end or start of r2
			// are inside r1, which isn't the case. So unreachable.
			_ => unreachable!(),
		},
	}
}

fn cubes_intersection(c1 :&Cube, c2 :&Cube) -> Cube {
	let x_int = interval_intersection(&c1[0], &c2[0]).unwrap();
	let y_int = interval_intersection(&c1[1], &c2[1]).unwrap();
	let z_int = interval_intersection(&c1[2], &c2[2]).unwrap();
	[x_int, y_int, z_int]
}

fn run_commands(cmds :&[(bool, Cube)]) -> u64 {
	let mut sum :i64 = 0;

	println!("number of commands: {}", cmds.len());
	for (i, (cmd, ranges)) in cmds.iter().enumerate() {
		// In the outer loop, we are only interested in
		// positive commands
		if !*cmd {
			continue;
		}
		sum += cube_size(ranges);
		println!("sum is {}", sum);
		if i + 1 == cmds.len() {
			continue;
		}
		let intersections = cmds[(i+1)..].iter()
			.filter(|(_, c)| cubes_overlap(&ranges, &cranges))
			.map(|(ccmd, cranges)| cubes_intersection(ranges, cranges))
			.collect::<Vec<_>>();
		/*
		for (_ccmd, cranges) in cmds[(i+1)..].iter() {
			if !cubes_overlap(&ranges, &cranges) {
				continue;
			}
			sum -= cube_size(&cubes_intersection(ranges, cranges));
			println!("    sum is {}", sum);
		}*/
		// TODO: avoid "double removals" by adding back intersections of any pair
		// of removals
	}
	assert!(sum >= 0, "sum is {} and thus smaller than 0", sum);
	sum as u64
}
*/

fn cube_size(c :&Cube) -> i64 {
	fn siz(r :&RangeInclusive<i32>) -> i64 {
		(*r.end() - *r.start()).abs() as i64 + 1
	}
	siz(&c[0]) * siz(&c[1]) * siz(&c[2])
}

fn interval_difference(r1 :&RangeInclusive<i32>, r2 :&RangeInclusive<i32>) -> Vec<RangeInclusive<i32>> {
	if r1.contains(r2.start()) || r1.contains(r2.end()) {
		// One of start and end of r2 is inside r1.
		// One of three cases (modulo equality/overlap):
		// ( [ ) ]
		//          [ ( ) ]
		//                   [ ( ] )
		// We return up to two different ranges
		let mut res = Vec::new();
		if r1.start() < r2.start() {
			res.push(*r1.start()..=(r2.start() - 1));
		}
		if r2.end() < r1.end() {
			res.push((r2.end() + 1)..=*r1.end());
		}
		res
	} else if r2.contains(r1.start()) && r2.contains(r1.end()) {
		// r1 wholly inside r2. Delete r1.
		vec![]
	} else {
		// Disjoint intervals. Not allowed.
		panic!("Intervals {:?} and {:?} are disjoint!", r1, r2);
	}
}

/// Determines the difference of two (overlapping) cubes as list of cubes.
///
/// Remove c2 from c1
fn cubes_difference(c1 :&Cube, c2 :&Cube) -> Vec<Cube> {
	let x_diff = interval_difference(&c1[0], &c2[0]);
	let y_diff = interval_difference(&c1[1], &c2[1]);
	let z_diff = interval_difference(&c1[2], &c2[2]);

	let mut intervals = vec![(x_diff, 0), (y_diff, 1), (z_diff, 2)];
	intervals.sort_by_key(|(diff, _c)| diff.len());
	intervals.reverse();

	/*for (diffs, coord) in intervals[0] {
		diffs
	}*/

	let mut res = Vec::new();
	res
}

fn run_commands(cmds :&[(bool, Cube)]) -> u64 {
	let mut cubes = HashSet::<Cube>::new();

	for (cmd, ranges) in cmds {
		let mut new_cubes = HashSet::new();
		for cube in cubes.iter() {
			if !cubes_overlap(cube, &ranges) {
				new_cubes.insert(cube.clone());
				continue;
			}
			let diff = cubes_difference(&cube, &ranges);
			for c in diff {
				new_cubes.insert(c);
			}
		}
		if *cmd {
			new_cubes.insert(ranges.clone());
		}
		cubes = new_cubes;
	}
	cubes.iter()
		.map(|c| cube_size(&c))
		.sum::<i64>()
		.try_into()
		.unwrap()
}
