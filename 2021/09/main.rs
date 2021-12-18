use std::collections::HashMap;
const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("Low points risk sum: {}", low_points_risk_sum(INPUT));
	println!("Product of the largest 3 basin sizes: {}", largest_3_basins_product(INPUT));
}

fn parse_field(input :&str) -> Vec<Vec<u8>> {
	input.trim().lines()
		.map(|l| l.trim().chars()
			.map(|c| {
				if !('0'..='9').contains(&c) {
					panic!("Not a digit: {}", c);
				}
				(c as u32 - '0' as u32) as u8
			})
			.collect::<Vec<_>>()
		)
		.collect::<Vec<_>>()
}

fn for_low_points(field :&[Vec<u8>], mut lowpoint_fn :impl FnMut(u8, usize, usize)) {
	for (y, line) in field.iter().enumerate() {
		for (x, &v) in line.iter().enumerate() {
			let mut cont = false;
			for_neighs(line.len(), field.len(), x, y,
				|xn, yn| if field[yn][xn] <= v {
					cont = true;
				});
			if cont {
				continue;
			}
			// Lowpoint.
			lowpoint_fn(v, x, y);
		}
	}
}

fn for_neighs(width :usize, height :usize, x :usize, y :usize, mut neigh_fn :impl FnMut(usize, usize)) {
	if y > 0 {
		neigh_fn(x, y - 1);
	}
	if y < height - 1 {
		neigh_fn(x, y + 1);
	}
	if x > 0 {
		neigh_fn(x - 1, y);
	}
	if x < width - 1 {
		neigh_fn(x + 1, y);
	}
}

fn low_points_risk_sum(input :&str) -> u32 {
	let field = parse_field(input);
	let mut risk_level_sum = 0;
	for_low_points(&field, |v, _x, _y| {
		let risk_level = v + 1;
		risk_level_sum += risk_level as u32;
	});
	risk_level_sum
}

macro_rules! dprintln {
	($($args:expr),*) => {
		//println!($($args),*);
	};
}

fn find_basin_sizes(field :&[Vec<u8>]) -> Vec<usize> {
	let mut basin_map = vec![vec![None; field[0].len()]; field.len()];
	let mut basin_sizes = HashMap::<(usize, usize), usize>::new();
	for_low_points(&field, |_v, x, y| {
		dprintln!("low point ({},{})", x, y);
		basin_map[y][x] = Some((x, y));
		basin_sizes.insert((x, y), 1);
	});

	for level in 0..9 {
		for (y, line) in field.iter().enumerate() {
			for (x, &v) in line.iter().enumerate() {
				if v != level {
					continue;
				}

				let mut basin = None;
				for_neighs(line.len(), field.len(), x, y,
					|xn, yn| {
						if field[yn][xn] > v {
							return;
						}
						dprintln!("considering smaller neighbour ({}, {}) => {:?}", xn, yn, basin_map[yn][xn]);
						if let Some(b) = basin_map[yn][xn] {
							basin = Some(b);
						}
					});
				if let Some(b) = basin {
					basin_map[y][x] = Some(b);
					dprintln!("increase basin");
					*basin_sizes.get_mut(&b).unwrap() += 1;
				}
			}
		}
	}
	let mut basin_sizes :Vec<usize> = basin_sizes.iter()
		.map(|s| *s.1)
		.collect();
	basin_sizes.sort();
	basin_sizes.reverse();

	basin_sizes
}

fn largest_3_basins_product(input :&str) -> usize {
	let field = parse_field(input);
	let basin_sizes = find_basin_sizes(&field);
	basin_sizes[0..3].iter().product()
}
