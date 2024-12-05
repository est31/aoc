
const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("xmas count: {}", count_xmas(INPUT));
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

fn count_xmas(s: &str) -> u32 {
	dprint!("--------------------\n");
	dprint!("{s}");
	dprint!("--------------------\n");
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

	dprint!("RIGHT HOR\n");
	sum += count_for_fn_st(|i, j| chars[i][j], height, width);
	sum += count_for_fn_di(|i, j| chars[i][j], height, width);
	dprint!("LEFT HOR\n");
	sum += count_for_fn_st(|i, j| chars[i][width - 1 - j], height, width);
	sum += count_for_fn_di(|i, j| chars[i][width - 1 - j], height, width);
	dprint!("UP VERT\n");
	sum += count_for_fn_st(|i, j| chars[j][i], width, height);
	sum += count_for_fn_di(|i, j| chars[height - 1 - j][i], width, height);
	dprint!("DOWN VERT\n");
	sum += count_for_fn_st(|i, j| chars[height - 1 - j][width - 1 - i], width, height);
	sum += count_for_fn_di(|i, j| chars[height - 1 - j][width - 1 - i], width, height);
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
		dprint!("\n");
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
		dprint!("    '{ch}'{}", self.state);
		if self.state == 4 {
			self.count += 1;
			self.state = 0;
		}
	}
}

fn count_for_fn_st(f: impl Fn(usize, usize) -> char, i_lim: usize, j_lim: usize) -> u32 {
	let mut counter = Counter::new();
	// Straight words
	dprint!("  straight:\n");
	for i in 0..i_lim {
		for j in 0..j_lim {
			let ch = f(i, j);
			counter.feed(ch);
		}
		counter.end_word();
	}
	dprint!("  partial count: {}\n", counter.count);
	counter.count
}

fn count_for_fn_di(f: impl Fn(usize, usize) -> char, i_lim: usize, j_lim: usize) -> u32 {
	let mut counter = Counter::new();
	// Diagonal words
	dprint!("  first diag:\n");
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

	dprint!("  next diag:\n");
	for j in 1..j_lim {
		for i in 0..i_lim {
			if i + j >= j_lim {
				break;
			}
			let ch = f(i, i + j);
			counter.feed(ch);
		}
		counter.end_word();
	}
	dprint!("  partial count: {}\n", counter.count);
	counter.count
}
