const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let groups = parse_1(INPUT);
	println!("sum 1: {}", sum(&groups));
	let groups = parse_2(INPUT);
	println!("sum 2: {}", sum(&groups));
}

fn parse_1(input: &str) -> Vec<u8> {
	input.lines()
		.map(|l| {
			let it = l.chars().filter(|c| c.is_ascii_digit());
			let first = it.clone().next().unwrap() as u8 - b'0';
			let last = it.rev().next().unwrap() as u8 - b'0';
			first * 10 + last
		})
		.collect()
}

fn parse_2(input: &str) -> Vec<u8> {
	const DIGITS: &[&str] = &[
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
		"0",
		"1",
		"2",
		"3",
		"4",
		"5",
		"6",
		"7",
		"8",
		"9",
	];
	let rev_digits = DIGITS
		.iter()
		.map(|s| s.chars().rev().collect::<String>())
		.collect::<Vec<_>>();
	fn first_digit<'a>(line: &str, digits: impl Iterator<Item = &'a str>) -> u8 {
		let idx = digits
			.enumerate()
			.filter_map(|(i, s)| {
				let offs = line.find(s)?;
				let i = (i as u8 + 1) % 10;
				//println!("{line}: {i} {offs}");
				Some((i, offs))
			})
			.min_by_key(|(_i, offs)| *offs)
			.unwrap().0;
		idx
	}
	input.lines()
		.map(|line| {
			let first = first_digit(line, DIGITS.iter().map(|s| *s));
			let line_rev = line.chars().rev().collect::<String>();
			let last = first_digit(&line_rev, rev_digits.iter().map(|s| s.as_str()));
			first * 10 + last
		})
		.collect()
}

fn sum(nums: &[u8]) -> u32 {
	nums.iter().map(|v| *v as u32).sum()
}
