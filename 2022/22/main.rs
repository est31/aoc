use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (map, cmds) = parse(INPUT);
	let p = final_password(&map, &cmds);
	println!("Final password: {p}");
}

#[derive(Debug, Copy, Clone)]
enum Cmd {
	GoAhead(u16),
	TurnLeft,
	TurnRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Field {
	Free,
	Void,
	Wall,
}

#[derive(Debug, Clone)]
struct Map {
	fields :Vec<Vec<Field>>,
	limits_horiz :Vec<(u64, u64)>,
	limits_vert :Vec<(u64, u64)>,
	row_col :(u64, u64),
	facing :u8,
}

fn parse_cmds(cmds_line :&str) -> Vec<Cmd> {
	let mut res = Vec::new();
	let mut in_number = None;
	for (i, ch) in cmds_line.chars().enumerate() {
		let cmd = match ch {
			'L' => Cmd::TurnLeft,
			'R' => Cmd::TurnRight,
			d if d.is_ascii_digit() => {
				if in_number.is_none() {
					in_number = Some(i)
				}
				continue;
			},
			o => panic!("invalid char '{o}'"),
		};
		if let Some(st_i) = in_number.take() {
			let amnt = u16::from_str(&cmds_line[st_i..i]).unwrap();
			res.push(Cmd::GoAhead(amnt));
		}
		res.push(cmd);
	}
	if let Some(i) = in_number.take() {
		let amnt = u16::from_str(&cmds_line[i..]).unwrap();
		res.push(Cmd::GoAhead(amnt));
	}
	res
}

fn min_max(it :impl Iterator<Item = u64> + Clone) -> (u64, u64) {
	let min = it.clone().min().unwrap();
	let max = it.max().unwrap();
	(min, max)
}

fn parse(input :&str) -> (Map, Vec<Cmd>) {
	let mut next_are_cmds = false;
	let mut cmds_line = "";
	let mut fields = input.lines()
		.map(|l| l.trim())
		.filter_map(|l| {
			if l.is_empty() {
				next_are_cmds = true;
				return None;
			}
			if next_are_cmds {
				cmds_line = l;
				return None;
			}
			let row = l.chars()
				.map(|ch| {
					match ch {
						' ' => Field::Void,
						'.' => Field::Free,
						'#' => Field::Wall,
						_ => panic!("Invalid field char '{ch}'"),
					}
				})
				.collect::<Vec<_>>();
			Some(row)
		})
		.collect::<Vec<_>>();

	// Rectangularize fields
	let width = fields.iter()
		.map(|row| row.len())
		.max()
		.unwrap();
	for row in fields.iter_mut() {
		row.extend(std::iter::repeat(Field::Void).take(width - row.len()));
	}

	let cmds = parse_cmds(cmds_line);

	let limits_horiz = fields.iter()
		.map(|row| {
			let it = row.iter()
				.enumerate()
				.filter(|(_i, fld)| **fld != Field::Void)
				.map(|(i, _fld)| i as u64);
			min_max(it)
		})
		.collect::<Vec<_>>();

	let limits_vert = (0..fields[0].len())
		.map(|col_i| {
			let it = fields.iter()
				.enumerate()
				.map(|(i, row)| (i, row[col_i]))
				.filter(|(_i, fld)| *fld != Field::Void)
				.map(|(i, _fld)| i as u64);
			min_max(it)
		})
		.collect::<Vec<_>>();

	// starting position: leftmost open tile on top row of tiles
	let starting_col = fields[0]
		.iter()
		.enumerate()
		.find(|(_i, fld)| **fld == Field::Free)
		.unwrap().0 as u64;

	let map = Map {
		fields,
		limits_horiz,
		limits_vert,
		row_col : (0, starting_col),
		facing : 0,
	};
	(map, cmds)
}

fn run_commands(map :&mut Map, cmds :&[Cmd]) {
	todo!()
}

fn final_password(map :&Map, cmds :&[Cmd]) -> u64 {
	let mut map = map.clone();
	run_commands(&mut map, cmds);
	let pw = map.row_col.0 * 1000 + map.row_col.1 * 4 + map.facing as u64;
	pw
}
