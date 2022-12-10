use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let sum = signal_strength_sum(&cmds);
	println!("signal strength sum: {}", sum);
}

#[derive(Debug)]
enum Cmd {
	Addx(i32),
	Nop,
}

fn parse(input :&str) -> Vec<Cmd> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			if l == "noop" {
				Cmd::Nop
			} else if l.starts_with("addx ") {
				let mut words = l.split_whitespace();
				let (Some(_), Some(v)) = (words.next(), words.next()) else {
					panic!("invalid addx line '{l}'");
				};
				let amnt = i32::from_str(v).unwrap();
				Cmd::Addx(amnt)
			} else {
				panic!("invalid line '{l}'");
			}
		})
		.collect::<Vec<_>>()
}

fn signal_strength_sum(cmds :&[Cmd]) -> u32 {
	let mut pc = 0;
	let mut x_val = 1;
	let mut sum = 0;
	for cmd in cmds {
		let pc_inc = match cmd {
			Cmd::Addx(_amnt) => 2,
			Cmd::Nop => 1,
		};
		for _ in 0..pc_inc {
			pc += 1;
			if [20, 60, 100, 140, 180, 220].contains(&pc) {
				let strength = pc * x_val;
				//println!("Strength at {pc} is: {pc}*{x_val}={strength}");
				sum += strength;
			}
		}
		if let Cmd::Addx(amnt) = cmd {
			x_val += amnt;
		}
	}
	sum.try_into().unwrap()
}
