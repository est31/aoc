use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let positions = parse_positions(&INPUT);
	let center_l1 = find_center_l1(&positions);
	let cost_l1 = cost_for_center_l1(&positions, center_l1);
	println!("L1 center: {}, cost: {}", center_l1, cost_l1);
	let center_l2 = find_center_l2(&positions);
	let cost_l2 = cost_for_center_l2(&positions, center_l2);
	println!("L2 center: {}, cost: {}", center_l2, cost_l2);
}

fn parse_positions(input :&str) -> Vec<i32> {
	input.trim()
		.split(',')
		.map(|n| i32::from_str(n).unwrap())
		.collect::<Vec<_>>()
}

fn find_center_l1(positions :&[i32]) -> i32 {
	find_center(positions, cost_for_center_l1)
}

fn find_center_l2(positions :&[i32]) -> i32 {
	find_center(positions, cost_for_center_l2)
}

fn find_center(positions :&[i32], cost_fn :fn(&[i32], i32) -> i32) -> i32 {
	// Why does the average not work here?
	// positions.iter().sum::<i32>() / (positions.len() as i32)
	let min_pos = *positions.iter().min().unwrap();
	let max_pos = *positions.iter().max().unwrap();
	(min_pos..=max_pos).into_iter()
		.min_by_key(|c| cost_fn(positions, *c))
		.unwrap()
}

fn cost_for_center_l1(positions :&[i32], center :i32) -> i32 {
	positions.iter().map(|p| (p - center).abs()).sum()
}

fn cost_for_center_l2(positions :&[i32], center :i32) -> i32 {
	positions.iter().map(|p| {
		let v = (p - center).abs();
		v * (v + 1) / 2
	}).sum()
}
