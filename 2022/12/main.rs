use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let fld = parse(INPUT);
	let steps = steps_to_goal(fld);
	println!("Steps to goal: {steps}");
}

fn parse(input :&str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
	let mut start = None;
	let mut end = None;
	let field = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.enumerate()
		.map(|(y, l)| {
			l.chars()
				.enumerate()
				.map(|(x, ch)| {
					match ch {
						'S' => {
							start = Some((x, y));
							0
						},
						'E' => {
							end = Some((x, y));
							b'z' - b'a'
						},
						'a'..='z' => ch as u8 - b'a',
						_ => panic!("invalid char '{ch}'"),
					}
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	(field, start.unwrap(), end.unwrap())
}

fn steps_to_goal((field, start, end) :(Vec<Vec<u8>>, (usize, usize), (usize, usize))) -> usize {
	let mut visited = vec![vec![false; field[0].len()]; field.len()];
	let mut active = HashSet::new();
	active.insert(start);
	let mut cnt = 0;
	while !active.is_empty() {
		for (ax, ay) in std::mem::take(&mut active) {
			if (ax, ay) == end {
				return cnt;
			}
			let val = field[ay][ax];
			let ax = ax as isize;
			let ay = ay as isize;
			for (nx, ny) in [(ax + 1, ay), (ax - 1, ay), (ax, ay - 1), (ax, ay + 1)] {
				if nx < 0 || ny < 0 { continue; }
				let nx = nx as usize;
				let ny = ny as usize;
				if nx >= field[0].len() || ny >= field.len() { continue; }
				if visited[ny][nx] { continue; }
				let nval = field[ny][nx];
				if nval > val + 1 { continue; }
				visited[ny][nx] = true;
				active.insert((nx, ny));
			}
		}
		cnt += 1;
	}
	panic!("no path found!");
}
