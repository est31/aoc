use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sensors = parse(INPUT);
	let nba = no_beacons_at(&sensors, 2_000_000);
	println!("No beacons at 2M: {nba}");
}

fn parse(input :&str) -> Vec<((i64, i64), (i64, i64))> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			assert!(l.starts_with("Sensor at"));
			let mut words = l.split('=');
			words.next().unwrap();

			let sns_x = words.next().unwrap();
			let sns_x = sns_x.split_once(',').unwrap().0;
			let sns_x = i64::from_str(sns_x).unwrap();

			let sns_y = words.next().unwrap();
			let sns_y = sns_y.split_once(':').unwrap().0;
			let sns_y = i64::from_str(sns_y).unwrap();

			let bcn_x = words.next().unwrap();
			let bcn_x = bcn_x.split_once(',').unwrap().0;
			let bcn_x = i64::from_str(bcn_x).unwrap();

			let bcn_y = words.next().unwrap();
			let bcn_y = i64::from_str(bcn_y).unwrap();

			((sns_x, sns_y), (bcn_x, bcn_y))
		})
		.collect::<Vec<_>>()
}

fn no_beacons_at(sensors :&[((i64, i64), (i64, i64))], row_num :i64) -> usize {
	let beacons = sensors.iter()
		.filter(|(_sns_p, bcn_p)| bcn_p.1 == row_num)
		.map(|(_sns_p, bcn_p)| bcn_p.0 as i64)
		.collect::<HashSet<_>>();
	let mut no_beacons = HashSet::new();
	for (sns_p, bcn_p) in sensors {
		let dist_x = (sns_p.0 - bcn_p.0).abs();
		let dist_y = (sns_p.1 - bcn_p.1).abs();
		let dist = dist_x + dist_y;

		let d = (sns_p.1 - row_num).abs();
		if dist < d {
			continue;
		}
		let start_x = sns_p.0 - (dist - d);
		let end_x = sns_p.0 + (dist - d);
		for x in start_x..=end_x {
			no_beacons.insert(x);
		}
	}
	no_beacons.retain(|x| !beacons.contains(x));
	no_beacons.len()
}
