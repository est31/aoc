use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

/*
1: 2 segments
4: 4 segments
7: 3 segments
8: 7 segments

0,6,9: 6 segments
2,3,5: 5 segments
*/

fn main() {
	println!("1,4,7,8 total count: {}", count_1478(INPUT));
	println!("decoded sum: {}", decoded_sum(INPUT));
}

fn count_1478(input :&str) -> usize {
	input.lines()
		.map(str::trim)
		.filter(|l| !l.is_empty())
		.map(|l| {
			let s = l.split('|')
				.nth(1).unwrap();
			s.trim()
				.split_whitespace()
				.filter(|s| matches!(&s.trim().len(), &2 | &4 | &3 | &7))
				.count()
		})
		.sum()
}

fn determine_mapping(p :[u8; 10]) -> HashMap<u8, u8> {
	let mut result = HashMap::new();

	let lengths = p.map(|v| v.count_ones());
	let find_lengths = |ndl| {
		lengths.iter()
			.enumerate()
			.filter(move |(_i, l)| **l == ndl)
			.map(|(i, _l)| i)
	};
	let find_length = |ndl| {
		find_lengths(ndl).next().unwrap()
	};

	let remove_with_common_segment_count = |list :&mut Vec<usize>, ndl :u8, number_of_segments| {
		let idx = list.iter()
			.enumerate()
			.filter(move |(_i, p_i)| (p[**p_i] & ndl).count_ones() == number_of_segments)
			.map(|(i, _p_i)| i)
			.next().unwrap();
		list.remove(idx)
	};

	// Determine 1,4,7,8 by their unique lengths
	let index_1 = find_length(2);
	let index_4 = find_length(4);
	let index_7 = find_length(3);
	let index_8 = find_length(7);

	result.insert(p[index_1], 1);
	result.insert(p[index_4], 4);
	result.insert(p[index_7], 7);
	result.insert(p[index_8], 8);

	// 0,6,9 all have 6 segments
	let mut indices_069 = find_lengths(6).collect::<Vec<_>>();
	assert_eq!(indices_069.len(), 3);

	// 6 is the only one that has only one segment in common with 1
	let index_6 = remove_with_common_segment_count(&mut indices_069, p[index_1], 1);
	// 9 and 0 remain. 9 wholly contains 4
	let index_9 = remove_with_common_segment_count(&mut indices_069, p[index_4], 4);
	let index_0 = indices_069.pop().unwrap();

	result.insert(p[index_0], 0);
	result.insert(p[index_6], 6);
	result.insert(p[index_9], 9);

	// 2,3,5 all have 5 segments
	let mut indices_235 = find_lengths(5).collect::<Vec<_>>();
	assert_eq!(indices_235.len(), 3);

	// 3 is the only one that has 2 segments in common with 1
	let index_3 = remove_with_common_segment_count(&mut indices_235, p[index_1], 2);
	// 2 and 5 remain. 9 wholly contains 5
	let index_5 = remove_with_common_segment_count(&mut indices_235, p[index_9], 5);
	let index_2 = indices_235.pop().unwrap();

	result.insert(p[index_2], 2);
	result.insert(p[index_3], 3);
	result.insert(p[index_5], 5);

	result

}

fn parse_line(line :&str) -> ([u8; 10], [u8; 4]) {
	let mut it = line.split('|');
	fn letter_to_digit(l :char) -> u8 {
		if !('a'..='g').contains(&l) {
			panic!("Invalid letter '{}'", l);
		}
		1 << (l as u32 - 'a' as u32)
	}
	fn build_arr<const N :usize>(s :&str) -> [u8; N] {
		let mut git = s.split_whitespace();
		[0u8; N].map(|_| {
			let group = git.next().unwrap();
			group.chars().map(letter_to_digit).sum()
		})
	}
	(build_arr(it.next().unwrap()), build_arr(it.next().unwrap()))
}

fn find_number(line :&str) -> u32 {
	let (p, s) = parse_line(line);
	let mapping = determine_mapping(p);
	let digits = s.map(|d| mapping[&d]);
	digits.iter().fold(0, |v, w| (v as u32) * 10 + (*w as u32))
}

fn decoded_sum(input :&str) -> u32 {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(find_number)
		.sum()
}
