use std::collections::{HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let grd = parse(INPUT);
	println!("price sum: {}", prices(&grd));
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

fn regions(garden :&[Vec<char>]) -> Vec<(char, HashSet<(usize, usize)>)> {
	let mut regions = HashMap::<char, Vec<HashSet<(usize, usize)>>>::new();
	let height = garden.len();
	for (y, l) in garden.iter().enumerate() {
		let width = l.len();
		for (x, ch) in l.iter().enumerate() {
			let regions_with_char = regions.entry(*ch)
				.or_default();
			let p = (y, x);
			let mut eligible = HashSet::new();
			for (rid, region) in regions_with_char.iter().enumerate() {
				for n in neighs(p, height, width) {
					if region.contains(&n) {
						eligible.insert(rid);
					}
				}
			}
			dprint!("ch: {ch}, pos: {p:?}, found: {regions_with_char:?}, eligible: {eligible:?}");
			if eligible.is_empty() {
				// New region
				regions_with_char.push(std::iter::once(p).collect());
				dprint!(" -> new region\n");
				continue;
			}
			let mut eligible = eligible.into_iter().collect::<Vec<_>>();
			if let &[rid] = &eligible.as_slice() {
				// Only one region
				regions_with_char[*rid].insert(p);
				dprint!(" -> add to region\n");
				continue;
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
			dprint!("\n");
		}
	}

	regions.into_iter()
		.map(|(ch, regions)| {
			regions.into_iter()
				.map(move |region| (ch, region))
		})
		.flatten()
		.collect()
}

fn price_of_region(ch: char, positions :&HashSet<(usize, usize)>) -> u32 {
	dprint!("region {ch:?}, positions: {: <2}", positions.len());
	let fence_length = positions.iter()
		.map(|p| {
			let mut fences = neighs(*p, usize::MAX, usize::MAX)
				.into_iter()
				.filter(|np| !positions.contains(np))
				.count();
			if p.0 == 0 {
				fences += 1;
			}
			if p.1 == 0 {
				fences += 1;
			}
			fences as u32
		})
		.sum::<u32>();
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
