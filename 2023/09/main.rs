use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	println!("sum extrapolated (end): {}", extrapolate_lines(&lines, false));
	println!("sum extrapolated (start): {}", extrapolate_lines(&lines, true));
}

fn parse(input :&str) -> Vec<Vec<i32>> {
	input.lines()
		.map(|l|{
			l.trim()
				.split_whitespace()
				.map(|c| i32::from_str(c).unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn extrapolate(history: &[i32], start :bool) -> i32 {
	if history.iter().all(|v| *v == 0) {
		return 0;
	}
	let next = history.windows(2)
		.map(|h| {
			h[1].checked_sub(h[0]).unwrap()
		})
		.collect::<Vec<i32>>();
	let extra = extrapolate(&next, start);
	if start {
		let first = history.first().unwrap();
		first - extra
	} else {
		let last = history.last().unwrap();
		last + extra
	}
}

fn extrapolate_lines(lines :&[Vec<i32>], start :bool) -> i32 {
	lines.iter()
		.map(|line| extrapolate(line, start))
		.sum()
}
