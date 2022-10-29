use std::str::FromStr;
use std::ops::RangeInclusive;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let ticket = parse(INPUT);
	let er = scanning_error_rate(&ticket);
	println!("Scanning error rate: {er}");
	let product = product(&ticket);
	println!("Product: {product}");
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
			for v in tck.iter() {
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

fn get_possibles(ticket :&Ticket) -> Vec<Vec<usize>> {
	let it = ticket.nearby.iter()
		.chain(std::iter::once(&ticket.own))
		.filter(|tck| {
			for v in tck.iter() {
				let found = ticket.fields.iter()
					.map(|f| [&(f.1).0, &(f.1).1].into_iter())
					.flatten()
					.any(|range| range.contains(v));
				if !found {
					return false;
				}
			}
			true
		});

	let mut possibles = vec![(0..ticket.fields.len()).collect::<Vec<_>>(); ticket.own.len()];
	for tck in it {
		for (i, v) in tck.iter().enumerate() {
			possibles[i].retain(|p| {
				let fld = &ticket.fields[*p];
				fld.1.0.contains(v) || fld.1.1.contains(v)
			});
		}
	}
	//println!("{possibles:?}");

	let mut changed = true;

	while changed {
		changed = false;

		changed |= elim_singlets(&mut possibles);

		//println!("singlets eliminated: {possibles:?}");

		changed |= elim_uniques(&mut possibles);

		//println!("uniques eliminated:  {possibles:?}");
	}

	possibles
}

fn elim_singlets(possibles :&mut [Vec<usize>]) -> bool {
	// [[1], [1, 2, 3], [2, 3]] -> [[1], [2, 3], [2, 3]]
	let mut any_change = false;
	loop {
		let mut changed = false;
		for i in 0..(possibles.len() - 1) {
			if possibles[i].len() <= 1 {
				continue;
			}
			for j in 0..possibles.len() {
				if i == j {
					continue;
				}
				if possibles[j].len() != 1 {
					continue;
				}
				let p = possibles[j][0];
				possibles[i].retain(|r| {
					let found = *r == p;
					changed |= found;
					!found
				});
			}
		}
		if !changed {
			return any_change;
		} else {
			any_change = true;
		}
	}
}

fn elim_uniques(possibles :&mut [Vec<usize>]) -> bool {
	// [[1, 2], [2, 3], [3, 4]] -> [[1], [2, 3], [4]]
	let mut any_change = false;
	loop {
		let mut changed = false;
		for i in 0..possibles.len() {
			// i is here a field index, not an index into possibles
			let mut ctr = 0;
			let mut found = None;
			for j in 0..possibles.len() {
				if possibles[j].contains(&i) {
					found = Some(j);
					ctr += 1;
				}
			}
			if ctr == 1 {
				let found = found.unwrap();
				if possibles[found].len() != 1 {
					possibles[found] = vec![i];
					changed = true;
				}
			}
		}
		if !changed {
			return any_change;
		} else {
			any_change = true;
		}
	}
}

fn product(ticket :&Ticket) -> u64 {
	let possibles = get_possibles(ticket);

	ticket.own.iter()
		.enumerate()
		.map(|(i, v)| {
			let pi = &possibles[i];
			assert_eq!(pi.len(), 1, "Field {i} has wrong number of possibles: {pi:?}");
			let i_translated = pi[0];
			if ticket.fields[i_translated].0.starts_with("departure") {
				//println!("v is {v}");
				*v as u64
			} else {
				1
			}
		})
		.product()
}
