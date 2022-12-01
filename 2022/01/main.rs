use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let groups = parse(INPUT);
	let mgs = max_group_sum(&groups);
	println!("max group sum: {mgs}");
}

fn parse(input :&str) -> Vec<Vec<u32>> {
	let mut res = Vec::new();
	let mut cur = Vec::new();
	for line in input.lines() {
		let line = line.trim();
		if line.is_empty() {
			res.push(std::mem::take(&mut cur));
			continue;
		}
		cur.push(u32::from_str(line).unwrap());
	}
	if !cur.is_empty() {
		res.push(std::mem::take(&mut cur));
	}
	res
}

fn max_group_sum(groups :&[Vec<u32>]) -> u32 {
	groups.iter()
		.map(|g| g.iter().sum())
		.max()
		.unwrap()
}
