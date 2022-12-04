use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let lines = parse(INPUT);
	let cont = contained_pairs(&lines);
	println!("contained pairs: {cont}");
}

fn parse(input :&str) -> Vec<[[u16; 2]; 2]> {
	input.lines()
		.map(|l| {
			let nums = l.split([',', '-'])
				.map(|ns| {
					match u16::from_str(ns) {
						Ok(n) => n,
						Err(_) => panic!("Invalid number '{ns}'"),
					}
				})
				.collect::<Vec<_>>();
			[[nums[0], nums[1]], [nums[2], nums[3]]]
		})
		.collect::<Vec<_>>()
}

fn first_contains_second(a1 :&[u16; 2], a2 :&[u16; 2]) -> bool {
	(a1[0] <= a2[0]) && (a1[1] >= a2[1])
}

fn contained_pairs(lines :&[[[u16; 2]; 2]]) -> usize {
	lines.iter()
		.filter(|[a1, a2]| {
			first_contains_second(a1, a2) ||
			first_contains_second(a2, a1)
		})
		.count()
}
