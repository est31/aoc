use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let field = parse(INPUT);
	let e = empty_ground_tiles(&field);
	println!("Empty tiles count: {e}");
}

fn parse(input :&str) -> HashSet<(i16, i16)> {
	input.lines()
		.filter(|l| !l.is_empty())
		.enumerate()
		.map(|(i, l)| {
			l.chars()
				.enumerate()
				.filter(|(_j, ch)| *ch == '#')
				.map(move |(j, _ch)| (i as i16, j as i16))
		})
		.flatten()
		.collect::<HashSet<_>>()
}

fn step(field :&HashSet<(i16, i16)>, si :usize) -> HashSet<(i16, i16)> {
	let dir_list = [
		((-1, 0), (0, 1)), // North
		((1, 0), (0, 1)), // South
		((0, -1), (1, 0)), // West
		((0, 1), (1, 0)), // East
	];
	let mut elf_proposals = HashMap::new();
	let mut with_proposals = HashMap::<_, u16>::new();
	let mut new_field = HashSet::new();
	for pos in field.iter() {
		let mut dest_pos = None;
		let neigh_cnt = ((pos.0 - 1)..=(pos.0 + 1))
			.map(|x| ((pos.1 - 1)..=(pos.1 + 1)).map(move |y| (x, y)))
			.flatten()
			.filter(|pos| field.contains(pos))
			.count();
		if neigh_cnt == 0 {
			panic!("neigh count of 0 for {pos:?}");
		}
		if neigh_cnt == 1 {
			// The elf is alone and does not move.
			new_field.insert(*pos);
			continue;
		}
		for dir_i in 0..dir_list.len() {
			let (dir, orth) = dir_list[(dir_i + si) % dir_list.len()];
			let candidate_pos = (pos.0 + dir.0, pos.1 + dir.1);
			let positions = [
				candidate_pos,
				(candidate_pos.0 + orth.0, candidate_pos.1 + orth.1),
				(candidate_pos.0 - orth.0, candidate_pos.1 - orth.1),
			];
			if positions.iter().any(|pos| field.contains(pos)) {
				continue;
			}
			dest_pos = Some(candidate_pos);
			break;
		}
		let Some(dest_pos) = dest_pos else {
			// The elf can propose no movement
			new_field.insert(*pos);
			continue;
		};
		elf_proposals.insert(*pos, dest_pos);
		*with_proposals.entry(dest_pos).or_default() += 1;
	}
	//println!("{elf_proposals:?}");
	for (pos, proposed) in elf_proposals {
		let npos = if with_proposals[&proposed] <= 1 {
			proposed
		} else {
			pos
		};
		new_field.insert(npos);
	}
	assert_eq!(new_field.len(), field.len(), "Mismatch in elf census");
	new_field
}

fn get_field_dimensions(field :&HashSet<(i16, i16)>) -> [(i16, i16); 2] {
	let (min_x, max_x) = min_max(field.iter().map(|(x, _y)| *x));
	let (min_y, max_y) = min_max(field.iter().map(|(_x, y)| *y));
	[(min_x, min_y), (max_x, max_y)]
}

#[cfg(test)]
fn print_field(field :&HashSet<(i16, i16)>) {
	print!("{}", field_to_str(field));
}

#[cfg(test)]
fn field_to_str(field :&HashSet<(i16, i16)>) -> String {
	let [(min_x, min_y), (max_x, max_y)] = get_field_dimensions(&field);
	let mut s = String::new();
	for x in min_x..=max_x {
		for y in min_y..=max_y {
			let ch = if field.contains(&(x, y)) {
				'#'
			} else {
				'.'
			};
			s.push(ch);
		}
		s.push('\n');
	}
	s
}

fn empty_ground_tiles(field :&HashSet<(i16, i16)>) -> u32 {
	let mut field = field.clone();
	for si in 0..10 {
		field = step(&field, si);
		#[cfg(test)]
		{
			println!("\n-----------------\nAfter step {si}:");
			print_field(&field);
		}
	}
	let [(min_x, min_y), (max_x, max_y)] = get_field_dimensions(&field);
	let xdiff = (max_x + 1 - min_x).abs() as u32;
	let ydiff = (max_y + 1 - min_y).abs() as u32;
	//println!("w={xdiff} h={ydiff}");
	let total_tiles = xdiff * ydiff;
	total_tiles - field.len() as u32
}

fn min_max(it :impl Iterator<Item = i16> + Clone) -> (i16, i16) {
	let min = it.clone().min().unwrap();
	let max = it.max().unwrap();
	(min, max)
}
