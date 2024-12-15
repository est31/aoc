use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let map = parse(INPUT);
	println!("trailhead sum: {}", trailhead_sum(&map));
	println!("trailhead ratings: {}", trailhead_ratings(&map));
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
	trailhead_sums(fields).0
}

fn trailhead_ratings(fields :&[Vec<u8>]) -> u32 {
	trailhead_sums(fields).1
}

fn trailhead_sums(fields :&[Vec<u8>]) -> (u32, u32) {
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
	let mut paths_uphill = HashMap::<(usize, usize), (HashSet<(usize, usize)>, u32)>::new();

	let mut trailhead_sum = 0;
	let mut trailhead_ratings = 0;
	while let Some((v, p)) = to_search.pop_last() {
		dprint!("v={v} p={p:?} ");
		let to_add = if v == 9 {
			(HashSet::from([p]), 1)
		} else {
			paths_uphill[&p].clone()
		};
		dprint!("to_add={to_add:?} ");
		if v == 0 {
			// trailhead
			trailhead_sum += to_add.0.len();
			trailhead_ratings += to_add.1;
			dprint!("-> HEAD ta={} inc={} SUM NOW {trailhead_sum}\n", to_add.0.len(), to_add.1);
			continue;
		}
		// iter over neighbours, only descend
		for neigh in neighs(p, height, width) {
			let neigh_v = fields[neigh.0][neigh.1];
			if neigh_v != v - 1 {
				continue;
			}
			dprint!(" {neigh:?}");
			let neigh_uphills = paths_uphill.entry(neigh)
				.or_default();
			neigh_uphills.0.extend(&to_add.0);
			neigh_uphills.1 += to_add.1;
			to_search.insert((neigh_v, neigh));
		}
		dprint!("\n");
	}

	(trailhead_sum as u32, trailhead_ratings)
}
