use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sensors = parse(INPUT);
	let nba = no_beacons_at(&sensors, 2_000_000);
	println!("No beacons at 2M: {nba}");
	let tf = find_tuning_freq(&sensors, 4_000_000);
	println!("Tuning freq: {tf}");
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

fn no_beacons_at(sensors :&[((i64, i64), (i64, i64))], row_num :i64) -> u64 {
	let beacons = sensors.iter()
		.filter(|(_sns_p, bcn_p)| bcn_p.1 == row_num)
		.map(|(_sns_p, bcn_p)| bcn_p.0 as i64)
		.collect::<HashSet<_>>();
	let no_beacons = no_beacons_intervals(sensors, row_num);
	let sum :i64 = no_beacons.iter()
		.map(|(st, end)| end - st + 1)
		.sum();
	sum as u64 - beacons.len() as u64
}

fn intervals_overlap(v :(i64, i64), w :(i64, i64)) -> bool {
	(v.0..=v.1).contains(&w.0) || (v.0..=v.1).contains(&w.1)
		|| (w.0..=w.1).contains(&v.0) || (w.0..=w.1).contains(&v.1)
}

fn no_beacons_intervals(sensors :&[((i64, i64), (i64, i64))], row_num :i64) -> Vec<(i64, i64)> {
	let mut no_beacons = Vec::new();
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
		let mut chng_idcs = no_beacons.iter()
			.enumerate()
			.filter(|(_i, itl)| intervals_overlap(**itl, (start_x, end_x)))
			.map(|(i, itvl)| (i, *itvl))
			.collect::<Vec<_>>();
		//println!("add {start_x}, {end_x}: {chng_idcs:?} INSIDE {no_beacons:?}");
		if chng_idcs.len() > 0 {
			//assert!([1, 2].contains(&chng_idcs.len()), "Wrong num: {chng_idcs:?}");
			// There is an overlap of one or multiple intervals.
			let start = chng_idcs[0].1.0.min(start_x);
			let end = chng_idcs.last().unwrap().1.1.max(end_x);
			while chng_idcs.len() > 1 {
				// TODO use let_chains once stable
				let i_end = chng_idcs.pop().unwrap().0;
				let i_st = chng_idcs.last().unwrap().0;
				for i in (i_st + 1)..=i_end {
					no_beacons.remove(i);
				}
			}
			no_beacons[chng_idcs[0].0] = (start, end);
		} else {
			// No overlap, just insert at the right position.
			let mut inserted = false;
			for i in 0..no_beacons.len() {
				if no_beacons[i].0 > end_x {
					no_beacons.insert(i, (start_x, end_x));
					inserted = true;
				}
			}
			if !inserted {
				no_beacons.push((start_x, end_x));
			}
		}

	}
	no_beacons
}

fn find_tuning_freq(sensors :&[((i64, i64), (i64, i64))], max_coord :i64) -> u64 {
	let mut found = None;
	for row_num in 0..=max_coord {
		let no_beacons = no_beacons_intervals(sensors, row_num);
		let no_beacons_count :i64 = no_beacons.iter()
			.filter(|itl| intervals_overlap(**itl, (0, max_coord)))
			.map(|&(st, end)| end.min(max_coord) - st.max(0) + 1)
			.sum();
		if no_beacons_count == max_coord + 1 {
			continue;
		}
		assert_eq!(no_beacons_count, max_coord);
		let intervals_restricted = no_beacons.iter()
			.filter(|itl| intervals_overlap(**itl, (0, max_coord)))
			.map(|&(st, end)| (st.max(0), end.min(max_coord)))
			.collect::<Vec<_>>();
		let col_num = match intervals_restricted.as_slice() {
			&[(st, end)] => {
				if st == 0 {
					0
				} else if end == max_coord - 1 {
					max_coord
				} else {
					panic!("Wrong single interval: {intervals_restricted:?}");
				}
			},
			&[(_, end_0), (st_1, _)] => {
				assert_eq!(st_1, end_0 + 2);
				end_0 + 1
			},
			_ => panic!("Wrong number of intervals: {intervals_restricted:?}"),
		};

		found = Some((row_num, col_num));
	}
	let (row, col) = found.unwrap();
	col as u64 * 4000000 + row as u64
}
