const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut sc = Scene::parse(INPUT);
	let mut steps = 0;
	while sc.step() {
		steps += 1;
	}
	steps += 1;
	println!("steps: {steps}");
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Field {
	Down,
	Right,
	Empty,
}

impl std::fmt::Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Field::Down => write!(f, "v"),
			Field::Right => write!(f, ">"),
			Field::Empty => write!(f, "."),
		}
	}
}

impl Field {
	fn from_char(c :char) -> Result<Self, String> {
		Ok(match c {
			'v' => Field::Down,
			'>' => Field::Right,
			'.' => Field::Empty,
			_ => return Err(format!("Char '{c}' is invalid field!")),
		})
	}
	fn is_empty(&self) -> bool {
		matches!(self, Field::Empty)
	}
}

#[derive(PartialEq, Eq)]
struct Scene {
	fields :Vec<Vec<Field>>,
}

impl std::fmt::Display for Scene {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for line in self.fields.iter() {
			line.iter()
				.map(|field| write!(f, "{field}"))
				.collect::<std::fmt::Result>()?;
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Scene {
	fn parse(input :&str) -> Self {
		let fields = input.lines()
			.map(|l| l.trim())
			.filter(|l| !l.is_empty())
			.map(|l| {
				l.chars()
					.map(|c| Field::from_char(c).unwrap_or_else(|e| panic!("{e}")))
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		Self {
			fields,
		}
	}
	fn step(&mut self) -> bool {
		let height = self.fields.len();
		let width = self.fields.get(0).expect("scene is empty").len();
		let mut was_update = false;

		let mut new_fields = self.fields.clone();
		for i in 0..height {
			for j in 0..width {
				let next_j = (j + 1) % width;
				if self.fields[i][j] == Field::Right && self.fields[i][next_j].is_empty() {
					// Do the move
					was_update = true;
					new_fields[i][next_j] = new_fields[i][j];
					new_fields[i][j] = Field::Empty;
				}
			}
		}
		self.fields = new_fields;

		let mut new_fields = self.fields.clone();
		for i in 0..height {
			let next_i = (i + 1) % height;
			for j in 0..width {
				if self.fields[i][j] == Field::Down && self.fields[next_i][j].is_empty() {
					// Do the move
					was_update = true;
					new_fields[next_i][j] = new_fields[i][j];
					new_fields[i][j] = Field::Empty;
				}
			}
		}
		self.fields = new_fields;

		was_update
	}
}
