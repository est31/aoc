use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pks = parse(INPUT);
	let enc_key = get_encryption_key(pks);
	println!("Encryption key: {enc_key}");
}

fn parse(input :&str) -> (u32, u32) {
	let mut lines = input.lines();
	let first = lines.next().unwrap();
	let second = lines.next().unwrap();
	let first = u32::from_str(first).unwrap();
	let second = u32::from_str(second).unwrap();
	(first, second)
}

fn transform(subject :u32, loop_size :u32) -> u32 {
	let mut val = 1;
	for _ in 0..loop_size {
		val = ((val as u128 * subject as u128) % 20201227) as u32;
	}
	val
}

fn find_loop_size(pubkey :u32) -> u32 {
	const LIMIT: u32 = 10_000_000;
	let mut val = 1;
	let subject = 7;
	for ls in 1..LIMIT {
		val = val * subject;
		val = val % 20201227;
		if val == pubkey {
			return ls;
		}
	}
	panic!("not found");
}

fn get_encryption_key(pubkeys :(u32, u32)) -> u32 {
	let private = find_loop_size(pubkeys.0);
	let enc_key = transform(pubkeys.1, private);
	enc_key
}
