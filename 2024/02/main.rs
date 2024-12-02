use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let reports = parse_reports(INPUT);
	println!("numbers of safe: {}", number_of_safe(&reports));
	println!("numbers of safe dampened: {}", number_of_safe_dampened(&reports));
}

fn parse_reports(s: &str) -> Vec<Vec<u16>> {
	let mut reports = Vec::new();
	for line in s.lines() {
		let report = line.split(' ')
			.map(|v| u16::from_str(v).unwrap())
			.collect::<Vec<_>>();
		reports.push(report);
	}
	reports
}

fn number_of_safe(reports: &[Vec<u16>]) -> usize {
	number_of_safe_budget(reports, 0)
}

fn number_of_safe_dampened(reports: &[Vec<u16>]) -> usize {
	number_of_safe_budget(reports, 1)
}

fn is_safe_with_budget(maybe_removed: Option<usize>, report: &[u16], budget: u8) -> bool {
	if budget == 1 && maybe_removed == None {
		if report.len() <= 2
			|| is_safe_with_budget(Some(0), report, 0)
			|| is_safe_with_budget(Some(1), report, 0)
		{
			return true;
		}
	}
	let mut mode = None;
	let mut last = None;
	let mut budget = budget;
	for (i, level) in report.iter().enumerate() {
		if let Some(maybe_removed) = maybe_removed {
			if i == maybe_removed {
				continue;
			}
		}
		let Some(last_) = last else {
			last = Some(level);
			continue
		};
		let diff = last_.max(level) - last_.min(level);
		if diff > 3 || diff == 0 {
			if budget == 0 {
				return false;
			}
			budget -= 1;
			continue;
		}
		if let Some(is_increasing) = mode {
			if (last_ < level) != is_increasing {
				if budget == 0 {
					return false;
				}
				budget -= 1;
				continue;
			}
		} else {
			mode = Some(last_ < level);
		}
		last = Some(level);
	}
	true
}

fn number_of_safe_budget(reports: &[Vec<u16>], budget: u8) -> usize {
	reports.iter()
		.filter(|report| is_safe_with_budget(None, &report, budget))
		.count()
}
