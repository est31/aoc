use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pt_rle = parse(INPUT);
	println!("sum counts: {}", sum_arrangement_counts(&pt_rle));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
	Operational,
	Damaged,
	Unknown,
}

fn parse(input :&str) -> Vec<(Vec<SpringState>, Vec<u16>)> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut l = l.split_whitespace();
			let pattern_str = l.next().unwrap();
			let rle_str = l.next().unwrap();
			let pattern = pattern_str.chars()
				.map(|ch| match ch {
					'.' => SpringState::Operational,
					'#' => SpringState::Damaged,
					'?' => SpringState::Unknown,
					_ => panic!("unexpected char '{ch}' in '{pattern_str}'"),
				})
				.collect::<Vec<_>>();
			let rle = rle_str.split(',')
				.map(|v| {
					u16::from_str(v).unwrap()
				})
				.collect::<Vec<_>>();
			(pattern, rle)
		})
		.collect::<Vec<_>>()
}

fn pattern_to_str(pattern :&[SpringState]) -> String {
	pattern.iter()
		.map(|st| match st {
			SpringState::Operational => '.',
			SpringState::Damaged => '#',
			SpringState::Unknown => '?',
		})
		.collect()
}

fn arrangement_count_bruteforce(pattern :&[SpringState], rle :&[u16]) -> u64 {
	let mut pattern = pattern.to_vec();
	arrangement_count_bruteforce_inner(&mut pattern, rle, 0)
}

fn determine_rle(pattern :&[SpringState]) -> Vec<u16> {
	let mut damaged_len = 0;
	let mut res = Vec::new();
	for pt in pattern {
		match pt {
			SpringState::Operational => {
				if damaged_len > 0 {
					res.push(damaged_len);
					damaged_len = 0;
				}
			},
			SpringState::Damaged => {
				damaged_len += 1;
			},
			SpringState::Unknown => {
				panic!("not supposed to find unknown at this stage");
			},
		}
	}

	if damaged_len > 0 {
		res.push(damaged_len);
	}
	//println!("  pattern: {} -> {:?}", pattern_to_str(pattern), res);
	res
}

fn arrangement_count_bruteforce_inner(pattern :&mut [SpringState], rle :&[u16], i :usize) -> u64 {
	for j in i.. {
		match pattern.get(j) {
			Some(SpringState::Unknown) => {
				pattern[j] = SpringState::Operational;
				//println!("j: {j} pattern OP: {}", pattern_to_str(pattern));
				let op_cnt = arrangement_count_bruteforce_inner(pattern, rle, j + 1);
				pattern[j] = SpringState::Damaged;
				//println!("j: {j} pattern DM: {}", pattern_to_str(pattern));
				let dmg_cnt = arrangement_count_bruteforce_inner(pattern, rle, j + 1);
				pattern[j] = SpringState::Unknown;
				return op_cnt + dmg_cnt;
			},
			Some(_) => {
				// Just advance j
			},
			None => {
				// Final case, determine rle and compare
				let rle_given = determine_rle(pattern);
				return (rle == rle_given) as u64;
			},
		}
	}
	unreachable!();
}

fn sum_arrangement_counts(list :&[(Vec<SpringState>, Vec<u16>)]) -> u64 {
	list.iter()
		.map(|(pattern, rle)| { arrangement_count_bruteforce(pattern, rle) })
		.sum()
}
