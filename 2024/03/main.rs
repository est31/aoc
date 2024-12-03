use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("sum of muls: {}", sum_of_muls(INPUT, false));
	println!("sum of muls (with enablement): {}", sum_of_muls(INPUT, true));
}

fn sum_of_muls(s: &str, modes: bool) -> u32 {
	let mut components = Vec::new();

	let mut dos = s.split("do()");
	while let Some(do_component) = dos.next() {
		let mut donts = do_component.splitn(2, "don't()");
		components.push((true, donts.next().unwrap()));
		if let Some(dont) = donts.next() {
			components.push((false, dont));
		}
	}

	let mut muls = Vec::new();
	for (enabled, component) in components {
		let mut it = component.split("mul");
		// The first component has no 'mul' prefix
		it.next().unwrap();
		while let Some(component) = it.next() {
			let Some(first_n) = component.strip_prefix("(") else {
				continue
			};
			if !first_n.contains(')') {
				continue;
			}
			let nums = first_n.split(')').next().unwrap();
			let mut nums_iter = nums.split(',');
			let (Some(first), Some(second)) = (nums_iter.next(), nums_iter.next()) else {
				continue
			};
			let (Ok(first), Ok(second)) = (u32::from_str(first), u32::from_str(second)) else {
				continue
			};

			if first >= 1000 || second >= 1000 {
				continue;
			}

			if enabled || !modes {
				muls.push((first, second));
			}
		}

	}
	muls.iter()
		.map(|(v0, v1)| *v0 * *v1)
		.sum()
}
