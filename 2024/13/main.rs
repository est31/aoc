use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mch = parse(INPUT);
	println!("min token counts: {}", min_token_counts(&mch));
	println!("min token counts add: {}", min_token_counts_add(&mch));
}

macro_rules! dprint {
	($($args:expr),*) => {
		//if false
			{ print!($($args),*); }
	};
}

#[derive(Copy, Clone, Debug)]
struct Machine {
	button_a: (u64, u64),
	button_b: (u64, u64),
	prize: (u64, u64),
}

fn parse(s: &str) -> Vec<Machine> {
	let s = s.trim();
	let mut lines = s.lines();
	let mut machines = Vec::new();
	while let Some(l) = lines.next() {
		let l = l.trim();
		if l.is_empty() { continue; }
		let Some(line_a) = l.strip_prefix("Button A:") else { panic!("Invalid A line '{l}'") };
		let l = lines.next().unwrap();
		let Some(line_b) = l.strip_prefix("Button B:") else { panic!("Invalid B line '{l}'") };
		let l = lines.next().unwrap();
		let Some(line_prize) = l.strip_prefix("Prize:") else { panic!("Invalid prize line '{l}'") };

		fn x_y(s :&str) -> (u64, u64) {
			let mut comps = s.split(['+', ',', '=']);
			comps.next();
			let x_s = comps.next().unwrap();
			comps.next();
			let y_s = comps.next().unwrap();
			(u64::from_str(x_s).unwrap(), u64::from_str(y_s).unwrap())
		}
		let button_a = x_y(line_a);
		let button_b = x_y(line_b);
		let prize = x_y(line_prize);

		machines.push(Machine { button_a, button_b, prize });
	}
	machines
}


impl Machine {
	fn tok_needed(&self) -> Option<u64> {
		dprint!("  -> tok_needed\n");
		// Calculate a * 3 + b under:
		// x = a * a_x + b * b_x
		// y = a * a_y + b * b_y
		// we get:
		// b = (y - a * a_y) / b_y
		// and:
		// x = a * a_x + (y - a * a_y) * b_x / b_y
		// x = a * a_x + y * b_x / b_y - a * a_y * b_x / b_y
		// x - y * b_x / b_y = a * (a_x - a_y * b_x / b_y)
		// a = (x - y * b_x / b_y) / (a_x - a_y * b_x / b_y)
		// a = (x * b_y - y * b_x) / (a_x * b_y - a_y * b_x)
		// We can put this into the above a * 3 + b formula.

		let a_x = self.button_a.0 as i64;
		let a_y = self.button_a.1 as i64;

		let b_x = self.button_b.0 as i64;
		let b_y = self.button_b.1 as i64;

		let p_x = self.prize.0 as i64;
		let p_y = self.prize.1 as i64;

		let a_frac = ifr(p_x * b_y - p_y * b_x, a_x * b_y - a_y * b_x);

		dprint!("    -> a_frac: {a_frac:?}\n");
		let a = a_frac.int_opt()?;

		if a * a_y > p_y {
			dprint!("    -> None as a * a_y > p_y\n");
			return None;
		}

		let b_frac = ifr(p_y - a * a_y, b_y);
		dprint!("    -> b_frac: {b_frac:?}\n");
		let b = b_frac.int_opt()?;

		let tok = a * 3 + b;
		if tok < 0 {
			dprint!("    -> None as tok < 0\n");
			return None;
		}

		dprint!("    -> ret: {tok}\n");
		Some(tok as u64)
	}
}

fn gcd(mut a :u64, mut b :u64) -> u64 {
	while b != 0 {
		let temp = b;
		b = a % b;
		a = temp;
	}
	a
}

fn ifr(v: i64, w: i64) -> Frac {
	Frac { v: v * w.signum(), w: w.abs() as u64, }
}

fn ufr(v: u64, w: u64) -> Frac {
	Frac { v: v as i64, w }
}

