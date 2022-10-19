use std::collections::HashSet;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let v = first_not_sum(25, &nums);
	println!("First number that's not a sum: {v}");
	let (min, max) = contiguous_smallest_largest(v, &nums);
	println!("Sum of smallest and largest: {}", min + max);
}

fn parse(input :&str) -> Vec<u64> {
	input.lines()
		.map(|l| u64::from_str(l).unwrap())
		.collect()
}

fn two_sum(target :u64, input :&[u64]) -> Option<(u64, u64)> {
	// Looks similar? This is taken from day 1 and a little bit customized
	let set = input.iter()
		.copied()
		.collect::<HashSet<_>>();
	for v in input.iter() {
		if *v > target {
			continue;
		}
		if set.contains(&(target - v)) {
			return Some((*v, target - v));
		}
	}
	None
}

fn first_not_sum(window_size :usize, nums :&[u64]) -> u64 {
	for i in 0..(nums.len() - window_size) {
		let nums_offset = &nums[i..];
		let v = nums_offset[window_size];
		if two_sum(v, &nums_offset[..window_size]).is_some() {
			continue;
		}
		return v;
	}
	panic!("Not found!");
}

fn contiguous_smallest_largest(target :u64, nums :&[u64]) -> (u64, u64) {
	for i in 0..nums.len() {
		let mut sum = 0;
		for (j, m) in nums[i..].iter().enumerate() {
			sum += m;
			if sum > target {
				break;
			}
			if sum == target {
				let min = nums[i..=(i+j)].iter().min().unwrap();
				let max = nums[i..=(i+j)].iter().max().unwrap();
				return (*min, *max);
			}
		}
	}
	panic!("Not found!");
}
