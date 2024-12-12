use std::collections::{HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let map = parse(INPUT);
	println!("defrag checksum: {}", defrag_checksum(&map));
}

macro_rules! dprint {
	($($args:expr),*) => {
		//if false
			{ print!($($args),*); }
	};
}

fn parse(s: &str) -> Vec<u8> {
	let s = s.trim();
	s.chars()
		.map(|ch| {
			if !('0'..='9').contains(&ch) {
				panic!("Not a number: '{ch}'");
			}
			ch as u8 - '0' as u8
		})
		.collect::<Vec<_>>()
}

fn defrag_checksum(map: &[u8]) -> u128 {
	let mut disk_chunks = Vec::new();
	// Preparation
	let mut file_ctr = 0;
	for (i, count) in map.iter().enumerate() {
		let id_num = if i % 2 == 0 {
			file_ctr += 1;
			file_ctr
		} else {
			0
		};
		disk_chunks.push((*count, id_num));
	}
	// Defrag
	let mut filled_before = 0;
	advance_filled_before(&disk_chunks, &mut filled_before);
	while filled_before < disk_chunks.len() - 1 {
		dprint!("filled_before = {filled_before}\n");
		let (last_cnt, last_v) = *disk_chunks.last().unwrap();
		let avail_dest = disk_chunks[filled_before].0;
		let avail_orig = last_cnt;
		let moved_cnt;
		dprint!("  -> avail_dest={avail_dest}, avail_orig={avail_orig}\n");
		if avail_dest <= avail_orig {
			moved_cnt = avail_dest;
			dprint!("  -> update ({}, {}) to ({moved_cnt}, {last_v})\n", disk_chunks[filled_before].0, disk_chunks[filled_before].1);
			disk_chunks[filled_before] = (moved_cnt, last_v);
		} else {
			moved_cnt = avail_orig;
			disk_chunks[filled_before].0 -= moved_cnt;
			disk_chunks.insert(filled_before, (moved_cnt, last_v));
			dprint!("  -> insert ({moved_cnt}, {last_v})\n");
		}
		disk_chunks.last_mut().unwrap().0 -= moved_cnt;
		if disk_chunks.last_mut().unwrap().0 == 0 {
			dprint!("  -> pop last chunk\n");
			disk_chunks.pop();
		}
		advance_filled_before(&disk_chunks, &mut filled_before);
	}
	dprint!("resulting disk_chunks: {disk_chunks:?}\n");
	// Checksum
	let mut sum = 0;
	let mut pos = 0;
	for (cnt, v) in disk_chunks.iter() {
		for _ in 0..*cnt {
			sum += pos * (*v as u128).saturating_sub(1);
			pos += 1;
		}
	}
	sum
}

fn advance_filled_before(disk_chunks: &[(u8, u32)], filled_before: &mut usize) {
	while *filled_before < disk_chunks.len() && (disk_chunks[*filled_before].1 != 0 || disk_chunks[*filled_before].0 == 0) {
		*filled_before += 1;
	}
}
