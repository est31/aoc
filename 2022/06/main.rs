const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let end = start_of_packet_end(INPUT);
	println!("start of packet end: {end}");
	let msg_end = start_of_msg_end(INPUT);
	println!("start of msg end: {msg_end}");
}

fn is_start(win :&[u8]) -> bool {
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

fn start_of_packet_end(input :&str) -> usize {
	start_of_end_siz(input, 4)
}

fn start_of_msg_end(input :&str) -> usize {
	start_of_end_siz(input, 14)
}

fn start_of_end_siz(input :&str, siz :usize) -> usize {
	let mut offs = 0;
	for win in input.trim().as_bytes().windows(siz) {
		if is_start(win) {
			break;
		}
		offs += 1;
	}
	return offs + siz
}
