const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let labels = get_labels_after_100(&nums);
	println!("Labels after 100 moves: {labels}");
}

fn parse(input :&str) -> Vec<u8> {
	input.trim()
		.chars()
		.map(|c| c as u8 - b'0')
		.collect::<Vec<_>>()
}

struct Cups {
	current :usize,
	cups :Vec<u8>,
}

impl Cups {
	fn new(cups :Vec<u8>) -> Self {
		Self {
			current : 0,
			cups,
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
				tgt = *self.cups.iter().max().unwrap();
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
		let mut dest_idx = dest_idx.unwrap();
		// Adjust for removal of cups
		let new_current = if dest_idx > current {
			dest_idx -= 3;
			current + 1
		} else {
			current + 4
		};
		let new_current = new_current % self.cups.len();
		// Now we put the cups next to the destination cup
		let c1 = self.cups.remove(current + 1);
		let c2 = self.cups.remove(current + 1);
		let c3 = self.cups.remove(current + 1);
		//println!("pick up: {c1} {c2} {c3}");
		//println!("destination: {}", self.cups[dest_idx]);
		self.cups.insert(dest_idx + 1, c1);
		self.cups.insert(dest_idx + 2, c2);
		self.cups.insert(dest_idx + 3, c3);

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
}

fn get_labels_after_n(nums :&[u8], n: u16) -> String {
	let mut cups = Cups::new(nums.to_vec());
	for _ in 0..n {
		cups.do_move();
	}
	cups.get_labels()
}

fn get_labels_after_100(nums :&[u8]) -> String {
	get_labels_after_n(nums, 100)
}
