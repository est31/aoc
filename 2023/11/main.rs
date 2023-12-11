use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let galaxies = parse(INPUT);
	println!("sum shortest: {}", sum_shortest(&galaxies));
}

fn parse(input :&str) -> HashSet<(u16, u16)> {
	input.lines()
		.enumerate()
		.map(|(i, l)|{
			l.trim()
				.chars()
				.enumerate()
				.filter(|(_j, c)| *c == '#')
				.map(move |(j, _c)| (i as u16, j as u16))
		})
		.flatten()
		.collect::<HashSet<_>>()
}

fn find_not(it :impl Iterator<Item = u16>) -> Vec<u16> {
	let nums = it.collect::<HashSet<_>>();
	let mut num_vec = nums.into_iter().collect::<Vec<_>>();
	num_vec.sort();
	num_vec.windows(2)
		.map(|w| (w[0] + 1)..w[1])
		.flatten()
		.collect::<Vec<_>>()
}

fn expand_with_not(v :u16, not :&[u16]) -> u16 {
	let prev_idx = not.binary_search(&v).unwrap_err();
	prev_idx as u16 + v
}

fn sum_shortest(galaxies :&HashSet<(u16, u16)>) -> u32 {
	let lines_not = find_not(galaxies.iter().map(|(i, _j)| *i));
	let cols_not = find_not(galaxies.iter().map(|(_i, j)| *j));

	let mut galaxies_expanded = galaxies.iter()
		.map(|pos| {
			let i = expand_with_not(pos.0, &lines_not);
			let j = expand_with_not(pos.1, &cols_not);
			(i, j)
		})
		.map(|pos| (pos.0 as i32, pos.1 as i32))
		.collect::<Vec<(_, _)>>();
	// Not really needed but if we need to do logging, this sort is helpful
	galaxies_expanded.sort();
	let mut sum = 0;
	for (i1, g1) in galaxies_expanded.iter().enumerate() {
		for g2 in &galaxies_expanded[..i1] {
			let l0_dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
			sum += l0_dist;
		}
	}
	sum as u32
}
