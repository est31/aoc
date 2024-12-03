use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("sum of muls: {}", sum_of_muls(INPUT));
}

fn sum_of_muls(s: &str) -> u32 {
	let mut muls = Vec::new();
	let mut it = s.split("mul");
	let mut first = true;
	while let Some(component) = it.next() {
		if first {
			first = false;
			continue;
		}
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

		muls.push((first, second));
	}
	muls.iter()
		.map(|(v0, v1)| *v0 * *v1)
		.sum()
}
