use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let scanners = parse_scanners(INPUT);
	let triples_map = build_triples_map(&scanners);
	let (scanner_positions, beacons_map) = find_beacons_scanners(&scanners, &triples_map);
	println!("Beacons count: {}", beacons_map.len());
	println!("Largest scanner distance: {}", largest_dist(&scanner_positions));
}

fn parse_scanners(input :&str) -> Vec<Vec<[i32; 3]>> {
	let mut scanners = Vec::new();
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut cur_scanner = Vec::new();
	let mut start = true;
	for l in lines {
		if l.starts_with("---") {
			// Start new scanner
			if !start {
				scanners.push(std::mem::take(&mut cur_scanner));
			}
			start = false;
			continue;
		}
		let mut nums = l.split(',')
			.map(|v| i32::from_str(v.trim()).unwrap());
		cur_scanner.push([nums.next().unwrap(),
			nums.next().unwrap(), nums.next().unwrap()]);
	}
	if !start {
		scanners.push(std::mem::take(&mut cur_scanner));
	}
	scanners
}

/// (|| a, b ||_2) ^ 2
fn dist(a :[i32; 3], b :[i32; 3]) -> i32 {
	a.iter().zip(b.iter())
		.map(|(ac, bc)| (ac - bc) * (ac - bc))
		.sum()
}

/// || a, b ||_1
fn dist_l1(a :[i32; 3], b :[i32; 3]) -> i32 {
	a.iter().zip(b.iter())
		.map(|(ac, bc)| (ac - bc).abs())
		.sum()
}

