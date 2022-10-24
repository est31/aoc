const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let fields = parse(INPUT);
	println!("Occupied seats after fixpoint reached: {}", step_until_no_change(&fields, step));
	println!("Modified rules: {}", step_until_no_change(&fields, step_p2));
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
		let mut res = Vec::with_capacity(3);
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

fn map_fields(fields :&[Vec<Field>], f :impl Fn(usize, usize, Field) -> (bool, Field)) -> (bool, Vec<Vec<Field>>) {
	let mut change = false;
	let res = fields.iter()
		.enumerate()
		.map(|(i, l)| {
			l.iter()
				.enumerate()
				.map(|(j, field)| {
					let (ch, field) = f(i, j, *field);
					change |= ch;
					field
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	(change, res)
}

fn step(fields :&[Vec<Field>]) -> (bool, Vec<Vec<Field>>) {
	map_fields(fields, |i, j, field| {
		let occupied_adj = count_occupied_adjacent(fields, i, j);
		match (field, occupied_adj) {
			(Field::EmptySeat, 0) => {
				(true, Field::FullSeat)
			},
			(Field::FullSeat, 4..) => {
				(true, Field::EmptySeat)
			},
			_ => (false, field),
		}
	})
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


fn count_occupied_dirs(fields :&[Vec<Field>], i :usize, j :usize) -> usize {
	let dirs = [
		(-1, -1), (0, -1), (1, -1),
		(-1, 0), (1, 0),
		(-1, 1), (0, 1), (1, 1),
	];
	let mut sum = 0;
	for dir in dirs {
		let mut i = i as isize;
		let mut j = j as isize;

		loop {
			i += dir.0;
			j += dir.1;

			if i < 0 || j < 0 {
				break;
			}
			if i >= fields.len() as isize || j >= fields[0].len() as isize {
				break;
			}
			let f = fields[i as usize][j as usize];
			match f {
				Field::FullSeat => {
					sum += 1;
					break;
				},
				Field::EmptySeat => break,
				Field::Floor => (),
			}
		}
	}
	sum
}

fn step_p2(fields :&[Vec<Field>]) -> (bool, Vec<Vec<Field>>) {
	map_fields(fields, |i, j, field| {
		let occupied_adj = count_occupied_dirs(fields, i, j);
		match (field, occupied_adj) {
			(Field::EmptySeat, 0) => {
				(true, Field::FullSeat)
			},
			(Field::FullSeat, 5..) => {
				(true, Field::EmptySeat)
			},
			_ => (false, field),
		}
	})
}


fn step_until_no_change(fields :&[Vec<Field>], step_fn :fn(&[Vec<Field>]) -> (bool, Vec<Vec<Field>>)) -> usize {
	let mut fields = fields.to_vec();
	loop {
		let (changes, new_fields) = step_fn(&fields);
		if !changes {
			break;
		}
		fields = new_fields;
	}
	count_occupied(&fields)
}
