use std::str::FromStr;
use std::ops::{RangeInclusive, Range};
use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let commands = parse_commands(INPUT);
	println!("Enabled after commands in initialization region: {}", run_commands_simple(&commands).len());
	println!("Enabled after commands: {}", run_commands(&commands));
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

fn compress_indices(cmds :&[(bool, Cube)], cmds_compressed :&mut [(bool, [Range<usize>; 3])], d: usize) -> Vec<i32> {
	let mut hm = HashMap::<i32, Vec<(usize, bool)>>::new();
	let it = cmds.iter()
		.enumerate()
		.map(|(i, (_, c))| [((i, true), *c[d].start()), ((i, false), *c[d].end() + 1)].into_iter())
		.flatten();
	for (i_b, v) in it {
		hm.entry(v).or_default().push(i_b);
	}
	let mut indices_c = hm.into_iter()
		.collect::<Vec<_>>();
	indices_c.sort();
	let r = indices_c.into_iter().enumerate()
		.map(|(idx_i, (v, l))| {
			for (i, b) in l {
				let r = &mut cmds_compressed[i].1[d];
				if b {
					*r = idx_i..r.end;
				} else {
					*r = (r.start)..idx_i;
				}
			}
			v
		})
		.collect::<Vec<_>>();
	//println!("{r:?}");
	r
}

fn run_commands(cmds :&[(bool, Cube)]) -> u64 {
	let mut cmds_compressed = cmds.iter()
		.map(|(b, _)| (*b, [0..0, 0..0, 0..0]))
		.collect::<Vec<_>>();
	let indices_x = compress_indices(cmds, &mut cmds_compressed, 0);
	let indices_y = compress_indices(cmds, &mut cmds_compressed, 1);
	let indices_z = compress_indices(cmds, &mut cmds_compressed, 2);

	//println!("cmds compressed: {cmds_compressed:?}");
	let mut enabled = HashSet::new();
	for (cmd, ranges) in cmds_compressed.iter() {
		for x in ranges[0].clone() {
			for y in ranges[1].clone() {
				for z in ranges[2].clone() {
					if *cmd {
						enabled.insert((x, y, z));
					} else {
						enabled.remove(&(x, y, z));
					}
				}
			}
		}
	}
	//println!("Number of enabled compressed regions: {}", enabled.len());
	enabled.into_iter()
		.map(|(x, y, z)| {
			let dx = indices_x[x + 1] - indices_x[x];
			let dy = indices_y[y + 1] - indices_y[y];
			let dz = indices_z[z + 1] - indices_z[z];
			let p = (dx as i64) * (dy as i64) * (dz as i64);
			//println!("({x}, {y}, {z}) -> {p}");
			p
		})
		.sum::<i64>() as _
}
