use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let groups = parse(INPUT);
	let mgs = max_group_sum(&groups);
	println!("max group sum: {mgs}");
	let m3gs = max_three_groups_sum(&groups);
	println!("max three groups sum: {m3gs}");
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

fn max_three_groups_sum(groups :&[Vec<u32>]) -> u32 {
	let mut group_sums = groups.iter()
		.map(|g| g.iter().sum())
		.collect::<Vec<u32>>();
	group_sums.sort();
	let l = group_sums.len();
	let sum = group_sums[l - 1] + group_sums[l - 2] + group_sums[l - 3];
	sum
}
