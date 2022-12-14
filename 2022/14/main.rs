use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let cap = sand_holding_cap(&cmds, false);
	println!("Sand holding cap: {cap}");
	let cap_floor = sand_holding_cap(&cmds, true);
	println!("Sand holding cap with floor: {cap_floor}");
}

type DrawCmd = [(u32, u32); 2];

fn parse(input :&str) -> Vec<DrawCmd> {
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut res = Vec::new();
	for l in lines {
		let mut prev = None;
		for seg in l.split(" -> ") {
			let mut coords = seg.split(',')
				.map(|v| u32::from_str(v).unwrap());
			let x = coords.next().unwrap();
			let y = coords.next().unwrap();
			let pos = (x, y);
			if let Some(pr) = prev.take() {
				res.push([pr, pos]);
			}
			prev = Some(pos);
		}
	}
	res
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Field {
	Empty,
	Rock,
	Sand,
}

fn build_scene(cmds :&[DrawCmd], with_floor :bool) -> Vec<Vec<Field>> {
	let width = *cmds.iter()
		.map(|[(sx, _), (ex, _)]| [sx, ex])
		.flatten()
		.max()
		.unwrap() as usize + 1;
	let height = *cmds.iter()
		.map(|[(_, sy), (_, ey)]| [sy, ey])
		.flatten()
		.max()
		.unwrap() as usize + 1;
	let (height, width) = if with_floor {
		(height + 2, (height + 500 + 2).max(width))
	} else {
		(height, width)
	};
	let mut scene = vec![vec![Field::Empty; width as usize]; height];
	for [start, end] in cmds {
		let min_x = start.0.min(end.0);
		let min_y = start.1.min(end.1);
		let max_x = start.0.max(end.0);
		let max_y = start.1.max(end.1);
		for x in min_x..=max_x {
			for y in min_y..=max_y {
				scene[y as usize][x as usize] = Field::Rock;
			}
		}
	}
	if with_floor {
		for x in 0..(width as usize) {
			scene.last_mut().unwrap()[x] = Field::Rock;
		}
	}
	scene
}

fn put_sand(scene :&mut [Vec<Field>]) -> bool {
	let mut x = 500;
	let mut y = 0;
	loop {
		let xl = [x, x - 1, x + 1];
		if y + 1 >= scene.len() {
			// Fall through
			return false;
		}
		let found = xl.into_iter()
		.find(|nx| scene[y + 1][*nx] == Field::Empty);
		if let Some(nx) = found {
			x = nx;
			y += 1;
		} else {
			// No move possible
			if y == 0 && scene[y][x] != Field::Empty {
				// Sand can't fall further
				return false;
			}
			// put the sand to rest.
			scene[y][x] = Field::Sand;
			return true;
		}
	}
}

/*
#[cfg(test)]
fn print_scene(scene :&[Vec<Field>]) {
	for line in scene {
		for (x, field) in line.iter().enumerate() {
			if x < 400 {
				continue;
			}
			let ch = match field {
				Field::Empty => '.',
				Field::Rock => '#',
				Field::Sand => 'O',
			};
			print!("{ch}");
		}
		println!();
	}
}*/

fn sand_holding_cap(cmds :&[DrawCmd], with_floor :bool) -> u32 {
	let mut scene = build_scene(cmds, with_floor);
	//print_scene(&scene);
	for sand_put in 0.. {
		if !put_sand(&mut scene) {
			return sand_put;
		}
	}
	unreachable!()
}
