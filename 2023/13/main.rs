const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let patterns = parse(INPUT);
	println!("summarized: {}", summarize_notes(&patterns));
}

fn parse(input :&str) -> Vec<Vec<Vec<bool>>> {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let mut patterns = Vec::new();
	let mut cur_pattern = Vec::new();
	while let Some(line) = lines.next() {
		if line.is_empty() {
			patterns.push(std::mem::take(&mut cur_pattern));
			continue;
		}
		let bools = line.chars()
			.map(|c| match c {
				'#' => true,
				'.' => false,
				_ => panic!("Unsupported character '{c}' in '{line}'"),
			})
			.collect::<Vec<_>>();
		cur_pattern.push(bools);
	}
	if cur_pattern.len() > 0 {
		patterns.push(cur_pattern);
	}
	patterns
}

fn columns_equal(pattern :&[Vec<bool>], ci :usize, cj :usize) -> bool {
	pattern.iter()
		.all(|l| l[ci] == l[cj])
}

fn lines_equal(pattern :&[Vec<bool>], li :usize, lj :usize) -> bool {
	pattern[li] == pattern[lj]
}

fn search(pattern :&[Vec<bool>], limit :usize, check_fn :fn(&[Vec<bool>], usize, usize) -> bool) -> Option<usize> {
	for i in 1..limit {
		if check_fn(pattern, i, i - 1) {
			let mut ex = 1;
			let found_mirror = loop {
				let ix = i + ex;
				let Some(jx) = (i - 1).checked_sub(ex) else {
					break true
				};
				if ix >= limit {
					break true;
				}
				if !check_fn(pattern, ix, jx) {
					break false;
				}
				ex += 1;
			};
			if found_mirror {
				return Some(i);
			}
		}
	}
	None
}

fn summarize_notes(patterns :&[Vec<Vec<bool>>]) -> u32 {
	let mut sum = 0;
	for pattern in patterns {
		let width = pattern[0].len();
		if let Some(i) = search(pattern, width, columns_equal) {
			sum += i;
			continue;
		}
		let height = pattern.len();
		if let Some(i) = search(pattern, height, lines_equal) {
			sum += i * 100;
			continue;
		}
	}
	sum as u32
}
