use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mem = parse(INPUT);
	println!("min steps: {}", mem.min_steps());
	let btb = mem.first_byte_to_block();
	println!("first byte to block: {},{}", btb.0, btb.1);
}

#[derive(Clone, PartialEq, Eq)]
struct Memory {
	height :usize,
	width :usize,
	corruptions :Vec<(usize, usize)>,
}

fn parse(s :&str) -> Memory {
	parse_sized(s, 71)
}

fn parse_sized(s :&str, size :usize) -> Memory {
	let corruptions = s.trim()
		.lines()
		.map(str::trim)
		.map(|l| {
			let mut x_y_it = l.split(',')
				.map(|v| usize::from_str(v).unwrap());
			(x_y_it.next().unwrap(), x_y_it.next().unwrap())
		})
		.collect::<Vec<_>>();
	let height = size;
	let width = size;

	Memory {
		height,
		width,
		corruptions,
	}
}

#[inline(always)]
fn neighs(p: (usize, usize), height :usize, width :usize) -> Vec<(usize, usize)> {
	let mut neighs = Vec::with_capacity(4);
	if p.0 > 0 {
		neighs.push((p.0 - 1, p.1));
	}
	if p.1 > 0 {
		neighs.push((p.0, p.1 - 1));
	}
	if p.0 + 1 < height {
		neighs.push((p.0 + 1, p.1));
	}
	if p.1 + 1 < width {
		neighs.push((p.0, p.1 + 1));
	}
	neighs
}

impl Memory {
	fn min_steps(&self) -> usize {
		self.min_steps_after(1024).expect("didn't get to end")
	}
	fn min_steps_after(&self, ms :usize) -> Option<usize> {
		let mut fields = vec![vec![false; self.width]; self.height];

		for corr in self.corruptions.iter().take(ms) {
			fields[corr.0][corr.1] = true;
		}
		self.min_steps_with(&fields)
	}
	fn min_steps_with(&self, fields :&[Vec<bool>]) -> Option<usize> {
		let start_pos = (0, 0);
		let end_pos = (self.height - 1, self.width - 1);

		let mut handled = HashSet::new();
		let mut unhandled = BTreeSet::new();
		unhandled.insert((0, start_pos));
		while let Some((cost, pos)) = unhandled.pop_first() {
			if fields[pos.0][pos.1] {
				continue;
			}
			if !handled.insert(pos) {
				continue;
			}
			if pos == end_pos {
				return Some(cost);
			}
			let neighs = neighs(pos, self.height, self.width);
			for new_pos in neighs {
				unhandled.insert((cost + 1, new_pos));
			}
		}

		None
	}
	fn first_byte_to_block(&self) -> (usize, usize) {
		let mut fields = vec![vec![false; self.width]; self.height];
		for corr in self.corruptions.iter() {
			fields[corr.0][corr.1] = true;
			if self.min_steps_with(&fields).is_none() {
				return *corr;
			}
		}
		panic!("never blocked")
	}
}
