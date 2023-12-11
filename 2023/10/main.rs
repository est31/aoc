use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	println!("farthest pos: {}", farthest(&field));
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(u8)]
enum Field {
	Vertical = b'|',
	Horizontal = b'-',
	BendNorthEast = b'L',
	BendNorthWest = b'J',
	BendSouthWest = b'7',
	BendSouthEast = b'F',
	Ground = b'.',
	Start = b'S',
}

fn parse(input :&str) -> Vec<Vec<Field>> {
	input.lines()
		.map(|l|{
			l.trim()
				.chars()
				.map(|c| match c as u8 {
					b'|' => Field::Vertical,
					b'-' => Field::Horizontal,
					b'L' => Field::BendNorthEast,
					b'J' => Field::BendNorthWest,
					b'7' => Field::BendSouthWest,
					b'F' => Field::BendSouthEast,
					b'.' => Field::Ground,
					b'S' => Field::Start,
					_ => panic!("Unexpected char '{c}'"),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn farthest(field :&[Vec<Field>]) -> u32 {
	let start_pos = field.iter()
		.enumerate()
		.filter_map(|(i, l)| l.iter()
			.enumerate()
			.find(|(_j, f)| f == &&Field::Start)
			.map(|(j, _f)| (i, j)))
		.next()
		.unwrap();
	let no_rev = walk_from_start(field, start_pos, false);
	let with_rev = walk_from_start(field, start_pos, true);
	//println!("no_rev: {no_rev:?}");
	//println!("with_rev: {with_rev:?}");

	let mut hm = HashMap::new();
	for (i, pos) in no_rev.into_iter().chain(with_rev.into_iter()) {
		let i_in_hm = hm.entry(pos).or_insert(i);
		*i_in_hm = i.min(*i_in_hm);
	}

	hm.iter()
		.map(|(_pos, i)| *i as u32)
		.max()
		.unwrap()
}

fn walk_from_start(field :&[Vec<Field>], start_pos :(usize, usize), rev :bool) -> Vec<(usize, (usize, usize))> {
	let mut prev_pos = start_pos;
	let mut pos = start_pos;
	let width = field[0].len();
	let height = field.len();
	let mut res = Vec::new();
	loop {
		use Field::*;

		res.push((res.len(), pos));

		let field_at_pos = field[pos.0][pos.1];
		//println!("at {pos:?}: {:?}", field_at_pos);

		let new_pos = if pos == start_pos {
			// Start case
			let mut possible_pos = Vec::with_capacity(4);
			if pos.0 > 0 && matches!(field[pos.0 - 1][pos.1], Vertical | BendSouthWest | BendSouthEast | Start) {
				possible_pos.push((pos.0 - 1, pos.1));
			}
			if pos.1 > 0 && matches!(field[pos.0][pos.1 - 1], Horizontal | BendNorthEast | BendSouthEast | Start) {
				possible_pos.push((pos.0, pos.1 - 1));
			}
			if pos.0 < width - 1 && matches!(field[pos.0 + 1][pos.1], Vertical | BendNorthEast | BendNorthWest | Start) {
				possible_pos.push((pos.0 + 1, pos.1));
			}
			if pos.1 < height - 1 && matches!(field[pos.0][pos.1 + 1], Horizontal | BendNorthWest | BendSouthWest | Start) {
				possible_pos.push((pos.0, pos.1 + 1));
			}
			//println!("  -> possible: {possible_pos:?} {:?}", possible_pos.iter().map(|(p0, p1)| field[*p0][*p1]).collect::<Vec<_>>());

			assert_eq!(possible_pos.len(), 2);

			if rev {
				*possible_pos.last().unwrap()
			} else {
				*possible_pos.first().unwrap()
			}
		} else {
			// Normal case
			let p = pos;
			let possible_pos = match field_at_pos {
				Vertical => [(p.0 - 1, p.1), (p.0 + 1, p.1)],
				Horizontal => [(p.0, p.1 - 1), (p.0, p.1 + 1)],
				BendNorthEast => [(p.0, p.1 + 1), (p.0 - 1, p.1)],
				BendNorthWest => [(p.0 - 1, p.1), (p.0, p.1 - 1)],
				BendSouthWest => [(p.0 + 1, p.1), (p.0, p.1 - 1)],
				BendSouthEast => [(p.0 + 1, p.1), (p.0, p.1 + 1)],
				Ground => panic!("ground encountered at {pos:?}"),
				Start => panic!("not supposed to be at start again!"),
			};
			//println!("  -> possible: {possible_pos:?} {:?}", possible_pos.iter().map(|(p0, p1)| field[*p0][*p1]).collect::<Vec<_>>());

			let mut it = possible_pos.iter()
				.filter(|p| **p != prev_pos);
			let next = it.next().unwrap();
			assert_eq!(it.next(), None);
			*next
		};

		prev_pos = pos;
		pos = new_pos;

		if pos == start_pos {
			break;
		}
	}
	res
}
