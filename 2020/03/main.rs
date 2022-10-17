const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let scene = parse(INPUT);
	let trees = count_trees(&scene);
	println!("trees with 3/1 slope: {trees}");
	let trees = count_trees_product(&scene);
	println!("trees with 3/1 slope: {trees}");
}

fn parse(input :&str) -> Vec<Vec<bool>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			l.chars()
				.map(|c| {
					match c {
						'#' => true,
						'.' => false,
						_ => panic!("Unknown character '{c}'!"),
					}
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn count_trees(scene :&[Vec<bool>]) -> u16 {
	let width = scene[0].len();
	let height = scene.len();
	let mut trees = 0;
	for y in 0..height {
		let x = (y * 3) % width;
		trees += scene[y][x] as u16;
	}
	trees
}

fn count_trees_product(scene :&[Vec<bool>]) -> u64 {
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	slopes.into_iter()
		.map(|sl| count_trees_for_slope(scene, sl))
		.product()
}

fn count_trees_for_slope(scene :&[Vec<bool>], (xs, ys) :(usize, usize)) -> u64 {
	let width = scene[0].len();
	let height = scene.len();
	let mut trees = 0;
	let mut y = 0;
	let mut x = 0;
	while y < height {
		trees += scene[y][x % width] as u64;
		x += xs;
		y += ys;
	}
	trees
}
