use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let num = play_game(&nums);
	println!("Num: {num}");
	// TODO is there a trick here?
	let num = play_game_n(&nums, 30_000_000);
	println!("Num 30M: {num}");
}

fn parse(input :&str) -> Vec<u32> {
	input.trim()
		.split(',')
		.map(|n| u32::from_str(n).unwrap())
		.collect::<Vec<_>>()
}

fn play_game(nums :&[u32]) -> u32 {
	play_game_n(nums, 2020)
}

fn play_game_n(nums :&[u32], limit :usize) -> u32 {
	let mut times = HashMap::new();
	let mut last_cont = None;
	let mut last = 0;
	for (i, n) in nums.iter().enumerate() {
		last_cont = times.insert(*n, i);
		last = *n;
	}
	for i in nums.len()..limit {
		let cur = if let Some(lt) = last_cont {
			(i - lt - 1) as u32
		} else {
			0
		};
		last_cont = times.insert(cur, i);
		//println!("{cur}");
		last = cur;
	}
	//println!("");
	last
}
