use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	let shiny_gold_id = get_shiny_gold_id(&lines.0).unwrap();
	let roots = search_roots(shiny_gold_id, &lines.1);
	println!("Roots for shiny gold: {roots}");
	let children = search_children(shiny_gold_id, &lines.1);
	println!("Contained by shiny gold: {children}");
}

type BagId = usize;

fn parse(input :&str) -> (HashMap<String, BagId>, Vec<(String, Vec<(u64, BagId)>)>) {
	let mut interner = HashMap::new();
	// First pass: populate the interner with the names.
	// We do this so that interned names correspond with offsets in the vec we return
	for line in input.lines() {
		let mut words = line.split_whitespace();
		let name = parse_pair(&mut words).expect("name not found");
		let l = interner.len();
		let _id = *interner.entry(name).or_insert(l);
	}
	let bags = input.lines()
		.map(|line| {
			let mut words = line.split_whitespace();
			let name = parse_pair(&mut words).expect("name not found");

			_ = words.next().unwrap(); // "bags"
			_ = words.next().unwrap(); // "contain"
			let mut contained = Vec::new();
			loop {
				let Some(count) = words.next() else {
					break
				};
				let Some(name) = parse_pair(&mut words) else {
					break
				};
				if name == "other bags." {
					break;
				}
				let count = u64::from_str(count).unwrap();
				let l = interner.len();
				let id = *interner.entry(name).or_insert(l);
				contained.push((count, id));
				let Some(_) = words.next() else { // "bags"
					break
				};
			}
			(name, contained)
		})
		.collect::<Vec<_>>();
	//println!("{interner:?}");
	//println!("{bags:?}");
	(interner, bags)
}

fn get_shiny_gold_id(interner :&HashMap<String, BagId>) -> Option<usize> {
	interner.get("shiny gold").copied()
}

fn search_roots(shiny_gold_id :usize, bag_lines :&[(String, Vec<(u64, BagId)>)]) -> usize {
	let mut rev_adj = HashMap::<usize, Vec<_>>::new();
	for (i, (_name, children)) in bag_lines.iter().enumerate() {
		for (_, child_id) in children.iter() {
			rev_adj.entry(*child_id).or_default().push(i);
		}
	}
	let mut set_of_roots = HashSet::new();
	let mut to_handle = HashSet::new();
	to_handle.insert(shiny_gold_id);
	while let Some(r_id) = to_handle.iter().next().copied() {
		to_handle.remove(&r_id);
		set_of_roots.insert(r_id);
		let Some(r_adj) = rev_adj.get(&r_id) else {
			continue
		};
		to_handle.extend(r_adj
			.iter()
			.filter(|r| !set_of_roots.contains(r))
			.copied());
	}
	set_of_roots.len() - 1
}

fn search_children(shiny_gold_id :usize, bag_lines :&[(String, Vec<(u64, BagId)>)]) -> u64 {
	let mut rev_adj = HashMap::<usize, Vec<_>>::new();
	for (i, (_name, children)) in bag_lines.iter().enumerate() {
		for (cnt, child_id) in children.iter() {
			rev_adj.entry(*child_id).or_default().push((cnt, i));
		}
	}

	let mut num_of_children = bag_lines.iter()
		.enumerate()
		.map(|(id, (_n, children))| (id, (0, children.len())))
		.collect::<HashMap<BagId, (u64, usize)>>();
	let mut to_handle = HashSet::new();
	to_handle.extend(bag_lines.iter()
		.enumerate()
		.filter(|(_, (_n, children))| children.is_empty())
		.map(|(i, _)| i));
	while let Some(r_id) = to_handle.iter().next().copied() {
		to_handle.remove(&r_id);
		let r_cnt = num_of_children[&r_id].0;
		let Some(r_adj) = rev_adj.get(&r_id) else {
			continue
		};
		for (cnt, r_id) in r_adj.iter() {
			let noc = num_of_children.get_mut(r_id).unwrap();
			noc.0 += **cnt * (1 + r_cnt);
			noc.1 = noc.1.checked_sub(1).unwrap();
			if noc.1 == 0 {
				to_handle.insert(*r_id);
			}
		}
	}
	assert_eq!(num_of_children[&shiny_gold_id].1, 0);
	num_of_children[&shiny_gold_id].0
}

fn parse_pair<'a>(mut it :impl Iterator<Item = &'a str>) -> Option<String> {
	Some(format!("{} {}", it.next()?, it.next()?))
}
