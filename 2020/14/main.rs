use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let sum = apply_cmds(&cmds);
	println!("Sum: {sum}");
}

#[derive(Copy, Clone, Debug)]
enum Cmd {
	Mask(u64, u64),
	Mem(u64, u64),
}

fn parse(input :&str) -> Vec<Cmd> {
	input.lines()
		.map(|l| {
			let mut it = l.split(" = ");
			let first = it.next().unwrap();
			let second = it.next().unwrap();
			//println!("{l}");
			if first == "mask" {
				let mut ones = 0;
				let mut zeroes = 0;
				for (i, ch) in second.chars().enumerate() {
					match ch {
						'0' => zeroes |= 1 << (35 - i),
						'1' => ones |= 1 << (35 - i),
						'X' => (),
						_ => panic!("Unexpected char '{ch}'!"),
					}
				}
				Cmd::Mask(ones, zeroes)
			} else {
				let addr_str = first.split(['[', ']'])
					.nth(1)
					.unwrap();
				let addr = u64::from_str(addr_str).unwrap();
				let content = u64::from_str(second).unwrap();
				Cmd::Mem(addr, content)
			}
		})
		.collect::<Vec<_>>()
}

fn apply_cmds(cmds :&[Cmd]) -> u64 {
	let mut mask_zeroes = 0;
	let mut mask_ones = 0;
	let mut memory = HashMap::new();
	for cmd in cmds {
		//println!("{cmd:?}");
		match cmd {
			Cmd::Mem(addr, content) => {
				let c = content | mask_ones;
				let c = c - (c & mask_zeroes);
				//println!("  {addr} <- {c} {c:b} (originally {content})");
				memory.insert(addr, c);
			},
			Cmd::Mask(ones, zeroes) => {
				//println!("  {ones:b} {zeroes:b}");
				mask_ones = *ones;
				mask_zeroes = *zeroes;
			},
		}
	}
	memory.iter()
		.map(|(_addr, content)| content)
		.sum()
}
