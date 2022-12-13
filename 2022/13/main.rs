use std::collections::HashSet;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lists = parse(INPUT);
	let sum = well_ordered_sum(&lists);
	println!("Well ordered lists: {sum}");
}

#[derive(PartialEq, Clone, Debug)]
enum MaybeList {
	List(Vec<MaybeList>),
	Num(u32),
}

impl PartialOrd for MaybeList {
	fn partial_cmp(&self, other :&Self) -> Option<Ordering> {

		println!("Comparing {self:?} with {other:?}");
		let cmp = |la :&[MaybeList], lb :&[MaybeList]| -> Option<Ordering> {
			for (ita, itb) in la.iter().zip(lb) {
				match ita.partial_cmp(itb) {
					Some(Ordering::Equal) => continue,
					other => return other,
				}
			}
			// Still no result, fall back to length comparison
			la.len().partial_cmp(&lb.len())
		};
		match (self, other) {
			(MaybeList::List(la), MaybeList::List(lb)) => cmp(la.as_slice(), lb.as_slice()),
			(MaybeList::Num(la), MaybeList::Num(lb)) => la.partial_cmp(lb),
			(MaybeList::List(la), lb @ MaybeList::Num(_)) => cmp(la.as_slice(), [lb.clone()].as_slice()),
			(la @ MaybeList::Num(_), MaybeList::List(lb)) => cmp([la.clone()].as_slice(), lb.as_slice()),
		}

	}
}

fn parse_maybe_list(input :&str) -> (MaybeList, &str) {
	//println!("input: '{input}'");
	let mut input = input;
	if let Ok(n) = u32::from_str(input) {
		return (MaybeList::Num(n), "");
	}
	let mut res = Vec::new();
	loop {
		if let Some(skipped) = input.strip_prefix("]") {
			input = skipped;
			break;
		}
		if input.is_empty() {
			break;
		}
		let mut sp_it = input.splitn(2, '[');
		let prefix = sp_it.next().unwrap();
		if !prefix.is_empty() {
			for comp in prefix.split(',') {
				res.push(parse_maybe_list(comp).0);
			}
		}
		if let Some(postfix) = sp_it.next() {
			let postfix = postfix.split(']').nth(0).unwrap();
			let (ml, postfix) = parse_maybe_list(postfix);
			res.push(ml);
			input = postfix;
		} else {
			input = "";
			break;
		}
	}
	return (MaybeList::List(res), input);
}

fn parse(input :&str) -> Vec<(MaybeList, MaybeList)> {
	let mut lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut res = Vec::new();
	while let Some(line) = lines.next() {
		let other_line = lines.next().unwrap();
		let (first, fst_s) = parse_maybe_list(line);
		let (second, snd_s) = parse_maybe_list(other_line);
		assert_eq!(fst_s, "");
		assert_eq!(snd_s, "");
		res.push((first, second));
	}
	res
}

fn well_ordered_sum(lists :&[(MaybeList, MaybeList)]) -> usize {
	lists.iter()
		.enumerate()
		.filter(|(_i, (la, lb))| la <= lb)
		.map(|(i, _)| i + 1)
		.sum()
}
