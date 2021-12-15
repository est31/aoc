use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Display;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct StrErr(String);

impl<T :Display> From<T> for StrErr {
	fn from(v :T) -> Self {
		StrErr(format!("{}", v))
	}
}

type Result<T> = std::result::Result<T, StrErr>;

fn main() -> Result<()> {
	let cave = Cave::parse(INPUT)?;
	let min_cost = cave.compute_minimum_cost();
	println!("Minimum cost is: {}", min_cost);
	let min_cost_big = cave.compute_minimum_cost_big();
	println!("Minimum cost of the big cave is: {}", min_cost_big);
	Ok(())
}

struct Cave {
	width :usize,
	cave :Vec<u8>,
}

impl Cave {
	fn parse(input :&str) -> Result<Self> {
		let mut cave = Vec::new();
		let mut width = None;
		for l in input.lines() {
			let l = l.trim();
			if l.len() == 0 {
				break;
			}
			if let Some(w) = width {
				if w != l.len() {
					Err(format!("Non rectangular cave. Expected width {}, got {}", w, l.len()))?
				}
			} else {
				width = Some(l.len());
			}
			for c in l.chars() {
				if !('0'..='9').contains(&c) {
					Err(format!("Character '{}' is not a digit", c))?
				}
				cave.push((c as u8) - '0' as u8);
			}
		}
		Ok(Self {
			width : width.unwrap_or(0),
			cave,
		})
	}
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.cave.len() / self.width
	}
	fn compute_minimum_cost(&self) -> u64 {
		if self.cave.len() <= 1 {
			return 0;
		}

		dijkstra_2d(self.width(), self.height(), |x, y| {
			let idx = x + self.width() * y;
			self.cave[idx] as u64
		}, [self.width() - 1, self.height() - 1])
	}
	fn compute_minimum_cost_big(&self) -> u64 {
		if self.cave.len() <= 1 {
			return 0;
		}

		dijkstra_2d(self.width() * 5, self.height() * 5, |x, y| {
			let cave_x = x / self.width();
			let cave_y = y / self.height();
			let cave_add = cave_x + cave_y;
			let idx = x % self.width() + (y % self.height()) * self.width();
			let cost_add = self.cave[idx] as u64 + cave_add as u64;
			(cost_add - 1) % 9 + 1
		}, [self.width() * 5 - 1, self.height() * 5 - 1])
	}
	#[cfg(test)]
	fn dump_map_big(&self) -> String {
		let cost_fn = |x, y| {
			let cave_x = x / self.width();
			let cave_y = y / self.height();
			let cave_add = cave_x + cave_y;
			let idx = x % self.width() + (y % self.height()) * self.width();
			let cost_add = self.cave[idx] as u64 + cave_add as u64;
			(cost_add - 1) % 9 + 1
		};
		let mut s = String::new();
		for y in 0..(self.width() * 5) {
			for x in 0..(self.height() * 5) {
				let cost = cost_fn(x, y);
				assert!(cost < 10 && cost > 0, "cost {} outside 1..10", cost);
				s += &format!("{}", cost);
			}
			s += "\n";
		}
		s
	}
}

fn dijkstra_2d(width :usize, height :usize,
		cost_add :impl Fn(usize, usize) -> u64, goal :[usize; 2]) -> u64 {
	let mut visited = vec![false; width * height];
	let mut min_heap = BinaryHeap::new();

	// Initialize the data for the origin
	visited[0] = true;
	min_heap.push((Reverse(0u64), [0usize, 0]));

	// Run dijkstra
	while let Some((Reverse(cost), [x, y])) = min_heap.pop() {
		macro_rules! neigh {
			($n_x:expr, $n_y:expr) => {
				let n_idx = $n_x + $n_y * width;
				if !visited[n_idx] {
					visited[n_idx] = true;
					let n_cost = cost + cost_add($n_x, $n_y);
					// Seach concluded
					if [$n_x, $n_y] == goal {
						return n_cost;
					}
					min_heap.push((Reverse(n_cost), [$n_x, $n_y]));
				}
			};
		}
		if x > 0 {
			neigh!(x - 1, y);
		}
		if x < width - 1 {
			neigh!(x + 1, y);
		}
		if y > 0 {
			neigh!(x, y - 1);
		}
		if y < height - 1 {
			neigh!(x, y + 1);
		}
	}

	panic!("Search ended without encountering goal vertex");
}
