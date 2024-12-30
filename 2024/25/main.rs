use std::collections::HashMap;

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
		match mode {
			None => (),
			Some('.') => keys.push(st),
			Some('#') => locks.push(st),
			Some(m) => panic!("invalid char '{m}'"),
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

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

fn check(lock :&[u8; 5], key :&[u8; 5]) -> bool {
	let res = lock.iter()
		.zip(key.iter())
		.all(|(l, k)| *l + (5 - *k) <= 5);
	dprint!("    checking lock={lock:?} and key={key:?} -> {res}\n");
	res
}

impl LockKeys {
	fn unique_fitting_pairs(&self) -> u32 {
		dprint!("keys: {:?}\n", self.keys);
		dprint!("locks: {:?}\n", self.locks);
		let mut locks_hm = HashMap::<_, Vec<_>>::new();
		for lock in self.locks.iter() {
			let key = lock[0];
			locks_hm.entry(key).or_default().push(*lock);
		}
		dprint!("locks hm: {:?}\n", locks_hm);
		let mut count = 0;
		for k in self.keys.iter() {
			dprint!("  key: {k:?}\n");
			for v in 0..=k[0] {
				let Some(locks) = locks_hm.get(&v) else { continue };
				count += locks.iter()
					.filter(|lock| check(lock, k))
					.count();
			}
		}
		count as u32
	}
}
