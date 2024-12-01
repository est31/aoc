use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (left, right) = parse_vecs(INPUT);
	println!("sum of diffs: {}", sum_of_diffs(&left, &right));
}

fn parse_vecs(s: &str) -> (Vec<u32>, Vec<u32>) {
	let mut left = Vec::new();
	let mut right = Vec::new();
	for line in s.lines() {
		let mut components = line.split("   ");
		let l = components.next().unwrap();
		let r = components.next().unwrap();
		left.push(u32::from_str(l).unwrap());
		right.push(u32::from_str(r).unwrap());
	}
	left.sort();
	right.sort();
	(left, right)
}

fn sum_of_diffs(left: &[u32], right: &[u32]) -> u32 {
	left.iter()
		.zip(right.iter())
		.map(|(l, r)| if *l > *r {
			*l - *r
		} else {
			*r - *l
		})
		.sum()
}
