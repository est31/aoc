use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sl = parse_slice(INPUT);
	let sc = six_steps(&sl, false);
	println!("Active cubes after six steps: {}", sc.len());
	let sc4 = six_steps(&sl, true);
	println!("Active cubes after six steps in 4 dims: {}", sc4.len());
}

type Scene = HashSet<(isize, isize, isize, isize)>;

fn parse_slice(input :&str) -> Scene {
	input.lines().enumerate()
		.map(|(i, l)| {
			l.chars().enumerate()
				.flat_map(move |(j, ch)| match ch {
					'.' => None,
					'#' => Some((i as _, j as _, 0, 0)),
					_ => panic!("Unexpected char {ch}"),
				})
		})
		.flatten()
		.collect()
}

fn get_min_max(sc :&Scene, f :impl Fn((isize, isize, isize, isize)) -> isize) -> (isize, isize) {
	let min = sc.iter()
		.copied()
		.map(&f)
		.min()
		.unwrap();
	let max = sc.iter()
		.copied()
		.map(&f)
		.max()
		.unwrap();
	(min - 1, max + 1)
}

fn count_around(sc :&Scene, x :isize, y :isize, z :isize, w :Option<isize>) -> usize {
	let mut cnt = 0;
	for xa in (x - 1)..=(x + 1) {
		for ya in (y - 1)..=(y + 1) {
			for za in (z - 1)..=(z + 1) {
				if let Some(w) = w {
					for wa in (w - 1)..=(w + 1) {
						if (x, y, z, w) == (xa, ya, za, wa) {
							continue;
						}
						cnt += sc.contains(&(xa, ya, za, wa)) as usize;
					}
				} else {
					if (x, y, z) == (xa, ya, za) {
						continue;
					}
					cnt += sc.contains(&(xa, ya, za, 0)) as usize;
				}
			}
		}
	}
	cnt
}

fn step(old :&Scene, four_dims :bool) -> Scene {
	let (x_min, x_max) = get_min_max(old, |tup| tup.0);
	let (y_min, y_max) = get_min_max(old, |tup| tup.1);
	let (z_min, z_max) = get_min_max(old, |tup| tup.2);
	let (w_min, w_max) = if four_dims {
		get_min_max(old, |tup| tup.3)
	} else {
		(0, 0)
	};
	let mut new_sc = Scene::new();
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			for z in z_min..=z_max {
				for w in w_min..=w_max {
					let active = old.contains(&(x, y, z, w));
					let wo = four_dims.then_some(w);
					let around = count_around(old, x, y, z, wo);
					match (active, around) {
						(true, 2) | (true, 3) | (false, 3) => {
							new_sc.insert((x, y, z, w));
						},
						_ => (),
					}
				}
			}
		}
	}
	new_sc
}

fn six_steps(old :&Scene, four_dims :bool) -> Scene {
	let mut sc = step(old, four_dims);
	for _ in 1..6 {
		sc = step(&sc, four_dims);
	}
	sc
}

#[cfg(test)]
fn print(sc :&Scene) {
	let (x_min, x_max) = get_min_max(sc, |tup| tup.0);
	let (y_min, y_max) = get_min_max(sc, |tup| tup.1);
	let (z_min, z_max) = get_min_max(sc, |tup| tup.2);

	for z in (z_min + 1)..=(z_max - 1) {
		println!("z={z}");
		for x in (x_min + 1)..=(x_max - 1) {
			for y in (y_min + 1)..=(y_max - 1) {
				let ch = if sc.contains(&(x, y, z, 0)) {
					'#'
				} else {
					'.'
				};
				print!("{ch}");
			}
			println!();
		}
	}
}
