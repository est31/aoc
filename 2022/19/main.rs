use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let bps = parse(INPUT);
	let s = quality_level_sum(&bps);
	println!("Quality level sum: {s}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

		if line.contains("Each") {
			let mut nums = line.split_whitespace()
				.map(u32::from_str)
				.filter_map(Result::ok);
			let ore_robot_cost = nums.next().unwrap();
			let clay_robot_cost = nums.next().unwrap();
			let obs_robot_cost = (nums.next().unwrap(), nums.next().unwrap());
			let geo_robot_cost = (nums.next().unwrap(), nums.next().unwrap());

			res.push(Blueprint {
				ore_robot_cost,
				clay_robot_cost,
				obs_robot_cost,
				geo_robot_cost,
			});

			continue;
		}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
	bp :&'a Blueprint,
	resources :[u32; 4],
	robots :[u32; 4],
	building :Option<usize>,
	time_rem :u8,
}

fn buy_robot<'a>(st :&State<'a>, kind :usize) -> Option<State<'a>> {
	let mut costs = [0; 4];
	match kind {
		0 => costs[0] = st.bp.ore_robot_cost,
		1 => costs[0] = st.bp.clay_robot_cost,
		2 => {
			costs[0] = st.bp.obs_robot_cost.0;
			costs[1] = st.bp.obs_robot_cost.1;
		},
		3 => {
			costs[0] = st.bp.geo_robot_cost.0;
			costs[2] = st.bp.geo_robot_cost.1;
		},
		_ => unreachable!(),
	}
	let mut st = *st;
	for (avail, cost) in st.resources.iter_mut().zip(costs.iter()) {
		*avail = (*avail).checked_sub(*cost)?;
	}
	st.building = Some(kind);
	Some(st)
}

fn geodes_to_open_st(mut st :State<'_>) -> u32 {
	if st.time_rem == 0 || st.time_rem == 1 || (false && st.time_rem == 2 && st.building.is_none()) {
		let ret = st.resources[3] + st.time_rem as u32 * st.robots[3];
		/*if ret >= 7 {
			println!("returning {ret}: {st:?}");
		}*/
		return ret;
	}
	st.time_rem -= 1;
	for (res, robots) in st.resources.iter_mut().zip(st.robots.iter()) {
		*res += robots;
	}
	if let Some(rb) = st.building.take() {
		st.robots[rb] += 1;
	}
	let max_when_building = (0..4)
		.filter_map(|kind| {
			let new_st = buy_robot(&st, kind)?;
			Some(geodes_to_open_st(new_st))
		})
		.max()
		.unwrap_or(0);
	let res = geodes_to_open_st(st).max(max_when_building);
	res
}

fn geodes_to_open(bp :Blueprint) -> u32 {
	let st = State {
		bp : &bp,
		resources : [0; 4],
		robots : [1, 0, 0, 0],
		building : None,
		time_rem : 24,
	};
	geodes_to_open_st(st)
}

fn quality_level_sum(bps :&[Blueprint]) -> u32 {
	bps.iter()
		.enumerate()
		.map(|(i, bp)| (i as u32 + 1) * geodes_to_open(*bp))
		.sum()
}