#[derive(Copy, Clone, Debug)]
struct Frac {
	v: i64,
	w: u64,
}

impl Frac {
	fn invert(mut self) -> Self {
		let sign = self.v.signum();
		let v = self.v.abs();
		self.v = self.w as i64 * sign;
		self.w = v as u64;
		self
	}
	fn mul_by(mut self, mul: u64) -> Self {
		self.v *= mul as i64;
		self
	}

	fn extend_by(&mut self, mul: u64) {
		self.v *= mul as i64;
		self.w *= mul;
	}
	fn normalize(&mut self) {
		let gcd = gcd(self.v.abs() as u64, self.w);
		self.v /= gcd as i64;
		self.w /= gcd;
	}
	fn int_opt(&self) -> Option<i64> {
		let v = self.v as u64;
		if v % self.w != 0 {
			return None;
		}
		Some((v / self.w) as i64 * self.v.signum())
	}
	fn uint_opt(&self) -> Option<u64> {
		if self.v < 0 {
			return None;
		}
		let v = self.v as u64;
		if v % self.w != 0 {
			return None;
		}
		Some(v / self.w)
	}
}

impl std::ops::Add<i64> for Frac {
	type Output = Frac;

	fn add(self, rhs :i64) -> Self::Output {
		let rhs = Frac { v: rhs, w: 1 };
		let mut r = self + rhs;
		r.normalize();
		r
	}
}

impl std::ops::Add<Self> for Frac {
	type Output = Frac;

	fn add(mut self, mut rhs :Self) -> Self::Output {
		self.extend_by(rhs.w);
		rhs.extend_by(self.w);
		self.v += rhs.v;
		self.normalize();
		self
	}
}

fn minimal_token_count_max_100_(m :&Machine) -> Option<(u64, u64)> {
	dprint!("  -> inner\n");
	for a_cnt in 0..100 {
		let a = (m.button_a.0 * a_cnt, m.button_a.1 * a_cnt);
		if a.0 > m.prize.0 || a.1 > m.prize.1 {
			return None;
		}
		if a == m.prize {
			dprint!("    -> a_cnt only: {a_cnt}\n");
			return Some((a_cnt, 0));
		}
		let need = (m.prize.0 - a.0, m.prize.1 - a.1);
		if (need.0 % m.button_b.0 != 0) || (need.1 % m.button_b.1 != 0) {
			continue;
		}
		let b_cnt = need.0 / m.button_b.0;
		if b_cnt * m.button_b.1 != need.1 {
			continue;
		}
		dprint!("    -> a_cnt: {a_cnt}, b_cnt: {b_cnt}\n");
		return Some((a_cnt, b_cnt));
	}
	None
}

fn minimal_token_count_max_100_slow(m :&Machine) -> Option<u64> {
	dprint!("Machine {m:?}\n");
	let no_swap = minimal_token_count_max_100_(m).map(|(a, b)| a * 3 + b);
	let mut m_swapped = m.clone();
	std::mem::swap(&mut m_swapped.button_a, &mut m_swapped.button_b);
	let swapped = minimal_token_count_max_100_(&m_swapped).map(|(a, b)| a + b * 3);
	swapped.min(no_swap)
}

fn minimal_token_count_max_100(m :&Machine) -> Option<u64> {
	dprint!("Machine {m:?}\n");
	m.tok_needed()
}

fn min_token_counts(machines :&[Machine]) -> u64 {
	machines.iter()
		.filter_map(|m| minimal_token_count_max_100(m))
		.sum::<u64>()
}

fn minimal_token_count_add(m :&Machine) -> Option<u64> {
	dprint!("Machine {m:?}\n");
	let mut m = m.clone();
	m.prize.0 += 10000000000000;
	m.prize.1 += 10000000000000;
	m.tok_needed()
}

fn min_token_counts_add(machines :&[Machine]) -> u64 {
	machines.iter()
		.filter_map(|m| minimal_token_count_add(m))
		.sum::<u64>()
}
