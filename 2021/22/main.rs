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

fn run_commands(cmds :&[(bool, [RangeInclusive<i32>; 3])]) -> u64 {
	let mut cubes = HashSet::<[RangeInclusive<i32>; 3]>::new();

	// TODO: do cubes calculation

	fn siz(r :&RangeInclusive<i32>) -> u64 {
		(*r.end() - *r.start() + 1) as u64
	}
	cubes.iter()
		.map(|c| siz(&c[0]) * siz(&c[1]) * siz(&c[2]))
		.sum()
}
