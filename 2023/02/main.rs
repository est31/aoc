use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let games = parse(INPUT);
	println!("sum possble: {}", possible_games_id_sum(&games));
	println!("sum min powers: {}", sum_of_min_powers(&games));
}

#[derive(Copy, Clone, Debug)]
struct Rgb {
	red: u16,
	green: u16,
	blue: u16,
}

impl Rgb {
	fn parse(s: &str) -> Self {
		let mut red = 0;
		let mut green = 0;
		let mut blue = 0;
		for component in s.split(',') {
			let component = component.trim();
			if component.len() == 0 {
				continue;
			}
			let mut sub_cmp = component.split(' ');
			let (Some(val_st), Some(color_st)) = (sub_cmp.next(), sub_cmp.next()) else {
				panic!("Invalid component '{component}'");
			};
			let val = u16::from_str(val_st).unwrap();
			match color_st {
				"red" => red = val,
				"green" => green = val,
				"blue" => blue = val,
				_ => panic!("Invalid color '{color_st}'"),
			}
		}
		Self { red, green, blue }
	}
	fn possible(&self) -> bool {
		self.red <= 12 && self.green <= 13 && self.blue <= 14
	}
	fn power(&self) -> u32 {
		self.red as u32 * self.green as u32 * self.blue as u32
	}
}

fn parse(input: &str) -> Vec<Vec<Rgb>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let revs = l.split(':').nth(1).unwrap();
			revs.split(';')
				.map(|r| Rgb::parse(r))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn possible_games_id_sum(games: &[Vec<Rgb>]) -> u16 {
	let sum: u16 = games.iter()
		.enumerate()
		.filter(|(_i, revelations)| {
			revelations.iter()
				.all(|rev| rev.possible())
		})
		.map(|(i, _rev)| i as u16 + 1)
		.sum();
	sum
}

fn sum_of_min_powers(games: &[Vec<Rgb>]) -> u32 {
	use std::cmp::max;
	let sum: u32 = games.iter()
		.map(|revelations| {
			let min = revelations.iter()
				.map(|rev| *rev)
				.reduce(|left, right| Rgb {
					red: max(left.red, right.red),
					green: max(left.green, right.green),
					blue: max(left.blue, right.blue),
				})
				.unwrap();
			//println!("min: {min:?}");
			min.power()
		})
		.sum();
	sum
}
