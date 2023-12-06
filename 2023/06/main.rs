use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let times_max_dist = parse(INPUT);
	println!("number product: {}", numbers_product(&times_max_dist));
	let corr = parse_kernig_correction(INPUT);
	println!("kerning corrected: {}", number_of_beating_hold_times(corr.0, corr.1));
}

fn parse(input :&str) -> Vec<(u64, u64)> {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let times = lines.next().unwrap();
	let max_distances = lines.next().unwrap();
	let times = times.split_whitespace()
		.skip(1)
		.map(|c| u64::from_str(c).unwrap())
		.collect::<Vec<_>>();
	let max_distances = max_distances.split_whitespace()
		.skip(1)
		.map(|c| u64::from_str(c).unwrap())
		.collect::<Vec<_>>();
	times.into_iter()
		.zip(max_distances.into_iter())
		.collect::<Vec<_>>()
}

fn parse_kernig_correction(input :&str) -> (u64, u64) {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let times = lines.next().unwrap();
	let max_distances = lines.next().unwrap();

	fn process(input :&str) -> u64 {
		let s = input.split(':')
			.nth(1)
			.unwrap();
		let s = s.split_whitespace()
			.collect::<String>();
		u64::from_str(&s).unwrap()
	}
	let times = process(&times);
	let max_distances = process(&max_distances);

	(times, max_distances)
}

fn number_of_beating_hold_times_bf(time :u64, dist :u64) -> u64 {
	// How many h exist so that h*(time - h) > dist?
	// brute force search
	(1..time)
		.filter(|&h| h * (time - h) > dist)
		.count() as u64
}

fn isqrt(input :u64) -> u64 {
	// TODO https://github.com/rust-lang/rust/issues/116226
	// eww floats
	(input as f64).sqrt() as u64
}

#[allow(unused)]
// doesn't work, there is bugs :/
fn number_of_beating_hold_times_fast(time :u64, dist :u64) -> u64 {
	// question: how many h exist so that h*(time - h) > dist?
	// -h*h + h*time - dist = 0
	let disc = (time * time).checked_sub(4 * dist);
	println!("time={time} dist={dist} disc={disc:?}");
	match disc {
		None => 0,
		Some(0) => 1,
		Some(disc) => {
			let sqrt = isqrt(disc * 4);
			/*if sqrt*sqrt == disc {
				sqrt - 2
			} else {
				// sqrt was rounded down, meaning we need to add two
				sqrt + 1
			}*/
			let res = if (sqrt / 2) * 2 < sqrt {
				println!(" -> if");
				sqrt / 2 + 1
			} else {
				if (sqrt * sqrt) / 4 == disc {
					println!(" -> el 1");
					sqrt / 2 - 1
				} else {
					println!(" -> el 2");
					sqrt / 2
				}
			};
			println!(" -> sqrt={sqrt} sq={} res={res}", sqrt*sqrt);
			res
		},
	}
}

fn number_of_beating_hold_times(time :u64, dist :u64) -> u64 {
	number_of_beating_hold_times_bf(time, dist)
}

fn numbers_product(times_max_dist :&[(u64, u64)]) -> u64 {
	times_max_dist.iter()
		.map(|(time, max_dist)| number_of_beating_hold_times(*time, *max_dist))
		.product()
}
