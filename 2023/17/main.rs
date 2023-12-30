use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	println!("heat loss: {}", heat_loss(&field));
	println!("heat loss ultra: {}", heat_loss_ultra(&field));
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
	heat_loss_generic(field, 0, 3)
}

fn heat_loss_ultra(field :&[Vec<u8>]) -> u32 {
	heat_loss_generic(field, 3, 10)
}

fn heat_loss_generic(field :&[Vec<u8>], min_walk_add :usize, max_walk :usize) -> u32 {
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
			($diff:expr, $x_diff:expr, $y_diff:expr, $dir:expr, $only_same_dir:expr) => {
				let mut n_x = x as isize;
				let mut n_y = y as isize;
				let mut n_cost = cost;
				for _d in 0..$diff {
					n_x += $x_diff;
					n_y += $y_diff;
					let n_x = n_x as usize;
					let n_y = n_y as usize;
					let vis = visited.get(&((n_x, n_y), $dir));
					let new_directions = if last_directions.0 == $dir {
						($dir, last_directions.1 + 1)
					} else {
						($dir, 1)
					};
					let dir_allowed = if $only_same_dir {
						last_directions.0 == $dir
					} else {
						let not_opposite = !last_directions.0.is_opposite($dir);
						not_opposite
					};
					let vis = if let Some(l) = vis {
						// Been visited in that direction before
						if min_walk_add == 0 || $only_same_dir {
							l <= &new_directions.1
						} else {
							l == &new_directions.1
						}
					} else {
						// Not been visited in that direction before
						false
					};
					if dir_allowed && new_directions.1 <= max_walk && !vis {
						visited.insert(((n_x, n_y), $dir), new_directions.1);
						let cost_add = field[n_y][n_x];
						n_cost += cost_add as u32;
						// Seach concluded
						if (n_x, n_y) == goal {
							return n_cost;
						}
						min_heap.push((Reverse(n_cost), (n_x, n_y), new_directions));
					}
				}
			};
		}
		macro_rules! neighs {
			($dist:expr, $only_same_dir:expr) => {
				if x >= $dist {
					neigh!($dist, -1, 0, Direction::Left, $only_same_dir);
				}
				if x < width - $dist {
					neigh!($dist, 1, 0, Direction::Right, $only_same_dir);
				}
				if y >= $dist {
					neigh!($dist, 0, -1, Direction::Up, $only_same_dir);
				}
				if y < height - $dist {
					neigh!($dist, 0, 1, Direction::Down, $only_same_dir);
				}
			};
		}
		if min_walk_add == 0 {
			neighs!(1, false);
		} else {
			neighs!(1, true);
			neighs!(1 + min_walk_add, false);
		}
	}

	panic!("search ended without finding goal");
}
