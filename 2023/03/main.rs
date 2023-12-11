use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pn_gr = pn_gr(INPUT);
	println!("part number sum: {}", pn_gr.0.iter().sum::<u32>());
	println!("gear ratios: {}", pn_gr.1.iter().sum::<u32>());
}

fn run_around_pos(pos :(usize, usize), mut f: impl FnMut(usize, usize)) {
	if pos.0 > 0 {
		if pos.1 > 0 {
			f(pos.0 - 1, pos.1 - 1);
		}
		f(pos.0 - 1, pos.1);
		f(pos.0 - 1, pos.1 + 1);
	}
	if pos.1 > 0 {
		f(pos.0, pos.1 - 1);
		f(pos.0 + 1, pos.1 - 1);
	}
	f(pos.0, pos.1 + 1);
	f(pos.0 + 1, pos.1);
	f(pos.0 + 1, pos.1 + 1);
}

fn pn_gr(input :&str) -> (Vec<u32>, Vec<u32>) {
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut numbers = HashMap::<usize, Vec<_>>::new();
	for (i, l) in lines.clone().enumerate() {
		let mut v = None;
		for (j, ch) in l.chars().enumerate() {
			match (ch.is_ascii_digit(), v) {
				(false, Some((st, num))) => {
					numbers.entry(i)
						.or_default()
						.push((st, j - 1, num));
					v = None;
				},
				(true, Some((st, num))) => {
					let mut num = num;
					num *= 10;
					num += (ch as u8 - b'0') as u32;
					v = Some((st, num));
				},
				(false, None) => (),
				(true, None) => {
					let num = (ch as u8 - b'0') as u32;
					v = Some((j, num));
				},
			}
		}
		if let Some((st, num)) = v {
			numbers.entry(i)
				.or_default()
				.push((st, l.len() - 1, num));
		}
	}
	//println!("numbers: {numbers:?}");
	let mut part_nums = HashSet::new();
	let mut gear_ratios = Vec::new();
	for (i, l) in lines.clone().enumerate() {
		for (j, ch) in l.chars().enumerate() {
			if !ch.is_ascii_punctuation() || ch == '.' {
				continue;
			}
			//println!(" punct {ch} at ({i}, {j})");
			let mut adj = HashSet::new();
			run_around_pos((i, j), |i_n, j_n| {
				let Some(nums) = numbers.get(&i_n) else { return };
				let maybe_num = nums.iter()
					.find(|(st, end, _num)| {
						(st..=end).contains(&&j_n)
					});
				if let Some(num) = maybe_num {
					//println!("   found {num:?} at ({i_n}, {j_n})");
					part_nums.insert((i_n, num));
					if ch == '*' {
						adj.insert((i_n, num));
					}
				}
			});
			if adj.len() == 2 {
				let gear_ratio = adj.iter()
					.map(|(_i, (_st, _end, num))| *num)
					.product();
				gear_ratios.push(gear_ratio);
			}
		}
	}
	let mut part_nums_vec = part_nums.into_iter()
		.map(|(_i, (_st, _end, num))| *num)
		.collect::<Vec<_>>();
	part_nums_vec.sort();
	(part_nums_vec, gear_ratios)
}
