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

fn number_of_beating_hold_times(time :u64, dist :u64) -> u64 {
	// question: how many h exist so that h*(time - h) > dist?
	// h*time - h*h - dist = 0
	number_of_beating_hold_times_bf(time, dist)
}

fn numbers_product(times_max_dist :&[(u64, u64)]) -> u64 {
	times_max_dist.iter()
		.map(|(time, max_dist)| number_of_beating_hold_times(*time, *max_dist))
		.product()
}
