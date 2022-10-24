const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let fields = parse(INPUT);
	println!("Occupied seats after fixpoint reached: {}", step_until_no_change(fields));
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
	Floor,
	EmptySeat,
	FullSeat,
}

impl Field {
	fn from_ch(ch :char) -> Option<Self> {
		Some(match ch {
			'.' => Field::Floor,
			'#' => Field::FullSeat,
			'L' => Field::EmptySeat,
			_ => return None,
		})
	}
	#[cfg(test)]
	fn to_ch(self) -> char {
		match self {
			Field::Floor => '.',
			Field::FullSeat => '#',
			Field::EmptySeat => 'L',
		}
	}
}

fn parse(input :&str) -> Vec<Vec<Field>> {
	input.lines()
		.map(|l| {
			l.chars()
			.map(|c| Field::from_ch(c).unwrap())
			.collect::<Vec<_>>()
		})
		.collect()
}

#[cfg(test)]
fn fields_to_string(fields :&[Vec<Field>]) -> String {
	fields.iter()
		.map(|l| {
			l.iter()
				.copied()
				.map(Field::to_ch)
				.chain(std::iter::once('\n'))
		})
		.flatten()
		.collect::<String>()
}

fn count_occupied_adjacent(fields :&[Vec<Field>], i :usize, j :usize) -> usize {
	fn range(v :usize, limit :usize) -> Vec<usize> {
		let mut res = Vec::with_capacity(2);
		if v > 0 {
			res.push(v - 1);
		}
		res.push(v);
		if v < limit {
			res.push(v + 1);
		}
		res
	}
	let i_range = range(i, fields.len() - 1);
	let j_range = range(j, fields[0].len() - 1);
	let mut sum = 0;
	for is in i_range {
		for js in j_range.clone() {
			if (is, js) == (i, j) {
				continue;
			}
			if let Field::FullSeat = fields[is][js] {
				sum += 1;
			}
		}
	}
	sum
}

fn step(fields :&[Vec<Field>]) -> (bool, Vec<Vec<Field>>) {
	let mut change = false;
	let res = fields.iter()
		.enumerate()
		.map(|(i, l)| {
			l.iter()
				.enumerate()
				.map(|(j, f)| {
					let occupied_adj = count_occupied_adjacent(fields, i, j);
					match (f, occupied_adj) {
						(Field::EmptySeat, 0) => {
							change = true;
							Field::FullSeat
						},
						(Field::FullSeat, 4..) => {
							change = true;
							Field::EmptySeat
						},
						_ => *f,
					}
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	(change, res)
}

fn count_occupied(fields :&[Vec<Field>]) -> usize {
	fields.iter()
		.map(|l| {
			l.iter()
				.filter(|f| matches!(f, Field::FullSeat))
				.count()
		})
		.sum()
}

fn step_until_no_change(fields :Vec<Vec<Field>>) -> usize {
	let mut fields = fields;
	loop {
		let (changes, new_fields) = step(&fields);
		if !changes {
			break;
		}
		fields = new_fields;
	}
	count_occupied(&fields)
}
