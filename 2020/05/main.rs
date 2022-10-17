const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let highest = highest_seat_id(INPUT);
	println!("highest seat ID: {highest}");
	let gap = gap_seat_id(INPUT);
	println!("gap seat ID: {gap}");
}

fn highest_seat_id(input :&str) -> u16 {
	let largest = seat_ids_iter(input).max();
	largest.unwrap()
}

fn gap_seat_id(input :&str) -> u16 {
	let mut seat_ids = seat_ids_iter(input)
		.collect::<Vec<_>>();
	seat_ids.sort();
	for w in seat_ids.windows(2) {
		// TODO use array_windows or something like it once it's stable
		let &[w0, w1] = w else { panic!() };
		if w0 + 1 != w1 {
			return w0 + 1;
		}
	}
	panic!("Didn't find gap!");
}


fn seat_ids_iter(input :&str) -> impl Iterator<Item = u16> + '_ {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| parse_row_col_seat_id(l).2)
}

fn parse_row_col_seat_id(line :&str) -> (u8, u8, u16) {
	let line = line.as_bytes();
	let row = parse_num(&line[0..7]);
	let col = parse_num(&line[7..10]);
	(row, col, row as u16 * 8 + col as u16)
}

fn parse_num(input :&[u8]) -> u8 {
	let mut res = 0u8;
	for (i, ch) in input.iter().enumerate() {
		let bit = match ch {
			b'F' | b'L' => 0,
			b'B' | b'R' => 1,
			_ => panic!("unsupported byte '{ch}'!"),
		};
		res |= bit << (input.len() - 1 - i);
	}
	res
}
