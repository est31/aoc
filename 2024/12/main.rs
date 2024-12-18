use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let grd = parse(INPUT);
	println!("prices fence: {}", prices(&grd));
	println!("prices sides: {}", prices_sides(&grd));
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

fn parse(s: &str) -> Vec<Vec<char>> {
	let s = s.trim();
	s.lines()
		.map(|l| {
			l.trim().chars().collect::<Vec<_>>()
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

struct Regions<I: Copy + Clone + PartialEq + Eq + Hash + Display + Debug> {
	regions: HashMap::<char, Vec<HashSet<(I, I)>>>,
}

impl<I: Copy + Clone + PartialEq + Eq + Hash + Display + Debug> Regions<I> {
	fn new() -> Self {
		Self {
			regions: HashMap::new(),
		}
	}

	fn add(&mut self, ch :char, p :(I, I), ngh :Vec<(I, I)>) {
		let regions_with_char = self.regions.entry(ch)
			.or_default();
		let mut eligible = HashSet::new();
		for (rid, region) in regions_with_char.iter().enumerate() {
			for n in ngh.iter() {
				if region.contains(n) {
					eligible.insert(rid);
				}
			}
		}
		dprint!("  ch: {ch}, pos: {p:?}, found: {:?}, eligible: {eligible:?}", regions_with_char);
		if eligible.is_empty() {
			// New region
			regions_with_char.push(std::iter::once(p).collect());
			dprint!(" -> new region\n");
			return;
		}
		let mut eligible = eligible.into_iter().collect::<Vec<_>>();
		if let &[rid] = &eligible.as_slice() {
			// Only one region
			regions_with_char[*rid].insert(p);
			dprint!(" -> add to region\n");
			return;
		}
		dprint!("\n");
		// General case: > 1 regions
		eligible.sort();
		while let Some(rid) = eligible.pop() {
			let Some(rid_tgt) = eligible.last() else {
				regions_with_char[rid].insert(p);
				break;
			};
			assert!(*rid_tgt < rid);
			let region = regions_with_char.remove(rid);
			regions_with_char[*rid_tgt].extend(region.into_iter());
			dprint!("   -> merge {rid:?} into {rid_tgt:?}: {:?}\n", regions_with_char[*rid_tgt]);
		}
	}
	fn regions(self) -> Vec<(char, HashSet<(I, I)>)> {
		self.regions.into_iter()
			.map(|(ch, regions)| {
				regions.into_iter()
					.map(move |region| (ch, region))
			})
			.flatten()
			.collect()
	}
}

fn regions(garden :&[Vec<char>]) -> Vec<(char, HashSet<(usize, usize)>)> {
	let mut regions = Regions::new();
	let height = garden.len();
	for (y, l) in garden.iter().enumerate() {
		let width = l.len();
		for (x, ch) in l.iter().enumerate() {
			let p = (y, x);
			regions.add(*ch, p, neighs(p, height, width));
		}
	}

	regions.regions()
}

#[inline(always)]
fn neighs_fences(p: (usize, usize), height :usize, width :usize) -> Vec<(char, (usize, usize))> {
	let mut neighs = Vec::with_capacity(4);
	if p.0 > 0 {
		neighs.push(('_', (p.0 - 1, p.1)));
	}
	if p.1 > 0 {
		neighs.push((']', (p.0, p.1 - 1)));
	}
	if p.0 + 1 < height {
		neighs.push(('-', (p.0 + 1, p.1)));
	}
	if p.1 + 1 < width {
		neighs.push(('[', (p.0, p.1 + 1)));
	}
	neighs
}

fn fence_for_region(positions :&HashSet<(usize, usize)>) -> Vec<(char, (isize, isize))> {
	positions.iter()
		.map(|p| {
			let mut fences = neighs_fences(*p, usize::MAX, usize::MAX)
				.into_iter()
				.filter(|(_ch, np)| !positions.contains(np))
				.map(|(ch, np)| {
					(ch, (np.0 as isize, np.1 as isize))
				})
				.collect::<Vec<_>>();
			if p.0 == 0 {
				fences.push(('_', (-1, p.1 as isize)));
			}
			if p.1 == 0 {
				fences.push((']', (p.0 as isize, -1)));
			}
			fences.into_iter()
		})
		.flatten()
		.collect::<Vec<_>>()
}

fn price_of_region(ch: char, positions :&HashSet<(usize, usize)>) -> u32 {
	dprint!("region '{ch}', positions: {: <2}", positions.len());
	let fence_length = fence_for_region(positions).len() as u32;
	let price = fence_length * positions.len() as u32;
	dprint!("  fence: {fence_length: <3}, area: {: <3} -> price: {price} \n", positions.len());
	price
}

fn prices(garden :&[Vec<char>]) -> u32 {
	let regions = regions(garden);
	regions.iter()
		.map(|(ch, region)| price_of_region(*ch, region))
		.sum()
}

fn price_of_region_sides(ch: char, positions :&HashSet<(usize, usize)>) -> u32 {
	dprint!("region '{ch}', positions: {: <2}\n", positions.len());
	let fenced = fence_for_region(positions)
		.into_iter()
		.collect::<HashSet<_>>();
	let mut sides = Regions::new();
	for (ch, p) in fenced.iter() {
		let p = *p;
		let neighs = match *ch {
			'_' | '-' => vec![(p.0, p.1 - 1), (p.0, p.1 + 1)],
			']' | '[' => vec![(p.0 - 1, p.1), (p.0 + 1, p.1)],
			_ => panic!(),
		};
		sides.add(*ch, p, neighs);
	}
	let sides = sides.regions();
		dprint!(" sides in region {ch:?}:\n");
	for (ch, side) in sides.iter() {
		dprint!("      -> ch: {ch}, sides: {side:?}\n");
	}
	let sides_count = sides.len() as u32;
	let price = sides_count * positions.len() as u32;
	dprint!("    -> sides: {sides_count: <3}, area: {: <3} -> price: {price} \n", positions.len());
	price
}

fn prices_sides(garden :&[Vec<char>]) -> u32 {
	let regions = regions(garden);
	regions.iter()
		.map(|(ch, region)| price_of_region_sides(*ch, region))
		.sum()
}
