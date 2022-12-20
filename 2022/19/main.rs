use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let bps = parse(INPUT);
	let s = quality_level_sum(&bps);
	println!("Quality level sum: {s}");
	let p = geodes_product(&bps);
	println!("Geodes product: {p}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Blueprint {
	ore_robot_cost :u32,
	clay_robot_cost :u32,
	obs_robot_cost :(u32, u32),
	geo_robot_cost :(u32, u32),
	max_ore_cost :u32,
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

			let max_ore_cost = ore_robot_cost.max(clay_robot_cost)
					.max(obs_robot_cost.0)
					.max(geo_robot_cost.0);

			res.push(Blueprint {
				ore_robot_cost,
				clay_robot_cost,
				obs_robot_cost,
				geo_robot_cost,
				max_ore_cost,
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

		let max_ore_cost = ore_robot_cost.max(clay_robot_cost)
				.max(obs_robot_cost.0)
				.max(geo_robot_cost.0);

		res.push(Blueprint {
			ore_robot_cost,
			clay_robot_cost,
			obs_robot_cost,
			geo_robot_cost,
			max_ore_cost,
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
	if kind != 3 {
		let max_cost = match kind {
			0 => st.bp.max_ore_cost,
			1 => st.bp.obs_robot_cost.1,
			2 => st.bp.geo_robot_cost.1,
			_ => unreachable!(),
		};
		if st.robots[kind] + 1 > max_cost {
			// It makes no sense to build a robot producing this resource
			// because it would increase per-round production of that resource
			// to be larger than any other robot's building cost.
			return None;
		}
	}
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

fn upper_geode_limit(st :&State<'_>) -> u32 {
	let time_rem = st.time_rem as u32;

	let robots_geode = st.robots[3] + (st.building == Some(3)) as u32;
	let robots_obs = st.robots[2] + (st.building == Some(2)) as u32;

	// The number of geodes we have plus what we will produce until the end
	// with the current robots.
	let base = st.resources[3] + time_rem * robots_geode;

	// A simple upper limit calculation of obsidian production.
	let upper_obsidian_limit = {
		let base = robots_obs + time_rem * robots_obs;
		base + (time_rem * (time_rem + 1)) / 2
	};

	let buildable_robots_obs = upper_obsidian_limit / st.bp.geo_robot_cost.1;
	// The maximum number of geode robots we can build with the obsidian upper limit
	let buildable_robots = buildable_robots_obs.min(time_rem);
	base + buildable_robots * (buildable_robots + 1) / 2 + (time_rem - buildable_robots) * buildable_robots
}

fn geodes_to_open_st(mut st :State<'_>, cmax :&mut u32) -> u32 {
	if st.time_rem == 0 || st.time_rem == 1 || (false && st.time_rem == 2 && st.building.is_none()) {
		let ret = st.resources[3] + st.time_rem as u32 * st.robots[3];
		/*if ret >= 7 {
			println!("returning {ret}: {st:?}");
		}*/
		*cmax = (*cmax).max(ret);
		return ret;
	}
	if *cmax >= upper_geode_limit(&st) {
		return 0;
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
			Some(geodes_to_open_st(new_st, cmax))
		})
		.max()
		.unwrap_or(0);
	let res = geodes_to_open_st(st, cmax).max(max_when_building);
	*cmax = (*cmax).max(res);
	res
}

fn geodes_to_open(bp :Blueprint, time_rem :u8) -> u32 {
	let st = State {
		bp : &bp,
		resources : [0; 4],
		robots : [1, 0, 0, 0],
		building : None,
		time_rem,
	};
	geodes_to_open_st(st, &mut 0)
}

fn quality_level_sum(bps :&[Blueprint]) -> u32 {
	bps.iter()
		.enumerate()
		.map(|(i, bp)| (i as u32 + 1) * geodes_to_open(*bp, 24))
		.sum()
}

fn geodes_product(bps :&[Blueprint]) -> u32 {
	bps.iter()
		.take(3)
		.map(|bp| geodes_to_open(*bp, 32))
		.sum()
}
