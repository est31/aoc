use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let map = parse(INPUT);
	println!("trailhead sum: {}", trailhead_sum(&map));
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

fn parse(s: &str) -> Vec<Vec<u8>> {
	let s = s.trim();
	s.lines()
		.map(|l| {
			l.trim().chars()
				.map(|ch| {
					if ch == '.' {
						return 255;
					}
					if !('0'..='9').contains(&ch) {
						panic!("Not a number: '{ch}'");
					}
					ch as u8 - '0' as u8
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

#[inline(always)]
fn neighs(p: (usize, usize), height :usize, width :usize) -> Vec<(usize, usize)> {
	let mut neighs = Vec::with_capacity(4);
	if p.0 > 0 {
		neighs.push((p.0 - 1, p.1));
	}
	if p.1 > 0 {
		neighs.push((p.0, p.1 - 1));
	}
	if p.0 + 1 < height {
		neighs.push((p.0 + 1, p.1));
	}
	if p.1 + 1 < width {
		neighs.push((p.0, p.1 + 1));
	}
	neighs
}

fn trailhead_sum(fields :&[Vec<u8>]) -> u32 {
	let height = fields.len();
	let width = fields[0].len();

	let mut trailheads = Vec::new();
	let mut ends = Vec::new();
	for (y, line) in fields.iter().enumerate() {
		for (x, fld) in line.iter().enumerate() {
			let p = (y, x);
			if *fld == 9 {
				ends.push(p);
			}
			if *fld == 0 {
				trailheads.push(p);
			}
		}
	}

	let mut to_search = ends.clone()
		.into_iter()
		.map(|p| (9, p))
		.collect::<BTreeSet<_>>();
	let mut paths_uphill = HashMap::<(usize, usize), HashSet<(usize, usize)>>::new();

	let mut trailhead_sum = 0;
	while let Some((v, p)) = to_search.pop_last() {
		dprint!("v={v} p={p:?} ");
		let to_add = if v == 9 {
			HashSet::from([p])
		} else {
			paths_uphill[&p].clone()
		};
		dprint!("to_add={to_add:?} ");
		if v == 0 {
			// trailhead
			trailhead_sum += to_add.len();
			dprint!("-> HEAD inc={} SUM NOW {trailhead_sum}\n", to_add.len());
			continue;
		}
		// iter over neighbours, only descend
		for neigh in neighs(p, height, width) {
			let neigh_v = fields[neigh.0][neigh.1];
			if neigh_v != v - 1 {
				continue;
			}
			dprint!(" {neigh:?}");
			paths_uphill.entry(neigh)
				.or_default()
				.extend(&to_add);
			to_search.insert((neigh_v, neigh));
		}
		dprint!("\n");
	}

	trailhead_sum as u32
}
