use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let dh = parse(INPUT);
	let sum = sum_of_small_dirs(&dh);
	println!("sum of small dirs: {sum}");
}

type DirHierarchy = HashMap<String, DirEntry>;

enum DirEntry {
	Dir(HashMap<String, DirEntry>),
	File(u64),
}

fn traverse_mut<'a>(dh :&'a mut DirHierarchy, path :&'a [String]) -> &'a mut DirHierarchy {
	path.iter()
		.fold(dh, |cur :&mut DirHierarchy, seg| {
			let cur = cur.entry(seg.clone()).or_insert_with(|| DirEntry::Dir(HashMap::new()));
			let DirEntry::Dir(hm) = cur else {
				panic!("path {path:?} has non-dir elem '{seg}'");
			};
			hm
		})
}

fn parse(input :&str) -> DirHierarchy {
	let lines = input.lines();
	let mut res = HashMap::new();
	let mut cur_path = Vec::<String>::new();
	for line in lines {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}
		if line.starts_with('$') {
			// command line. read the command.
			if line.starts_with("$ ls") {
				// Ignore
			} else if line.starts_with("$ cd") {
				let Some(operand) = line.split(' ').nth(2) else {
					panic!("Can't get operand from line '{line}'");
				};
				if operand == ".." {
					// cd .. in / should not error but be no-op
					_ = cur_path.pop();
				} else if operand.starts_with("/") {
					cur_path = operand.split("/")
						.map(|s| s.to_owned())
						.skip(1)
						.collect::<Vec<_>>();
				} else {
					cur_path.extend(operand.split("/").map(|s| s.to_owned()));
				}
			} else {
				panic!("Invalid cmd '{line}'");
			}
			// We have handled the case.
			continue;
		}
		let words = line.split(' ').collect::<Vec<_>>();
		assert_eq!(words.len(), 2, "invalid line '{line}'");
		let dir = traverse_mut(&mut res, &cur_path);
		if words[0] == "dir" {
			// Not really needed but shrug.
			dir.insert(words[1].to_owned(), DirEntry::Dir(HashMap::new()));
		} else {
			let size = u64::from_str(words[0]).unwrap();
			dir.insert(words[1].to_owned(), DirEntry::File(size));
		}
	}
	res
}

fn sum_of_small_dirs(dh :&DirHierarchy) -> u64 {
	dir_and_small_dir_size(dh).1
}

fn dir_and_small_dir_size(dh :&DirHierarchy) -> (u64, u64) {
	let mut dir_size = 0;
	let mut small_dir_size = 0;
	for (_name, entry) in dh.iter() {
		match entry {
			DirEntry::Dir(entries) => {
				let (siz, small_siz) = dir_and_small_dir_size(&entries);
				//println!("  for '{_name}': siz={siz} sm={small_siz}");
				dir_size += siz;
				small_dir_size += small_siz;
				//println!("  sms={small_dir_size}");
			},
			DirEntry::File(siz) => dir_size += siz,
		}
	}
	if dir_size <= 100_000 {
		small_dir_size += dir_size;
	}
	//println!("ret: siz={dir_size} sm={small_dir_size}");
	(dir_size, small_dir_size)
}
