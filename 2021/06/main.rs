use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut fishes = Lanternfishes::from_str(&INPUT);
	fishes.steps(80);
	println!("Count after 80 steps: {}", fishes.total_count());
	fishes.steps(256 - 80);
	println!("Count after 256 steps: {}", fishes.total_count());
}

struct Lanternfishes {
	states :[u128; 9],
}

impl Lanternfishes {
	fn from_str(input :&str) -> Self {
		let mut states = [0; 9];
		input.trim()
			.split(',')
			.map(|n| u128::from_str(n).unwrap())
			.for_each(|n| {
				states[n as usize] += 1;
			});
		Self {
			states,
		}
	}
	fn step(&mut self) {
		let states = self.states.clone();
		for (i, s) in self.states.iter_mut().enumerate() {
			if i < 8 {
				*s = states[i + 1];
			} else {
				*s = states[0];
			}
		}
		self.states[6] += states[0];
	}
	fn steps(&mut self, steps :usize) {
		for _ in 0..steps {
			self.step();
		}
	}
	fn total_count(&self) -> u128 {
		self.states.iter().sum()
	}
}
