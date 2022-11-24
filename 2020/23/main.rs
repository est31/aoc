const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let labels = get_labels_after_100(&nums);
	println!("Labels after 100 moves: {labels}");
}

fn parse(input :&str) -> Vec<u32> {
	input.trim()
		.chars()
		.map(|c| (c as u8 - b'0').into())
		.collect::<Vec<_>>()
}

struct Cups {
	current :usize,
	cups :Vec<u32>,
	biggest_cup_num :u32,
}

impl Cups {
	fn new(cups :Vec<u32>) -> Self {
		let biggest_cup_num = *cups.iter().max().unwrap();
		Self {
			current : 0,
			cups,
			biggest_cup_num,
		}
	}
	fn add_until_million(&mut self) {
		for c in self.cups.len()..1_000_000 {
			self.cups.push(c as u32);
		}
		self.biggest_cup_num = 1_000_000 - 1;
	}
	fn move_from_to(&mut self, source :usize, dest :usize) {
		// let cup = self.cups.remove(source);
		// self.cups.insert(dest, cup);
		if source == dest {
			// Nothing to do! :)
			return;
		}
		if source < dest {
			if (dest - source) * 2 > self.cups.len() {
				let cup = self.cups.remove(source);
				self.cups.insert(dest, cup);
			} else {
				for i in source..dest {
					self.cups.swap(i, i + 1);
				}
			}
		} else {
			if (source - dest) * 2 > self.cups.len() {
				let cup = self.cups.remove(source);
				self.cups.insert(dest, cup);
			} else {
				for i in (dest..source).rev() {
					self.cups.swap(i, i + 1);
				}
			}
		}
	}
	fn do_move(&mut self) {
		let len = self.cups.len();
		//println!("--move--");
		//println!("cups: {:?} {}", self.cups, self.cups[self.current]);
		// Make sure there is no wrapping for the removed cups
		while (self.current + 3) >= self.cups.len() {
			self.current -= 1;
			let num = self.cups.remove(0);
			self.cups.push(num);
		}
		let current = self.current;
		// Find destination cup.
		let mut tgt = self.cups[current] - 1;
		let mut dest_idx = None;
		while dest_idx.is_none() {
			if tgt == 0 {
				tgt = self.biggest_cup_num;
			}
			let found = self.cups.iter().enumerate().find(|(_i, c)| c == &&tgt);
			if let Some((tgt_idx, _tgt)) = found {
				if ((current+1)..=(current + 3)).contains(&tgt_idx) {
					tgt -= 1;
					continue;
				}
				dest_idx = Some(tgt_idx);
			}
			tgt -= 1;
		}
		let dest_idx = dest_idx.unwrap();

		//println!("pick up: {} {} {}", self.cups[current + 1], self.cups[current + 2], self.cups[current + 3]);
		//println!("destination: {}", self.cups[dest_idx]);

		// Adjust for removal of cups,
		// and put the cups next to the destination cup
		let new_current = if dest_idx > current {
			self.move_from_to(current + 1, dest_idx);
			self.move_from_to(current + 1, dest_idx);
			self.move_from_to(current + 1, dest_idx);
			current + 1
		} else {
			self.move_from_to(current + 1, dest_idx + 1);
			self.move_from_to(current + 2, dest_idx + 2);
			self.move_from_to(current + 3, dest_idx + 3);
			current + 4
		};
		let new_current = new_current % self.cups.len();

		self.current = new_current;
		assert_eq!(len, self.cups.len());
	}
	fn get_labels(&self) -> String {
		let mut res = String::new();
		let idx_of_one = self.cups.iter()
			.enumerate()
			.find(|(_, num)| **num == 1)
			.unwrap()
			.0;
		for i in 1..self.cups.len() {
			let j = (i + idx_of_one) % self.cups.len();
			let num = self.cups[j];
			res = format!("{res}{num}");
		}
		res
	}
	fn get_labels_product_right_of_one(&self) -> u64 {
		let idx_of_one = self.cups.iter()
			.enumerate()
			.find(|(_, num)| **num == 1)
			.unwrap()
			.0;
		let next = (idx_of_one + 1) % self.cups.len();
		let next2 = (idx_of_one + 2) % self.cups.len();
		let next = self.cups[next] as u64;
		let next2 = self.cups[next2] as u64;
		next * next2
	}
}

fn get_labels_after_n(nums :&[u32], n: u16) -> String {
	let mut cups = Cups::new(nums.to_vec());
	for _ in 0..n {
		cups.do_move();
	}
	cups.get_labels()
}

fn get_labels_after_100(nums :&[u32]) -> String {
	get_labels_after_n(nums, 100)
}

fn get_labels_after_ten_million(nums :&[u32]) -> u64 {
	let mut cups = Cups::new(nums.to_vec());
	cups.add_until_million();
	for _ctr in 0..10_000_000u64 {
		if _ctr % 10_000 == 0 {
			println!("{_ctr} => {:.2}", (_ctr as f64 / 10_000_000.0));
		}
		cups.do_move();
	}
	cups.get_labels_product_right_of_one()
}
