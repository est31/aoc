const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (enhancement_algo, mut image) = parse_algo_input(INPUT);
	image.enhance(&enhancement_algo);
	image.enhance(&enhancement_algo);
	let pixels_lit = image.count_pixels_lit();
	println!("Pixels lit in result after 2 enhancements: {}", pixels_lit);
	for _ in 0..(50 - 2) {
		image.enhance(&enhancement_algo);
	}
	let pixels_lit = image.count_pixels_lit();
	println!("Pixels lit in result after 50 enhancements: {}", pixels_lit);
}

macro_rules! dprint {
	($($args:expr),*) => {
		//print!($($args),*);
	};
}

fn parse_algo_input(input :&str) -> (Vec<bool>, Image) {
	let mut lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let l = lines.next().unwrap();
	let enhancement_algo = l.chars()
		.map(|c| {
			match c {
				'.' => false,
				'#' => true,
				_ => panic!("unexpected char '{}'", c),
			}
		})
		.collect::<Vec<_>>();
	assert_eq!(enhancement_algo.len(), 512);
	let input_image = lines.map(|l| {
		l.chars()
			.map(|c| {
				match c {
					'.' => false,
					'#' => true,
					_ => panic!("unexpected char '{}'", c),
				}
			})
			.collect::<Vec<_>>()
	})
	.collect::<Vec<_>>();
	let mut w = None;
	for l in input_image.iter() {
		if let Some(w) = w {
			assert_eq!(w, l.len());
		}
		w = Some(l.len());
	}
	let image = Image {
		pixels : input_image,
		env : false,
	};
	(enhancement_algo, image)
}

struct Image {
	pixels :Vec<Vec<bool>>,
	env :bool,
}

impl Image {
	fn enhance(&mut self, algo :&[bool]) {
		let input = &self.pixels;
		let mut res = vec![vec![false; input[0].len() + 2]; input.len() + 2];
		for y in 0..(input.len() + 2) {
			for x in 0..(input[0].len() + 2) {
				let mut idx = 0;
				for y_k in (y as isize - 1)..=(y as isize + 1) {
					for x_k in (x as isize - 1)..=(x as isize + 1) {
						let x_ko = x_k - 1;
						let y_ko = y_k - 1;
						let b = if (x_ko < 0) || (y_ko < 0)
								|| (x_ko >= input[0].len() as isize)
								|| (y_ko >= input.len() as isize) {
							self.env
						} else {
							input[y_ko as usize][x_ko as usize]
						};
						dprint!("{}", if b { '#' } else { '.' });
						idx <<= 1;
						idx |= b as usize;
					}
					dprint!("\n");
				}
				dprint!("{:09b}\n", idx);
				let lit = algo[idx];
				res[y][x] = lit;
			}
		}
		self.pixels = res;
		let env_idx = if self.env { 0b111111111 } else { 0 };
		self.env = algo[env_idx];
	}
	fn count_pixels_lit(&self) -> usize {
		if self.env {
			panic!("infinitely many pixels lit");
		}
		self.pixels.iter()
			.map(|l| l.iter().filter(|v| **v).count())
			.sum()
	}
}
#[cfg(test)]
fn line_to_string(l :&[bool]) -> String {
	l.iter()
		.map(|b| if *b { '#' } else { '.' })
		.collect()
}
