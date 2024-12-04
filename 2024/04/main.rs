
const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("xmas count: {}", count_xmas(INPUT));
}

fn count_xmas(s: &str) -> u32 {
	let chars = s.lines()
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<_>>>();
	let height = chars.len();
	if height == 0 {
		return 0;
	}
	let width = chars[0].len();
	if width == 0 {
		return 0;
	}
	let mut sum = 0;
	sum += count_for_fn(|i, j| chars[i][j], height, width);
	sum += count_for_fn(|i, j| chars[i][width - 1 - j], height, width);
	sum += count_for_fn(|i, j| chars[j][i], width, height);
	sum += count_for_fn(|i, j| chars[height - 1 - j][i], width, height);
	sum
}

fn count_for_fn(f: impl Fn(usize, usize) -> char, i_lim: usize, j_lim: usize) -> u32 {
	let mut state = 0;
	let mut count = 0;
	for i in 0..i_lim {
		for j in 0..j_lim {
			let ch = f(i, j);
			state = match (state, ch) {
				(_, 'X') => 1,
				(1, 'M') => 2,
				(2, 'A') => 3,
				(3, 'S') => 4,
				_ => 0,
			};
			if state == 4 {
				count += 1;
				state = 0;
			}
		}
	}
	count
}
