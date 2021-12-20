const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut octopuses = parse_octopuses(INPUT);
	println!("Steps: {}", octopuses_steps(&mut octopuses, 100));
	let mut octopuses = parse_octopuses(INPUT);
	println!("First total blink: {}", octopuses_first_total_blink(&mut octopuses));
}

fn parse_octopuses(input :&str) -> Vec<Vec<u8>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| l.chars()
			.map(|c| {
				if !('0'..='9').contains(&c) {
					panic!("encountered unsupported char {}", c);
				}
				c as u8 - b'0'
			})
			.collect::<Vec<_>>())
		.collect()
}

fn octopuses_step(area :&mut [Vec<u8>]) -> u32 {
	area.iter_mut()
		.for_each(|l| l.iter_mut().for_each(|o| *o += 1));
	let mut blunken = vec![vec![false; area[0].len()]; area.len()];
	let mut flash_counter = 0;
	while {
		let mut had_blinkers = false;
		for y in 0..area.len() {
			for x in 0..area[y].len() {
				if area[y][x] < 10 {
					continue;
				}
				area[y][x] = 0;
				blunken[y][x] = true;
				had_blinkers = true;
				flash_counter += 1;
				let x_min = if x > 0 {
					x - 1
				} else {
					x
				};
				let x_max = if x < area[y].len() - 1 {
					x + 1
				} else {
					x
				};
				let y_min = if y > 0 {
					y - 1
				} else {
					y
				};
				let y_max = if y < area.len() - 1 {
					y + 1
				} else {
					y
				};
				for xn in x_min..=x_max {
					for yn in y_min..=y_max {
						if (x, y) == (xn, yn) {
							continue;
						}
						if blunken[yn][xn] {
							continue;
						}
						area[yn][xn] += 1;
					}
				}
			}
		}
		had_blinkers
	} {}
	flash_counter
}

fn octopuses_steps(area :&mut [Vec<u8>], steps :usize) -> u32 {
	(0..steps).map(|_| octopuses_step(area)).sum()
}

fn octopuses_first_total_blink(area :&mut [Vec<u8>]) -> usize {
	for st in 1.. {
		if octopuses_step(area) == 100 {
			return st;
		}
	}
	panic!("not supposed to happen")
}
