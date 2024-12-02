use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let reports = parse_reports(INPUT);
	println!("numbers of safe: {}", number_of_safe(&reports));
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
	reports.iter()
		.filter(|report| {
			let mut mode = None;
			let mut last = None;
			for level in report.iter()  {
				let Some(last_) = last else {
					last = Some(level);
					continue
				};
				let diff = last_.max(level) - last_.min(level);
				if diff > 3 || diff == 0 {
					return false;
				}
				if let Some(is_increasing) = mode {
					if (last_ < level) != is_increasing {
						return false;
					}
				} else {
					mode = Some(last_ < level);
				}
				last = Some(level);
			}
			true
		})
		.count()
}