/// a - b
fn diff(a :[i32; 3], b :[i32; 3]) -> [i32; 3] {
	[a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// a + b
fn add(a :[i32; 3], b :[i32; 3]) -> [i32; 3] {
	[a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn point_distances([a, b, c] :[[i32; 3]; 3]) -> [i32; 3] {
	[dist(a, b), dist(b, c), dist(a, c)]
}

type TriplesMap = HashMap<[i32; 3], Vec<(usize, [usize; 3])>>;

fn build_triples_map(scanners :&[Vec<[i32; 3]>]) -> TriplesMap {
	let mut triples_map = HashMap::new();
	for (sci, sc) in scanners.iter().enumerate() {
		for (bci_1, bc_1) in sc.iter().enumerate() {
			for (bci_2, bc_2) in sc[0..bci_1].iter().enumerate() {
				for (bci_3, bc_3) in sc[0..bci_2].iter().enumerate() {
					let mut distances = point_distances([*bc_1, *bc_2, *bc_3]);
					distances.sort();
					triples_map.entry(distances)
						.or_insert(Vec::new())
						.push((sci, [bci_1, bci_2, bci_3]));
				}
			}
		}
	}
	triples_map
}

macro_rules! dprintln {
	($($args:expr),*) => {
		//println!($($args),*);
	};
}

fn find_beacons_scanners(scanners :&[Vec<[i32; 3]>], triples_map :&TriplesMap) -> (Vec<[i32; 3]>, HashSet<[i32; 3]>) {
	let mut scanner_neigh_pairs = HashMap::<[usize; 2], Vec<[&(usize, [usize; 3]); 2]>>::new();
	for (_d, trs) in triples_map.iter() {
		// We are only interested in common triples
		if trs.len() == 1 {
			continue;
		}
		for (tri_1, tr_1) in trs.iter().enumerate() {
			for (_tri_2, tr_2) in trs[0..tri_1].iter().enumerate() {
				if tr_1.0 == tr_2.0 {
					// Discard matching triple pairs in the same scanner region
					continue;
				}
				let mut trs = [tr_1, tr_2];
				trs.sort_by_key(|tr| tr.0);
				scanner_neigh_pairs.entry(trs.map(|tr| tr.0))
					.or_insert(Vec::new())
					.push(trs);
			}
		}
	}
	let mut scanner_neighs = vec![Vec::new(); scanners.len()];
	for ([sci_1, sci_2], neigh) in scanner_neigh_pairs.iter() {
		if neigh.len() > 1 {
			dprintln!("    pair {} <-> {} => {}", sci_1, sci_2, neigh.len());
		}
		// We are only interested in scanner pairs with
		// enough common triples. The document writes about
		// 12 common ones, which means there should be
		// binomial(12, 3) = 220 common pairs.
		// SOMETIMES the number can be even larger.
		// E.g. when you have multiple (>2) triples with the same
		// distance triple, then the code above will add each
		// 2-triple combination as a common triple, leading to
		// overcounting if two of the triples belong to
		// the same scanner. That's why we limit from above only
		// instead of having a precise limit. In theory this effect
		// could also bloat up wrong scanner pairs, but this is
		// highly unlikely.
		const LIMIT :usize = 220;
		if neigh.len() < LIMIT {
			continue;
		}
		scanner_neighs[*sci_1].push((sci_2, neigh.clone()));
		let mut neigh_rev = neigh.clone();
		neigh_rev.iter_mut().for_each(|t| t.reverse());
		scanner_neighs[*sci_2].push((sci_1, neigh_rev));
		dprintln!("Scanner pair {} <-> {} has neighbours", sci_1, sci_2);
	}

	let oris = Orientation::orientations();

	// Build the final scanner map
	let mut scanners_visited = vec![false; scanners.len()];
	let mut scanners_to_visit = vec![(0, [0, 0, 0], Orientation::null())];

	let mut beacons = HashSet::new();
	let mut scanner_positions = vec![[0, 0, 0]; scanners.len()];
	while let Some((sc_i, sc_pos, sc_ori)) = scanners_to_visit.pop() {
		dprintln!("Visit scanner {}", sc_i);
		scanner_positions[sc_i] = sc_pos;
		scanners_visited[sc_i] = true;
		let sc_1 = &scanners[sc_i];

		for bc in sc_1 {
			let bc_absolute = add(sc_ori.apply(*bc), sc_pos);
			beacons.insert(bc_absolute);
		}

		for (&neigh_i, neigh) in &scanner_neighs[sc_i] {
			if scanners_visited[neigh_i] {
				continue;
			}
			let mut pos_ori_votes = HashMap::<([i32; 3], Orientation), usize>::new();
			// Determine the neighbours position and orientation
			for [(_, tri_1), (_, tri_2)] in neigh {
				let sc_2 = &scanners[neigh_i];
				let tr_1 = [sc_1[tri_1[0]], sc_1[tri_1[1]], sc_1[tri_1[2]]];
				let tr_2 = [sc_2[tri_2[0]], sc_2[tri_2[1]], sc_2[tri_2[2]]];
				let (_, tr_2_ordered) = if let Some(o) = align_order(tr_1, tr_2) {
					o
				} else {
					// We can't find a unique alginment.
					// Discard this pair for the voting.
					continue;
				};
				for (bc_1, bc_2) in tr_1.iter().zip(tr_2_ordered.iter()) {
					let bc_1_absolute = add(sc_ori.apply(*bc_1), sc_pos);
					for ori in oris.iter() {
						let neigh_pos_absolute = diff(bc_1_absolute, ori.apply(*bc_2));
						*pos_ori_votes.entry((neigh_pos_absolute, *ori))
							.or_insert(0) += 1;
					}
				}
			}
			let ((pos, ori), _votes) = pos_ori_votes.iter()
				.max_by_key(|((_pos, _ori), votes)| *votes)
				.unwrap();
			scanners_to_visit.push((neigh_i, *pos, *ori));
		}
	}
	let scanners_visited_num = scanners_visited.iter().filter(|v| **v).count();
	if scanners_visited_num < scanners.len() {
		panic!("Only visited {} scanners out of {}", scanners_visited_num, scanners.len());
	}
	(scanner_positions, beacons)
}

fn align_order(v1 :[[i32; 3]; 3], v2 :[[i32; 3]; 3]) -> Option<(usize, [[i32; 3]; 3])> {
	let dist_v1 = point_distances(v1);
	let permutations = (0..PERMUTATIONS.len())
		.filter(|&p| point_distances(apply_permutation(v2, p)) == dist_v1)
		.collect::<Vec<_>>();
	if permutations.len() != 1 {
		// Multiple possible permutations. Abort.
		return None;
	}
	let p = permutations[0];
	Some((p, apply_permutation(v2, p)))
}

/*fn possible_permutations(v1 :[[i32; 3]; 3], v2 :[[i32; 3]; 3]) -> Vec<(usize, [[i32; 3]; 3])> {
	let dist_v1 = point_distances(v1);
	(0..PERMUTATIONS.len())
		.map(|p| (p, apply_permutation(v2, p)))
		.filter(|(_p, vp)| point_distances(*vp) == dist_v1)
		.collect::<Vec<_>>()
}*/

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Orientation {
	coord_multipliers :[i32; 3],
	coord_permutation :usize,
}

const PERMUTATIONS :[[usize; 3]; 6] = [
	[0, 1, 2],
	[0, 2, 1],
	[2, 0, 1],
	[1, 0, 2],
	[1, 2, 0],
	[2, 1, 0],
];

fn apply_permutation<T :Copy>(v :[T; 3], p :usize) -> [T; 3] {
	let p = PERMUTATIONS[p];
	[v[p[0]], v[p[1]], v[p[2]]]
}

impl Orientation {
	fn orientations() -> Vec<Self> {
		let mut res = Vec::new();
		for coord_permutation in 0..PERMUTATIONS.len() {
			for mx in [true, false] {
				for my in [true, false] {
					for mz in [true, false] {
						fn f(v :bool) -> i32 {
							((v as i32) * 2) - 1
						}
						res.push(Orientation {
							coord_multipliers : [mx , my, mz].map(f),
							coord_permutation,
						});
					}
				}
			}
		}
		res
	}
	fn null() -> Self {
		Orientation {
			coord_multipliers : [1, 1, 1],
			coord_permutation : 0,
		}
	}
	fn apply(&self, coords :[i32; 3]) -> [i32; 3] {
		let mut c = coords;
		for (c, m) in c.iter_mut().zip(self.coord_multipliers.iter()) {
			*c *= m;
		}
		apply_permutation(c, self.coord_permutation)
	}
}

fn largest_dist(scanner_positions :&[[i32; 3]]) -> i32 {
	scanner_positions.iter()
		.map(|sc| scanner_positions.iter()
			.map(|scb| dist_l1(*sc, *scb))
			.max()
			.unwrap())
		.max()
		.unwrap()
}
