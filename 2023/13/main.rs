const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let patterns = parse(INPUT);
	println!("summarized: {}", summarize_notes(&patterns));
	println!("summarized (smudge): {}", summarize_notes_smudge(&patterns));
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

fn columns_neq(pattern :&[Vec<bool>], ci :usize, cj :usize) -> usize {
	pattern.iter()
		.filter(|l| l[ci] != l[cj])
		.count()
}

fn lines_neq(pattern :&[Vec<bool>], li :usize, lj :usize) -> usize {
	pattern[li].iter()
		.zip(pattern[lj].iter())
		.filter(|(v, w)| v != w)
		.count()
}

fn search(pattern :&[Vec<bool>], limit :usize, smudge_tgt :usize, neq_fn :impl Fn(&[Vec<bool>], usize, usize) -> usize) -> Option<usize> {
	for i in 1..limit {
		let mut smudge_budget = smudge_tgt;
		let mut check_fn = |i, j| {
			let neq = neq_fn(pattern, i, j);
			match smudge_budget.checked_sub(neq) {
				Some(new_tgt) => {
					smudge_budget = new_tgt;
					true
				},
				None => false,
			}
		};
		if check_fn(i, i - 1) {
			let mut ex = 1;
			let found_mirror = loop {
				let ix = i + ex;
				let Some(jx) = (i - 1).checked_sub(ex) else {
					break true
				};
				if ix >= limit {
					break true;
				}
				if !check_fn(ix, jx) {
					break false;
				}
				ex += 1;
			};
			if found_mirror && smudge_budget == 0 {
				return Some(i);
			}
		}
	}
	None
}

fn summarize_notes(patterns :&[Vec<Vec<bool>>]) -> u32 {
	summarize_notes_generic(patterns, 0)
}

fn summarize_notes_smudge(patterns :&[Vec<Vec<bool>>]) -> u32 {
	summarize_notes_generic(patterns, 1)
}

fn summarize_notes_generic(patterns :&[Vec<Vec<bool>>], smudge_tgt :usize) -> u32 {
	let mut sum = 0;
	for pattern in patterns {
		let width = pattern[0].len();
		if let Some(i) = search(pattern, width, smudge_tgt, columns_neq) {
			sum += i;
			continue;
		}
		let height = pattern.len();
		if let Some(i) = search(pattern, height, smudge_tgt, lines_neq) {
			sum += i * 100;
			continue;
		}
	}
	sum as u32
}
