const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let nums = parse(INPUT);
	let labels = get_labels_after_100(&nums);
	println!("Labels after 100 moves: {labels}");
	let lr = get_labels_after_ten_million(&nums);
	println!("Labels after 10 million moves: {}", lr.0 as u64 * lr.1 as u64);
}

fn parse(input :&str) -> Vec<u32> {
	input.trim()
		.chars()
		.map(|c| (c as u8 - b'0').into())
		.collect::<Vec<_>>()
}

struct Cups {
	current :u32,
	cups :Vec<(u32, u32)>,
	biggest_cup_num :u32,
}

impl Cups {
	fn new(icups :Vec<u32>) -> Self {
		Self::new_with_additions(icups, false)
	}
	fn new_with_additions(icups :Vec<u32>, million :bool) -> Self {
		let mut biggest_cup_num = *icups.iter().max().unwrap();
		let mut cups = vec![(0, 0); biggest_cup_num as usize + 1];
		for win in icups.windows(3) {
			cups[win[1] as usize] = (win[0], win[2]);
		}
		if million {
			for c in icups.len()..1_000_000 {
				cups.push((c as u32 - 1, c as u32 + 1));
			}
			cups[icups.len()].0 = *icups.last().unwrap();
			cups.last_mut().unwrap().1 = icups[0];

			cups[icups[0] as usize] = (cups.len() as u32 - 1, icups[1]);
			cups[*icups.last().unwrap() as usize] = (icups[icups.len() - 2], icups.len() as u32);

			biggest_cup_num = 1_000_000 - 1;
		} else {
			cups[icups[0] as usize] = (*icups.last().unwrap(), icups[1]);
			cups[*icups.last().unwrap() as usize] = (icups[icups.len() - 2], icups[0]);
		}
		Self {
			current : icups[0],
			cups,
			biggest_cup_num,
		}
	}
	fn do_move(&mut self) {
		//println!("--move--");
		//println!("cups: {:?} {}", self.cups, self.cups[self.current]);
		// Make sure there is no wrapping for the removed cups
		let current = self.current;

		let c1 = self.cups[current as usize].1;
		let c2 = self.cups[c1 as usize].1;
		let c3 = self.cups[c2 as usize].1;

		let new_current = self.cups[c3 as usize].1;

		// Find destination cup.
		let mut tgt = current - 1;
		let mut dest = None;
		while dest.is_none() {
			if tgt == 0 {
				tgt = self.biggest_cup_num;
			}
			if self.cups[tgt as usize] == (0, 0) {
				panic!("Cup {tgt} doesn't exist");
			}
			if [c1, c2, c3].contains(&tgt) {
				tgt -= 1;
				continue;
			}
			dest = Some(tgt);

			tgt -= 1;
		}
		let dest = dest.unwrap();
		let dest_succ = self.cups[dest as usize].1;

		//println!("pick up: {} {} {}", c1, c2, c3);
		//println!("destination: {}", self.cups[dest_idx]);

		// Do the move.

		// First cut them out from their old place
		self.cups[current as usize].1 = new_current;
		self.cups[new_current as usize].0 = current;

		// Then put them into their new one
		self.cups[dest as usize].1 = c1;
		self.cups[dest_succ as usize].0 = c3;

		self.cups[c1 as usize].0 = dest;
		self.cups[c3 as usize].1 = dest_succ;

		self.current = new_current;
	}
	fn get_labels(&self) -> String {
		let mut res = String::new();
		let mut cur = self.cups[1].1;
		while cur != 1 {
			res = format!("{res}{cur}");
			cur = self.cups[cur as usize].1;
		}
		res
	}
	fn get_labels_right_of_one(&self) -> (u32, u32) {
		let next = self.cups[1].1;
		let next2 = self.cups[next as usize].1;
		(next, next2)
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

fn get_labels_after_ten_million(nums :&[u32]) -> (u32, u32) {
	let mut cups = Cups::new_with_additions(nums.to_vec(), true);
	for _ctr in 0..10_000_000u64 {
		/*if _ctr % 10_000 == 0 {
			println!("{_ctr} => {:.2}", (_ctr as f64 / 10_000_000.0));
		}*/
		cups.do_move();
	}
	cups.get_labels_right_of_one()
}
