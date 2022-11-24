use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let flipped = do_flips(&cmds);
	println!("Flipped count: {}", flipped.len());
	let flipped_after = n_days(100, flipped);
	println!("Flipped after 100 days: {}", flipped_after.len());
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
	NorthEast,
	NorthWest,
	East,
	West,
	SouthEast,
	SouthWest,
}

fn parse_line(line :&str) -> Vec<Dir> {
	let mut line = line.as_bytes();
	let mut res = Vec::new();
	while !line.is_empty() {
		if line.starts_with(b"e") {
			res.push(Dir::East);
			line = &line[1..];
			continue;
		}
		if line.starts_with(b"w") {
			res.push(Dir::West);
			line = &line[1..];
			continue;
		}

		if line.starts_with(b"ne") {
			res.push(Dir::NorthEast);
		} else if line.starts_with(b"nw") {
			res.push(Dir::NorthWest);
		} else if line.starts_with(b"se") {
			res.push(Dir::SouthEast);
		} else if line.starts_with(b"sw") {
			res.push(Dir::SouthWest);
		} else {
			panic!("Invalid line starters: {:02x} {:02x}", line[0], line[1]);
		}

		line = &line[2..];
	}
	res
}

fn parse(input :&str) -> Vec<Vec<Dir>> {
	input.lines()
		.map(parse_line)
		.collect::<Vec<_>>()
}

fn coords_for_cmds(line :&[Dir]) -> (i16, i16) {
	let mut x = 0;
	let mut y = 0;
	for dir in line {
		match dir {
			Dir::NorthEast => {
				x -= 1;
				y += 1;
			},
			Dir::NorthWest => {
				y += 1;
			},
			Dir::East => {
				x -= 1;
			},
			Dir::West => {
				x += 1;
			},
			Dir::SouthEast => {
				y -= 1;
			},
			Dir::SouthWest => {
				x += 1;
				y -= 1;
			},
		}
	}
	(x, y)
}

fn do_flips(cmds :&[Vec<Dir>]) -> HashSet<(i16, i16)> {
	let mut flipped = HashSet::new();
	for line in cmds {
		let coords = coords_for_cmds(line);
		if flipped.contains(&coords) {
			flipped.remove(&coords);
		} else {
			flipped.insert(coords);
		}
	}
	flipped
}

fn neighs_contained(x :i16, y :i16, hs :&HashSet<(i16, i16)>) -> u8 {
	let mut ret = 0;
	// NorthEast
	ret += hs.contains(&(x - 1, y + 1)) as u8;
	// NorthWest
	ret += hs.contains(&(x, y + 1)) as u8;
	// East
	ret += hs.contains(&(x - 1, y)) as u8;
	// West
	ret += hs.contains(&(x + 1, y)) as u8;
	// SouthEast
	ret += hs.contains(&(x, y - 1)) as u8;
	// SouthWest
	ret += hs.contains(&(x + 1, y - 1)) as u8;
	ret
}

fn day(old :HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
	let x_min = old.iter().map(|(x, _y)| x).min().unwrap() - 1;
	let x_max = old.iter().map(|(x, _y)| x).max().unwrap() + 1;
	let y_min = old.iter().map(|(_x, y)| y).min().unwrap() - 1;
	let y_max = old.iter().map(|(_x, y)| y).max().unwrap() + 1;
	let mut new = HashSet::new();
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			let cont = neighs_contained(x, y, &old);
			let was_black = old.contains(&(x, y));
			if was_black && (cont == 0 || cont > 2) {
				// Flip to white
				continue;
			}
			if was_black || cont == 2 {
				// Tile will be black
				new.insert((x, y));
			}
		}
	}
	new
}

fn n_days(n :usize, old :HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
	let mut hs = old;
	for _ in 0..n {
		hs = day(hs);
	}
	hs
}
