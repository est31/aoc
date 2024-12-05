use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut ru = parse(INPUT);
	println!("updates sum: {}", updates_sum(&ru));
	println!("updates sum ordered: {}", updates_sum_ordered(&mut ru));
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

struct RulesUpdates {
	rules: HashMap<u16, HashSet<u16>>,
	updates: Vec<Vec<u16>>,
}

fn parse(s: &str) -> RulesUpdates {
	let mut lines = s.lines();

	let mut rules = HashMap::<_, HashSet<_>>::new();
	while let Some(line) = lines.next() {
		if line.is_empty() {
			break;
		}
		let mut components = line.split("|");
		let first = components.next().unwrap();
		let second = components.next().unwrap();
		let first = u16::from_str(first).unwrap();
		let second = u16::from_str(second).unwrap();
		rules.entry(first).or_default().insert(second);
	}
	let mut updates = Vec::new();
	while let Some(line) = lines.next() {
		let update = line.split(",")
			.map(|c| u16::from_str(c).unwrap())
			.collect::<Vec<_>>();
		updates.push(update);
	}
	RulesUpdates {
		rules,
		updates,
	}
}

fn updates_sum(ru :&RulesUpdates) -> u32 {
	ru.updates.iter()
		.filter(|update| {
			for (i, v) in update.iter().enumerate() {
				if i + 1 == update.len() {
					break;
				}
				for w in update[i + 1..].iter() {
					let Some(w_rule) = ru.rules.get(&w) else { continue };
					if w_rule.contains(v) {
						dprint!("  {w}|{v}\n");
						return false;
					}
				}
			}
			dprint!("  in order\n");
			true
		})
		.map(|update| update[update.len()/2] as u32)
		.sum()
}

fn updates_sum_ordered(ru :&mut RulesUpdates) -> u32 {
	ru.updates.iter_mut()
		.filter_map(|update| {
			let mut i = 0;
			let mut swapped = false;
			'outer: loop {
				for j in i + 1..update.len() {
					let v = update[i];
					let w = update[j];
					let Some(w_rule) = ru.rules.get(&w) else { continue };
					if w_rule.contains(&v) {
						dprint!("  {w}|{v} -> swap\n");
						update.swap(i, j);
						swapped = true;
						continue 'outer;
					}
				}
				i += 1;
				if i >= update.len() - 1 {
					break;
				}
			}
			swapped.then_some(update)
		})
		.map(|update| update[update.len()/2] as u32)
		.sum()
}
