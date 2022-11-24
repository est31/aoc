use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let flipped_count = do_flips(&cmds);
	println!("Flipped count: {flipped_count}");
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

fn do_flips(cmds :&[Vec<Dir>]) -> u64 {
	let mut flipped = HashSet::new();
	for line in cmds {
		let coords = coords_for_cmds(line);
		if flipped.contains(&coords) {
			flipped.remove(&coords);
		} else {
			flipped.insert(coords);
		}
	}
	flipped.len() as u64
}
