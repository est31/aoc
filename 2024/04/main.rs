use std::collections::{HashMap, HashSet};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("xmas count: {}", count_xmas(INPUT));
	println!("x_mas count: {}", count_x_mas(INPUT));
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

	let mut counter_s = CounterXmas::new();
	let mut counter_d = CounterXmas::new();

	dprint!("RIGHT HOR\n");
	count_for_fn_st(&mut counter_s, |i, j| (i, j), &chars, height, width);
	count_for_fn_di(&mut counter_d, |i, j| (i, j), &chars, height, width);
	dprint!("LEFT HOR\n");
	count_for_fn_st(&mut counter_s, |i, j| (i, width - 1 - j), &chars, height, width);
	count_for_fn_di(&mut counter_d, |i, j| (i, width - 1 - j), &chars, height, width);
	dprint!("UP VERT\n");
	count_for_fn_st(&mut counter_s, |i, j| (j, i), &chars, width, height);
	count_for_fn_di(&mut counter_d, |i, j| (height - 1 - j, i), &chars, width, height);
	dprint!("DOWN VERT\n");
	count_for_fn_st(&mut counter_s, |i, j| (height - 1 - j, width - 1 - i), &chars, width, height);
	count_for_fn_di(&mut counter_d, |i, j| (height - 1 - j, width - 1 - i), &chars, width, height);

	counter_s.count + counter_d.count
}

struct CounterXmas {
	count: u32,
	state: u8,
}

impl CounterXmas {
	fn new() -> Self {
		Self {
			count: 0,
			state: 0,
		}
	}
}

impl Counter for CounterXmas {
	fn end_word(&mut self) {
		dprint!("\n");
		self.state = 0;
	}
	fn feed(&mut self, ch: char, _pos: (usize, usize)) {
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

trait Counter {
	fn end_word(&mut self);
	fn feed(&mut self, ch: char, pos: (usize, usize));
}

struct CounterMas {
	count: HashSet<(usize, usize)>,
	state: (u8, Option<(usize, usize)>),
	a_positions: HashMap<(usize, usize), usize>,
}

impl CounterMas {
	fn new() -> Self {
		Self {
			count: HashSet::new(),
			state: (0, None),
			a_positions: HashMap::new(),
		}
	}
}

impl Counter for CounterMas {
	fn end_word(&mut self) {
		dprint!("\n");
		self.state = (0, None);
	}
	fn feed(&mut self, ch: char, pos: (usize, usize)) {
		self.state = match (self.state.0, ch) {
			(_, 'M') => (1, None),
			(1, 'A') => (2, Some(pos)),
			(2, 'S') => (3, self.state.1),
			_ => (0, None),
		};
		dprint!("    '{ch}'{}", self.state.0);
		if self.state.0 == 3 {
			let a_pos = self.state.1.unwrap();
			let num = self.a_positions.entry(a_pos).or_default();
			*num += 1;
			dprint!(" XX{num}XX ");
			if *num == 2 {
				self.count.insert(a_pos);
			}
			self.end_word();
		}
	}
}

fn count_x_mas(s: &str) -> u32 {
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

	let mut counter_s = CounterMas::new();
	let mut counter_d = CounterMas::new();

	dprint!("RIGHT HOR\n");
	count_for_fn_st(&mut counter_s, |i, j| (i, j), &chars, height, width);
	count_for_fn_di(&mut counter_d, |i, j| (i, j), &chars, height, width);
	dprint!("LEFT HOR\n");
	count_for_fn_st(&mut counter_s, |i, j| (i, width - 1 - j), &chars, height, width);
	count_for_fn_di(&mut counter_d, |i, j| (i, width - 1 - j), &chars, height, width);
	dprint!("UP VERT\n");
	count_for_fn_st(&mut counter_s, |i, j| (j, i), &chars, width, height);
	count_for_fn_di(&mut counter_d, |i, j| (height - 1 - j, i), &chars, width, height);
	dprint!("DOWN VERT\n");
	count_for_fn_st(&mut counter_s, |i, j| (height - 1 - j, width - 1 - i), &chars, width, height);
	count_for_fn_di(&mut counter_d, |i, j| (height - 1 - j, width - 1 - i), &chars, width, height);

	dprint!("  count straight: {}\n", counter_s.count.len());
	dprint!("  count diagonal: {}\n", counter_d.count.len());
	let combined = &counter_s.count | &counter_d.count;
	combined.len() as _
}

fn count_for_fn_st(counter: &mut impl Counter, f: impl Fn(usize, usize) -> (usize, usize), chars: &[Vec<char>], i_lim: usize, j_lim: usize) {
	// Straight words
	dprint!("  straight:\n");
	for i in 0..i_lim {
		for j in 0..j_lim {
			let pos = f(i, j);
			let ch = chars[pos.0][pos.1];
			counter.feed(ch, pos);
		}
		counter.end_word();
	}
}

fn count_for_fn_di(counter: &mut impl Counter, f: impl Fn(usize, usize) -> (usize, usize), chars: &[Vec<char>], i_lim: usize, j_lim: usize) {
	// Diagonal words
	dprint!("  first diag:\n");
	for i in 0..i_lim {
		for j in 0..j_lim {
			if i + j >= i_lim {
				break;
			}
			let pos = f(i + j, j);
			let ch = chars[pos.0][pos.1];
			counter.feed(ch, pos);
		}
		counter.end_word();
	}

	dprint!("  next diag:\n");
	for j in 1..j_lim {
		for i in 0..i_lim {
			if i + j >= j_lim {
				break;
			}
			let pos = f(i, i + j);
			let ch = chars[pos.0][pos.1];
			counter.feed(ch, pos);
		}
		counter.end_word();
	}
}
