use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let times_max_dist = parse(INPUT);
	println!("number product: {}", numbers_product(&times_max_dist));
}

fn parse(input :&str) -> Vec<(u32, u32)> {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let times = lines.next().unwrap();
	let max_distances = lines.next().unwrap();
	let times = times.split_whitespace()
		.skip(1)
		.map(|c| u32::from_str(c).unwrap())
		.collect::<Vec<_>>();
	let max_distances = max_distances.split_whitespace()
		.skip(1)
		.map(|c| u32::from_str(c).unwrap())
		.collect::<Vec<_>>();
	times.into_iter()
		.zip(max_distances.into_iter())
		.collect::<Vec<_>>()
}

fn number_of_beating_hold_times_bf(time :u32, dist :u32) -> u32 {
	// How many h exist so that h*(time - h) > dist?
	(1..time)
		.filter(|&h| h * (time - h) > dist)
		.count() as u32
}

fn number_of_beating_hold_times(time :u32, dist :u32) -> u32 {
	// question: how many h exist so that h*(time - h) > dist?
	// h*time - h*h - dist = 0
	number_of_beating_hold_times_bf(time, dist)
}

fn numbers_product(times_max_dist :&[(u32, u32)]) -> u32 {
	times_max_dist.iter()
		.map(|(time, max_dist)| number_of_beating_hold_times(*time, *max_dist))
		.product()
}
