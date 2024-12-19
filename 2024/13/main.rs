use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mch = parse(INPUT);
	println!("min token counts: {}", min_token_counts(&mch));
}

macro_rules! dprint {
	($($args:expr),*) => {
		//if false
			{ print!($($args),*); }
	};
}

#[derive(Copy, Clone, Debug)]
struct Machine {
	button_a: (u32, u32),
	button_b: (u32, u32),
	prize: (u32, u32),
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

		fn x_y(s :&str) -> (u32, u32) {
			let mut comps = s.split(['+', ',', '=']);
			comps.next();
			let x_s = comps.next().unwrap();
			comps.next();
			let y_s = comps.next().unwrap();
			(u32::from_str(x_s).unwrap(), u32::from_str(y_s).unwrap())
		}
		let button_a = x_y(line_a);
		let button_b = x_y(line_b);
		let prize = x_y(line_prize);

		machines.push(Machine { button_a, button_b, prize });
	}
	machines
}

fn minimal_token_count_max_100_(m :&Machine) -> Option<(u32, u32)> {
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

fn minimal_token_count_max_100(m :&Machine) -> Option<u32> {
	dprint!("Machine {m:?}\n");
	let no_swap = minimal_token_count_max_100_(m).map(|(a, b)| a * 3 + b);
	let mut m_swapped = m.clone();
	std::mem::swap(&mut m_swapped.button_a, &mut m_swapped.button_b);
	let swapped = minimal_token_count_max_100_(&m_swapped).map(|(a, b)| a + b * 3);
	swapped.min(no_swap)
}

fn min_token_counts(machines :&[Machine]) -> u32 {
	machines.iter()
		.filter_map(|m| minimal_token_count_max_100(m))
		.sum::<u32>()
}
