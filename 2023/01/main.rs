const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let groups = parse(INPUT);
	println!("sum: {}", sum(&groups));
}

fn parse(input: &str) -> Vec<u8> {
	input.lines()
		.map(|l| {
			let it = l.chars().filter(|c| c.is_ascii_digit());
			let first = it.clone().next().unwrap() as u8 - b'0';
			let last = it.rev().next().unwrap() as u8 - b'0';
			first * 10 + last
		})
		.collect()
}

fn sum(nums: &[u8]) -> u32 {
	nums.iter().map(|v| *v as u32).sum()
}
