use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let components = parse(INPUT);
	println!("results sum: {}", sum_hashes(&components));
	println!("focusing power: {}", focusing_power(&components));
}

fn parse(input :&str) -> Vec<String> {
	input.split(',')
		.map(|c| c.trim().to_owned())
		.collect::<Vec<_>>()
}

fn hash(s :&str) -> u32 {
	let mut v = 0;
	for ch in s.chars() {
		v += ch as u32;
		v *= 17;
		v %= 256;
	}
	v
}

fn sum_hashes(components :&[String]) -> u32 {
	components.iter()
		.map(|s| hash(s))
		.sum::<u32>()
}

enum Operation<'a> {
	Remove(&'a str),
	Insert(&'a str, u8),
}

fn get_operations(components :&[String]) -> Vec<Operation<'_>> {
	components.iter()
		.map(|cmp| {
			if cmp.contains('-') {
				let label = cmp.split('-').next().unwrap();
				Operation::Remove(label)
			} else if cmp.contains('=') {
				let mut cmps = cmp.split('=');
				let label = cmps.next().unwrap();
				let focal_len_str = cmps.next().unwrap();
				let focal_len = u8::from_str(focal_len_str).unwrap();
				Operation::Insert(label, focal_len)
			} else {
				panic!("Either '-' or '=' required in component '{cmp}'");
			}
		})
		.collect::<Vec<_>>()
}

fn focusing_power(components :&[String]) -> u64 {
	let operations = get_operations(components);
	let mut boxes :[Vec<(&str, u8)>; 256] = core::array::from_fn(|_| Vec::new());
	for op in operations {
		match op {
			Operation::Remove(lbl) => {
				let box_idx = hash(lbl);
				boxes[box_idx as usize].retain(|lns| lns.0 != lbl);
			}
			Operation::Insert(lbl, fcl_len) => {
				let box_idx = hash(lbl);
				let box_ = &mut boxes[box_idx as usize];
				if let Some(lns_ref) = box_.iter_mut().find(|lns| lns.0 == lbl) {
					lns_ref.1 = fcl_len;
				} else {
					box_.push((lbl, fcl_len));
				}
			}
		}

	}
	let total_focusing_power = boxes.iter()
		.enumerate()
		.map(|(bidx, lenses)| {
			let bidx = bidx as u64 + 1;
			lenses.iter()
				.enumerate()
				.map(|(lns_idx, (_lbl, fcl_len))| {
					bidx * (lns_idx as u64 + 1) * *fcl_len as u64
				})
				.sum::<u64>()
		})
		.sum::<u64>();
	total_focusing_power
}
