const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let components = parse(INPUT);
	println!("results sum: {}", sum_hashes(&components));
}

fn parse(input :&str) -> Vec<String> {
	input.split(',')
		.map(|c| c.trim().to_owned())
		.collect::<Vec<_>>()
}

fn hash(s :&str) -> u32 {
	let mut v = 0;
	for ch in s.chars() {
		v += ch as u32;
		v *= 17;
		v %= 256;
	}
	v
}

fn sum_hashes(components :&[String]) -> u32 {
	components.iter()
		.map(|s| hash(s))
		.sum::<u32>()
}
