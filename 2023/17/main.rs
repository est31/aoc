use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	println!("heat loss: {}", heat_loss(&field));
}

fn parse(input :&str) -> Vec<Vec<u8>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			l.chars()
				.map(|ch| ch.to_digit(10).unwrap() as u8)
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Direction {
	None,
	Right,
	Down,
	Left,
	Up,
}

impl Direction {
	fn is_opposite(&self, other :Direction) -> bool {
		use Direction::*;
		match (*self, other) {
			(Right, Left) | (Left, Right) => true,
			(Up, Down) | (Down, Up) => true,
			_ => false,
		}
	}
}

fn heat_loss(field :&[Vec<u8>]) -> u32 {
	// bases on copy of 2021/15 dijkstra
	let mut visited = HashMap::<_, usize>::new();
	let mut min_heap = BinaryHeap::new();

	let height = field.len();
	let width = field[0].len();

	// Initialize the data for the origin
	let last_directions = (Direction::None, 1);
	*visited.entry(((0, 0), Direction::None)).or_default() = 0;
	min_heap.push((Reverse(0), (0, 0), last_directions));

	let goal = (width - 1, height - 1);

	// Run dijkstra
	while let Some((Reverse(cost), (x, y), last_directions)) = min_heap.pop() {
		//println!("cost: {cost}, pos: ({x}, {y}), ld: {last_directions:?}");
		macro_rules! neigh {
			($n_x:expr, $n_y:expr, $dir:expr) => {
				let vis = visited.get(&(($n_x, $n_y), $dir));
				let new_directions = if last_directions.0 == $dir {
					($dir, last_directions.1 + 1)
				} else {
					($dir, 1)
				};
				let not_opposite = !last_directions.0.is_opposite($dir);
				let vis = if let Some(l) = vis {
					// Been visited in that direction before
					l <= &new_directions.1
				} else {
					// Not been visited in that direction before
					false
				};
				// Remember: None (not visited yet in that direction) is
				if not_opposite && new_directions.1 <= 3 && !vis {
					visited.insert((($n_x, $n_y), $dir), new_directions.1);
					let cost_add = field[$n_y][$n_x];
					let n_cost = cost + cost_add as u32;
					// Seach concluded
					if ($n_x, $n_y) == goal {
						return n_cost;
					}
					min_heap.push((Reverse(n_cost), ($n_x, $n_y), new_directions));
				}
			};
		}
		if x > 0 {
			neigh!(x - 1, y, Direction::Left);
		}
		if x < width - 1 {
			neigh!(x + 1, y, Direction::Right);
		}
		if y > 0 {
			neigh!(x, y - 1, Direction::Up);
		}
		if y < height - 1 {
			neigh!(x, y + 1, Direction::Down);
		}
	}

	panic!("search ended without finding goal");
}
