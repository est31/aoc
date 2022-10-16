use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	let conformances = count_conformances_1(&lines);
	println!("conformances policy 1: {conformances}");
	let conformances = count_conformances_2(&lines);
	println!("conformances policy 2: {conformances}");
}

type Line = (u8, u8, char, String);

fn parse(input :&str) -> Vec<Line> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut it = l.split(' ');
			let (Some(range), Some(ch), Some(pw)) = (it.next(), it.next(), it.next()) else {
				panic!("Invalid line: '{l}'");
			};
			let mut it = range.split('-');
			let (Some(st), Some(end)) = (it.next(), it.next()) else {
				panic!("Can't parse range: '{range}'")
			};
			let st = u8::from_str(st).unwrap();
			let end = u8::from_str(end).unwrap();
			let ch = ch.chars().next().unwrap();
			(st, end, ch, pw.to_string())
		})
		.collect::<Vec<_>>()
}

fn check_conforms_1((st, end, ch, pw) :&Line) -> bool {
	let count = pw.chars()
		.filter(|c| c == ch)
		.count();
	if count > u8::MAX.into() {
		return false;
	}
	(*st..=*end).contains(&(count as u8))
}

fn count_conformances_1(lines :&[Line]) -> usize {
	lines.iter()
		.filter(|l| check_conforms_1(l))
		.count()
}

fn check_conforms_2((st, end, ch, pw) :&Line) -> bool {
	let first = pw.chars()
		.nth(*st as usize - 1)
		.map(|c| c == *ch)
		.unwrap_or(false);
	let second = pw.chars()
		.nth(*end as usize - 1)
		.map(|c| c == *ch)
		.unwrap_or(false);
	//println!("'{pw}' {first} {second} {st} {:?}", pw.chars().nth(*st as usize));
	(first as u8 + second as u8) == 1
}

fn count_conformances_2(lines :&[Line]) -> usize {
	lines.iter()
		.filter(|l| check_conforms_2(l))
		.count()
}
