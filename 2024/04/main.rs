
const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("xmas count: {}", count_xmas(INPUT));
}

fn count_xmas(s: &str) -> u32 {
	let chars = s.lines()
		.filter(|l| !l.is_empty())
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
	/*sum += count_for_fn(|i, j| {
		println!("  ({i:02}, {j:02})");
		chars[i][j]
	}, height, width);*/

	sum += count_for_fn(|i, j| chars[i][j], height, width);
	sum += count_for_fn(|i, j| chars[i][width - 1 - j], height, width);
	sum += count_for_fn(|i, j| chars[height - 1 - j][width - 1 - i], width, height);
	sum += count_for_fn(|i, j| chars[height - 1 - j][i], width, height);
	sum
}

struct Counter {
	count: u32,
	state: u8,
}

impl Counter {
	fn new() -> Self {
		Counter {
			count: 0,
			state: 0,
		}
	}
	fn end_word(&mut self) {
		self.state = 0;
	}
	fn feed(&mut self, ch: char) {
		self.state = match (self.state, ch) {
			(_, 'X') => 1,
			(1, 'M') => 2,
			(2, 'A') => 3,
			(3, 'S') => 4,
			_ => 0,
		};
		if self.state == 4 {
			println!("    XMAS!");
			self.count += 1;
			self.state = 0;
		}
	}
}

fn count_for_fn(f: impl Fn(usize, usize) -> char, i_lim: usize, j_lim: usize) -> u32 {
	let mut counter = Counter::new();
	// Straight words
	println!("  straight:");
	for i in 0..i_lim {
		for j in 0..j_lim {
			let ch = f(i, j);
			counter.feed(ch);
		}
		counter.end_word();
	}

	// Diagonal words
	println!("  first diag:");
	for i in 0..i_lim {
		for j in 0..j_lim {
			if i + j >= i_lim {
				break;
			}
			let ch = f(i + j, j);
			counter.feed(ch);
		}
		counter.end_word();
	}

	println!("  next diag:");
	for j in 1..j_lim {
		for i in 0..i_lim {
			if i + j >= i_lim {
				break;
			}
			let ch = f(i + j, j);
			counter.feed(ch);
		}
		counter.end_word();
	}
	println!("  partial count: {}", counter.count);
	counter.count
}
