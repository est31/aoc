use std::collections::HashMap;
use std::mem::take;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sum = sum_of_counts(INPUT);
	println!("sum of counts: {sum}");
	let sum = sum_of_everyone_yes(INPUT);
	println!("sum of everyone yes: {sum}");
}

fn sum_of_counts(input :&str) -> u32 {
	group_counts(input)
		.map(|(count, _num_everyone_yes)| count)
		.sum()
}

fn sum_of_everyone_yes(input :&str) -> u32 {
	group_counts(input)
		.map(|(_count, num_everyone_yes)| num_everyone_yes)
		.sum()
}

fn group_counts(input :&str) -> impl Iterator<Item=(u32, u32)> + '_ {
	let mut lines = input.lines();
	return std::iter::from_fn(move || {
		//print!("{group:?}");
		let mut group = HashMap::<char, u32>::new();
		let mut group_size = 0;
		while let Some(line) = lines.next() {
			let line = line.trim();
			if line.is_empty() {
				//println!(" -> {group:?}");
				let num_everyone_yes = group.iter()
					.filter(|(_c, v)| **v == group_size)
					.count();
				let ret = take(&mut group).len() as u32;
				return Some((ret, num_everyone_yes as u32));
			}
			group_size += 1;
			for ch in line.chars() {
				*group.entry(ch).or_default() += 1;
			}
		}
		let num_everyone_yes = group.iter()
			.filter(|(_c, v)| **v == group_size)
			.count();
		(group_size > 0).then_some((group.len() as u32, num_everyone_yes as u32))
	})
}
