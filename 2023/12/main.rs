use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pt_rle = parse(INPUT);
	println!("sum counts: {}", sum_counts(&pt_rle));
	println!("sum counts (folded): {}", sum_counts_folded(&pt_rle));
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

#[allow(unused)]
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
	arrangement_count_bruteforce_inner(&mut pattern, rle, 0, &mut HashMap::new())
}

fn arrangement_count_bruteforce_inner(pattern :&mut [SpringState], rle :&[u16], i :usize, hm :&mut HashMap<FitState, u64>) -> u64 {
	let (fits_somewhat, fits_completely, fit_state) = rle_prefix_fits(pattern, rle);
	if !fits_somewhat {
		return 0;
	} else if fits_completely {
		return 1;
	} else if let Some(cnt) = hm.get(&fit_state) {
		return *cnt;
	}
	for j in i.. {
		match pattern.get(j) {
			Some(SpringState::Unknown) => {
				pattern[j] = SpringState::Operational;
				//println!("j: {j} pattern OP: {}", pattern_to_str(pattern));
				let op_cnt = arrangement_count_bruteforce_inner(pattern, rle, j + 1, hm);
				pattern[j] = SpringState::Damaged;
				//println!("j: {j} pattern DM: {}", pattern_to_str(pattern));
				let dmg_cnt = arrangement_count_bruteforce_inner(pattern, rle, j + 1, hm);
				pattern[j] = SpringState::Unknown;
				let res = op_cnt + dmg_cnt;

				hm.insert(fit_state, res);
				return res;
			},
			Some(_) => {
				// Just advance j
			},
			None => {
				// Final case, determine rle and compare
				return rle_prefix_fits(pattern, rle).1 as u64;
			},
		}
	}
	unreachable!();
}

type FitState = (Option<usize>, u16, usize);

fn rle_prefix_fits(pattern :&[SpringState], rle :&[u16]) -> (bool, bool, FitState) {
	let mut damaged_len = 0;
	let mut ends_at_unknown = None;
	let mut built = Vec::new();
	for (i, pt) in pattern.iter().enumerate() {
		match pt {
			SpringState::Operational => {
				if damaged_len > 0 {
					built.push(damaged_len);
					damaged_len = 0;
				}
			},
			SpringState::Damaged => {
				damaged_len += 1;
			},
			SpringState::Unknown => {
				ends_at_unknown = Some(i);
				break;
			},
		}
	}

	if damaged_len > 0 && ends_at_unknown.is_none() {
		built.push(damaged_len);
	}

	let fits_somewhat = rle.starts_with(&built);

	let fit_state = (ends_at_unknown, damaged_len, built.len());
	let fits_completely = fits_somewhat && rle.len() == built.len() && ends_at_unknown.is_none();
	//println!("  pattern: {} -> {:?}", pattern_to_str(pattern), res);
	(fits_somewhat, fits_completely, fit_state)
}

fn fold(pattern :&[SpringState]) -> Vec<SpringState> {
	let it = pattern.iter()
		.map(|pat| *pat)
		.chain(std::iter::once(SpringState::Unknown));
	std::iter::repeat(it)
		.take(4)
		.flatten()
		.chain(pattern.iter().map(|p| *p))
		.collect::<Vec<_>>()
}

fn fold_rle(pattern :&[u16]) -> Vec<u16> {
	let it = pattern.iter()
		.map(|pat| *pat);
	std::iter::repeat(it)
		.take(5)
		.flatten()
		.collect::<Vec<_>>()
}

fn sum_counts(list :&[(Vec<SpringState>, Vec<u16>)]) -> u64 {
	list.iter()
		.map(|(pattern, rle)| { arrangement_count_bruteforce(pattern, rle) })
		.sum()
}

fn sum_counts_folded(list :&[(Vec<SpringState>, Vec<u16>)]) -> u64 {
	list.iter()
		.map(|(pattern, rle)| (fold(pattern), fold_rle(rle)))
		.map(|(pattern, rle)| {
			//println!("{}", pattern_to_str(&pattern));
			arrangement_count_bruteforce(&pattern, &rle)
		})
		.sum()
}
