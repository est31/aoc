use std::collections::{HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let map = parse(INPUT);
	println!("unique count with antinode: {}", unique_with_antinode(&map));
}

fn parse(s: &str) -> Map {
	let mut height = 0;
	let mut width = 0;
	let mut antennas = HashMap::<_, Vec<_>>::new();
	for (i, l) in s.lines().enumerate() {
		height += 1;
		width = l.len() as u32;
		for (j, ch) in l.chars().enumerate() {
			if ch == '.' {
				continue;
			}
			let for_ch = antennas.entry(ch)
				.or_default();
			for_ch.push((i, j));
		}
	}
	Map {
		height,
		width,
		antennas,
	}
}

struct Map {
	height: u32,
	width: u32,
	antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
	fn pos_in_bounds(&self, p0: i32, p1: i32) -> bool {
		(0..self.height as i32).contains(&p0) &&
		(0..self.width as i32).contains(&p1)
	}
}

fn unique_with_antinode(map: &Map) -> u32 {
	let mut with_antinode = HashSet::new();

	let maybe_add = |with_antinode :&mut HashSet<_>, an_y, an_x| {
		if map.pos_in_bounds(an_y, an_x) {
			with_antinode.insert((an_y as u32, an_x as u32));
		}
	};
	for (_ch, antennas) in map.antennas.iter() {
		for (i, a0) in antennas.iter().enumerate() {
			if i + 1 == antennas.len() {
				continue;
			}
			for a1 in antennas[i + 1..].iter() {
				let y_diff = a1.0 as i32 - a0.0 as i32;
				let x_diff = a1.1 as i32 - a0.1 as i32;

				let an_0_y = a0.0 as i32 - y_diff;
				let an_0_x = a0.1 as i32 - x_diff;
				maybe_add(&mut with_antinode, an_0_y, an_0_x);

				let an_1_y = a1.0 as i32 + y_diff;
				let an_1_x = a1.1 as i32 + x_diff;
				maybe_add(&mut with_antinode, an_1_y, an_1_x);
			}
		}
	}

	with_antinode.len() as u32
}
