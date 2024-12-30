use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lks = parse(INPUT);
	println!("unique fitting pairs: {}", lks.unique_fitting_pairs());
}

fn parse(s :&str) -> LockKeys {
	let mut lines = s.trim()
		.lines()
		.map(str::trim);

	let mut keys = Vec::new();
	let mut locks = Vec::new();

	'outer: loop {
		let mut st = [0; 5];
		let mut mode = None;
		while let Some(l) = lines.next() {
			let Some(m) = mode else {
				mode = Some(l.chars().next().unwrap());
				continue;
			};
			if l.is_empty() {
				match m {
					'.' => keys.push(st),
					'#' => locks.push(st),
					_ => panic!("invalid char '{m}'"),
				}
				continue 'outer;
			}
			assert_eq!(l.len(), st.len());
			for (i, ch) in l.chars().enumerate() {
				if ch == m {
					st[i] += 1;
				}
			}
		}
		break;
	}

	LockKeys {
		keys,
		locks,
	}
}

struct LockKeys {
	keys :Vec<[u8; 5]>,
	locks :Vec<[u8; 5]>,
}

impl LockKeys {
	fn unique_fitting_pairs(&self) -> u32 {
		let locks_hs = self.locks.iter()
			.copied()
			.collect::<HashSet<[_; 5]>>();
		assert_eq!(locks_hs.len(), self.locks.len());
		self.keys.iter()
			.filter(|k| locks_hs.contains(*k))
			.count() as u32
	}
}
