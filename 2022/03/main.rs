use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	let prios = priorities_of_dupes(&lines);
	println!("priorities: {prios}");
	let g_prios = group_priorities(&lines);
	println!("group priorities: {g_prios}");
}

fn parse(input :&str) -> Vec<(&str, &str)> {
	input.lines()
		.map(|l| {
			let len = l.len();
			assert_eq!(len % 2, 0, "len {len} is not even");
			let hlen = len / 2;
			let comp_1 = &l[0..hlen];
			let comp_2 = &l[hlen..];
			(comp_1, comp_2)
		})
		.collect::<Vec<_>>()
}

fn priority(ch :char) -> u16 {
	let p = match ch {
		'a'..='z' => ch as u8 - b'a' + 1,
		'A'..='Z' => ch as u8 - b'A' + 27,
		_ => panic!("invalid char '{ch}'"),
	};
	p.into()
}

fn priorities_of_dupes(lines :&[(&str, &str)]) -> u16 {
	lines.iter()
		.map(|(c1, c2)| {
			let hs :HashSet<char> = c1.chars().collect();
			let dupes = c2.chars()
				.filter(|ch| hs.contains(ch))
				.map(|ch| priority(ch))
				.collect::<Vec<_>>();
			if dupes.iter().any(|d| *d != dupes[0]) {
				panic!("Multiple dupes found in '{c1}' '{c2}'");
			}
			dupes[0]
		})
		.sum()
}

fn chars<'a>(line :(&'a str, &'a str)) -> impl Iterator<Item = char> + 'a {
	line.0.chars().chain(line.1.chars())
}

fn group_priorities(lines :&[(&str, &str)]) -> u16 {
	lines.chunks(3)
		.map(|chunk| {
			let hs0 :HashSet<char> = chars(chunk[0]).collect();
			let hs1 :HashSet<char> = chars(chunk[1])
				.filter(|ch| hs0.contains(ch))
				.collect();
			let hs2 :HashSet<char> = chars(chunk[2])
				.filter(|ch| hs1.contains(ch))
				.collect();
			assert_eq!(hs2.len(), 1);
			let badge = hs2.iter().next().unwrap();
			priority(*badge)
		})
		.sum()
}
