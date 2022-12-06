const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let end = start_of_packed_end(INPUT);
	println!("start of packet end: {end}");
}

fn is_start_of_pck(win :&[u8]) -> bool {
	for (i, wi) in win.iter().enumerate() {
		if i == win.len() - 1 {
			return true;
		}
		if win[(i + 1)..].iter().any(|v| v == wi) {
			return false;
		}
	}
	unreachable!();
}

fn start_of_packed_end(input :&str) -> usize {
	let mut offs = 0;
	for win in input.trim().as_bytes().windows(4) {
		if is_start_of_pck(win) {
			break;
		}
		offs += 1;
	}
	return offs + 4
}
