use std::collections::HashMap;
use std::fmt::Display;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct StrErr(String);

impl<T :Display> From<T> for StrErr {
	fn from(v :T) -> Self {
		StrErr(format!("{}", v))
	}
}

type Result<T> = std::result::Result<T, StrErr>;

fn main() -> Result<()> {
	let (template, rules) = parse(INPUT)?;
	let mut p = template.to_string();

	for _ in 0..10 {
		p = rules.run_step(&p);
	}

	let occ = count_byte_occurences(p.as_bytes());
	let diff = compute_diff(&occ);

	println!("Diff after 10 steps: {}", diff);

	let mut p = Polymer::from_str(&p);
	for st in 10..40 {
		println!("Running step {}...", st + 1);
		p = p.run_step(&rules);
	}
	let occ = p.count_byte_occurences();
	let diff = compute_diff(&occ);

	println!("Diff after 40 steps: {}", diff);
	Ok(())
}

fn count_byte_occurences(arr :&[u8]) -> [u64; 256] {
	let mut occ = [0; 256];
	for c in arr {
		occ[*c as usize] += 1;
	}
	occ
}

fn compute_diff(occ :&[u64]) -> u64 {
	occ.iter().filter(|v| **v > 0).max().unwrap() -
		occ.iter().filter(|v| **v > 0).min().unwrap()
}

fn parse(input :&str) -> Result<(&str, Rules)> {
	let mut lines = input.lines();
	let template = lines.next().ok_or("No polymer template")?;
	let rules = Rules::from_lines(lines)?;
	Ok((template, rules))
}

struct Rules {
	rules :HashMap<[u8; 2], char>,
}

impl Rules {
	fn from_lines<'s>(lines_iter :impl Iterator<Item = &'s str>) -> Result<Self> {
		let mut rules = HashMap::new();
		for l in lines_iter {
			if l.is_empty() {
				continue;
			}
			let b = l.as_bytes();
			if b.len() != 7 {
				Err(format!("this line has the wrong length {}: '{}'", b.len(), l))?;
			}
			rules.insert([b[0], b[1]], b[6] as char);
		}
		Ok(Rules {
			rules,
		})
	}
	fn run_step(&self, template :&str) -> String {
		let mut res = String::new();
		for p in template.as_bytes().windows(2) {
			let p @ [p0, _] = if let [p0, p1] = p {
				[*p0, *p1]
			} else {
				panic!()
			};
			res.push(p0 as char);
			if let Some(r) = self.rules.get(&p) {
				res.push(*r);
			}
		}
		if template.len() > 0 {
			res.push(template.as_bytes()[template.len() - 1] as char);
		}
		res
	}
}

#[derive(PartialEq, Eq, Debug)]
struct Polymer {
	first_last :[u8; 2],
	pair_occurences :HashMap<[u8; 2], u64>,
}

impl Polymer {
	fn pair(&mut self, pair :&[u8; 2]) -> &mut u64 {
		self.pair_occurences.entry(*pair).or_insert(0)
	}
	fn from_str(s :&str) -> Self {
		let s_bytes = s.as_bytes();
		let mut poly = Polymer {
			first_last : [s_bytes[0], s_bytes[s_bytes.len() - 1]],
			pair_occurences: HashMap::new(),
		};
		for p in s.as_bytes().windows(2) {
			let p = if let [p0, p1] = p {
				[*p0, *p1]
			} else {
				panic!()
			};
			*poly.pair(&p) += 1;
		}
		poly
	}

	fn run_step(&self, rules :&Rules) -> Self {
		let mut res = Polymer {
			first_last : self.first_last,
			pair_occurences: HashMap::new(),
		};
		for (p @ &[p0, p1], occurences) in self.pair_occurences.iter() {
			if let Some(r) = rules.rules.get(p) {
				*res.pair(&[p0, *r as u8]) += occurences;
				*res.pair(&[*r as u8, p1]) += occurences;
			} else {
				*res.pair(&p) += occurences;
			}
		}
		res
	}

	fn count_byte_occurences(&self) -> [u64; 256] {
		let mut occ = [0; 256];

		for (&[p0, p1], occurences) in self.pair_occurences.iter() {
			occ[p0 as usize] += occurences;
			occ[p1 as usize] += occurences;
		}
		occ[self.first_last[0] as usize] += 1;
		occ[self.first_last[1] as usize] += 1;

		occ.iter_mut().for_each(|v| *v /= 2);
		occ
	}

	#[cfg(test)]
	fn len(&self) -> u64 {
		self.pair_occurences.iter()
			.map(|(_p, occ)| *occ)
			.sum::<u64>() + 1
	}
}
