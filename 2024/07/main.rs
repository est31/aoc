use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let eqs = parse(INPUT);
	println!("total calibration sum: {}", total_calibration_res(&eqs));
}

fn parse(s: &str) -> Vec<(u64, Vec<u64>)> {
	s.lines()
		.map(|l| {
			let mut cmps = l.split(": ");
			let r = cmps.next().unwrap();
			let r = u64::from_str(r).unwrap();
			let t = cmps.next().unwrap();
			let terms = t.split(' ')
				.map(|c| u64::from_str(c).unwrap())
				.collect::<Vec<_>>();
			(r, terms)
		})
		.collect::<Vec<_>>()
}

fn res_so_far(terms: &[u64], ops: &[bool]) -> u64 {
	terms.iter()
		.zip(ops.iter())
		.fold(0, |acc, (t, o)| if *o {
			acc + (*t)
		} else {
			acc * (*t)
		})
}

fn sat_inner(res: u64, terms: &[u64], ops: &mut Vec<bool>) -> bool {
	if ops.len() == terms.len() {
		res_so_far(terms, ops) == res
	} else {
		ops.push(true);
		let sat = sat_inner(res, terms, ops);
		ops.pop();
		if sat {
			return true;
		}
		ops.push(false);
		let sat = sat_inner(res, terms, ops);
		ops.pop();
		sat
	}
}

fn satisfyable(res: u64, terms: &[u64]) -> bool {
	let mut ops = Vec::with_capacity(terms.len());
	sat_inner(res, terms, &mut ops)
}

fn total_calibration_res(eqs: &[(u64, Vec<u64>)]) -> u64 {
	eqs.iter()
		.filter(|(res, terms)| satisfyable(*res, terms))
		.map(|(res, _)| *res)
		.sum::<u64>()
}
