use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let eqs = parse(INPUT);
	println!("total calibration sum: {}", total_calibration_res(&eqs));
	println!("total calibration sum (concat): {}", total_calibration_concat(&eqs));
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

#[derive(Clone, Copy)]
enum Operation {
	Mul,
	Add,
	Concat,
}

fn concat(a: u64, b: u64) -> u64 {
	let b_digits = b.ilog(10);
	let mul = 10_u64.pow(b_digits + 1);
	a * mul + b
}

fn sat_inner(res: u64, acc: u64, terms: &[u64], allowed_ops: &[Operation]) -> bool {
	if terms.is_empty() {
		acc == res
	} else {
		if acc > res {
			return false;
		}
		let t = terms[0];
		for op in allowed_ops {
			let new_acc = match *op {
				Operation::Mul => acc * t,
				Operation::Add => acc + t,
				Operation::Concat => concat(acc, t),
			};
			let sat = sat_inner(res, new_acc, &terms[1..], allowed_ops);
			if sat {
				return true;
			}
		}
		false
	}
}

fn satisfyable(res: u64, terms: &[u64], allowed_ops :&[Operation]) -> bool {
	sat_inner(res, 0, terms, allowed_ops)
}

fn total_calibration_res(eqs: &[(u64, Vec<u64>)]) -> u64 {
	let allowed_ops = [Operation::Add, Operation::Mul];
	eqs.iter()
		.filter(|(res, terms)| satisfyable(*res, terms, &allowed_ops))
		.map(|(res, _)| *res)
		.sum::<u64>()
}

fn total_calibration_concat(eqs: &[(u64, Vec<u64>)]) -> u64 {
	let allowed_ops = [
		Operation::Add,
		Operation::Mul,
		Operation::Concat
	];
	eqs.iter()
		.filter(|(res, terms)| satisfyable(*res, terms, &allowed_ops))
		.map(|(res, _)| *res)
		.sum::<u64>()
}
