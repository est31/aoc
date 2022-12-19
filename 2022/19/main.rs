use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let bps = parse(INPUT);
	let s = quality_level_sum(&bps);
	println!("Quality level sum: {s}");
}

#[derive(Copy, Clone, Debug)]
struct Blueprint {
	ore_robot_cost :u32,
	clay_robot_cost :u32,
	obs_robot_cost :(u32, u32),
	geo_robot_cost :(u32, u32),
}

fn parse(input :&str) -> Vec<Blueprint> {
	let mut res = Vec::new();
	let mut lines = input.lines().map(|l| l.trim());
	while let Some(line) = lines.next() {
		let line = if line.is_empty() {
			lines.next().unwrap()
		} else {
			line
		};
		assert!(line.starts_with("Blueprint"));

		let line = lines.next().unwrap();
		let mut words = line.split_whitespace();
		let num = words.nth(4).unwrap();
		let ore_robot_cost = u32::from_str(num).unwrap();

		let line = lines.next().unwrap();
		let mut words = line.split_whitespace();
		let num = words.nth(4).unwrap();
		let clay_robot_cost = u32::from_str(num).unwrap();

		let line = lines.next().unwrap();
		let mut nums = line.split_whitespace()
			.map(u32::from_str)
			.filter_map(Result::ok);
		let obs_robot_cost = (nums.next().unwrap(), nums.next().unwrap());

		let line = lines.next().unwrap();
		let mut nums = line.split_whitespace()
			.map(u32::from_str)
			.filter_map(Result::ok);
		let geo_robot_cost = (nums.next().unwrap(), nums.next().unwrap());

		res.push(Blueprint {
			ore_robot_cost,
			clay_robot_cost,
			obs_robot_cost,
			geo_robot_cost,
		});
	}
	res
}

fn quality_level_sum(bps :&[Blueprint]) -> u32 {
	todo!()
}
