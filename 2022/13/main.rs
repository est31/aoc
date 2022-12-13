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
	let dk = decoder_key(&lists);
	println!("Decoder key: {dk}");
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum MaybeList {
	List(Vec<MaybeList>),
	Num(u32),
}


impl PartialOrd for MaybeList {
	fn partial_cmp(&self, other :&Self) -> Option<Ordering> {

		//println!("Comparing {self:?} with {other:?}");
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

impl Ord for MaybeList {
	fn cmp(&self, other :&Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

fn parse_maybe_list(input :&str) -> (MaybeList, &str) {
	//println!("input: '{input}'");
	let mut input = input;
	if let Ok(n) = u32::from_str(input) {
		return (MaybeList::Num(n), "");
	}

	if input.is_empty() {
		return (MaybeList::List(Vec::new()), input);
	}

	assert_eq!(&input[0..=0], "[");
	input = &input[1..];

	let mut res = Vec::new();
	loop {
		if input.starts_with("[") {
			let (ml, postfix) = parse_maybe_list(input);
			res.push(ml);
			input = postfix;
		} else if let Some(skipped) = input.strip_prefix("]") {
			input = skipped;
			break;
		} else if let Some((first, second)) = input.split_once([',', ']']) {
			res.push(parse_maybe_list(first).0);
			input = second;
		} else if input.is_empty() {
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

fn decoder_key(lists :&[(MaybeList, MaybeList)]) -> usize {
	let mut lists = lists.iter()
		.cloned()
		.map(|(l0, l1)| [l0, l1].into_iter())
		.flatten()
		.collect::<Vec<_>>();
	let d2 = parse_maybe_list("[[2]]").0;
	let d6 = parse_maybe_list("[[6]]").0;
	lists.extend_from_slice(&[d2.clone(), d6.clone()]);
	lists.sort();
	//for list in &lists { println!("    {list:?}"); }
	let v = lists.iter()
		.enumerate()
		.filter(|(_i, l)| {
			*l == &d2 || *l == &d6
		})
		.map(|(i, _l)| i + 1)
		.collect::<Vec<_>>();
	assert_eq!(v.len(), 2);
	v.iter().product()
}
