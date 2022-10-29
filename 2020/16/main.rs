use std::str::FromStr;
use std::ops::RangeInclusive;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let ticket = parse(INPUT);
	let er = scanning_error_rate(&ticket);
	println!("Scanning error rate: {er}");
}

struct Ticket {
	fields :Vec<(String, (RangeInclusive<u16>, RangeInclusive<u16>))>,
	own :Vec<u16>,
	nearby :Vec<Vec<u16>>,
}

fn parse_range(s :&str) -> RangeInclusive<u16> {
	let mut c = s.split('-');
	let first = c.next().unwrap();
	let last = c.next().unwrap();
	let first = u16::from_str(first).unwrap();
	let last = u16::from_str(last).unwrap();
	first..=last
}

fn parse_list(input :&str) -> Vec<u16> {
	input.trim()
		.split(',')
		.map(|n| u16::from_str(n).unwrap())
		.collect::<Vec<_>>()
}

fn parse(input :&str) -> Ticket {
	let mut lines = input.lines();
	let mut fields = Vec::new();
	while let Some(line) = lines.next() {
		if line.trim().is_empty() {
			break;
		}
		let mut components = line.split(": ");
		let s = components.next().unwrap();
		let mut components = components.next().unwrap().split(" or ");
		let first = components.next().unwrap();
		let last = components.next().unwrap();
		fields.push((s.to_owned(), (parse_range(first), parse_range(last))));
	}

	let _your_ticket = lines.next().unwrap();

	let own = parse_list(lines.next().unwrap());

	let _empty = lines.next().unwrap();
	let _nearby_tickets = lines.next().unwrap();

	let nearby = lines.map(parse_list)
		.collect::<Vec<_>>();

	Ticket {
		fields,
		own,
		nearby,
	}
}

fn scanning_error_rate(ticket :&Ticket) -> u16 {
	ticket.nearby.iter()
		.map(|tck| {
			let mut s = 0;
			for v in tck {
				let found = ticket.fields.iter()
					.map(|f| [&(f.1).0, &(f.1).1].into_iter())
					.flatten()
					.any(|range| range.contains(v));
				if !found {
					s += *v;
				}
			}
			s
		})
		.sum()
}
