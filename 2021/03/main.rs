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
	let ge = gamma_espilon(INPUT)?;
	println!("Product gamma * epsilon: {}", ge.0 * ge.1);
	let (ox, co2) = ox_co2(INPUT)?;
	println!("Product oxygen * co2: {}", ox * co2);
	Ok(())
}

fn gamma_espilon(input :&str) -> Result<(u64, u64)> {
	let mut occ_1 = Vec::new();
	let mut ctr = 0;
	for l in input.lines() {
		let l = l.trim();
		if l.is_empty() {
			continue;
		}
		ctr += 1;
		if occ_1.len() == 0 {
			occ_1 = vec![0; l.len()];
		}
		for (o, b) in occ_1.iter_mut().zip(l.as_bytes().iter()) {
			if *b == b'1' {
				*o += 1;
			}
		}
	}
	let gamma = occ_1.iter()
		.map(|o| (*o > (ctr - *o)) as u64)
		.fold(0u64, |v, w| (v << 1) + w);
	let epsilon = occ_1.iter()
		.map(|o| (*o < (ctr - *o)) as u64)
		.fold(0u64, |v, w| (v << 1) + w);
	Ok((gamma, epsilon))
}

fn find_x_most_common_line(input :&str, x :bool) -> &str {
	let mut lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.collect::<Vec<_>>();
	let mut d = 0;
	while lines.len() > 1 {
		let mut occ = 0;
		// Determine x-most common bit
		for l in lines.iter() {
			if l.as_bytes()[d] == b'1' {
				occ += 1;
			}
		}
		// Determine the byte to consider
		let b = x ^ ((lines.len() - occ) > occ);
		// Do the filtering
		lines.retain(|l| (l.as_bytes()[d] == b'1') == b);
		d += 1;
	}
	lines.pop().unwrap()
}

fn ox_co2(input :&str) -> Result<(u64, u64)> {
	let ox_line = find_x_most_common_line(input, true);
	let co2_line = find_x_most_common_line(input, false);

	let ox = u64::from_str_radix(ox_line, 2)?;
	let co2 = u64::from_str_radix(co2_line, 2)?;

	Ok((ox, co2))
}
