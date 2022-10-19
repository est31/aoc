use std::collections::HashSet;
use std::str::FromStr;
use std::mem::swap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut instructions = parse(INPUT);
	let v = exec_until_repetition(&instructions);
	println!("Value at first repetition: {v}");
	let w = find_instruction_to_flip(&mut instructions);
	println!("Value at termination after fix: {w}");
}

#[derive(Copy, Clone)]
enum Ins {
	Acc,
	Nop,
	Jmp,
}

fn parse(input :&str) -> Vec<(Ins, i32)> {
	input.lines()
		.map(|l| {
			let mut words = l.split_whitespace();
			let first = words.next();
			let second = words.next().unwrap();
			let v = i32::from_str(second).unwrap();
			let ins = match first {
				Some("acc") => Ins::Acc,
				Some("nop") => Ins::Nop,
				Some("jmp") => Ins::Jmp,
				_ => panic!("Invalid line '{l}'"),
			};
			(ins, v)
		})
		.collect()
}

fn exec_until_repetition_or_end(instructions :&[(Ins, i32)]) -> (i32, bool) {
	let mut visited = HashSet::new();
	let mut i = 0;
	let mut acc = 0;
	while let Some((ins, v)) = instructions.get(i) {
		if !visited.insert(i) {
			return (acc, true);
		}
		match ins {
			Ins::Acc => {
				acc += v;
			},
			Ins::Nop => (),
			Ins::Jmp => {
				i = (i as i32 + v) as usize;
				continue;
			},
		}
		i += 1;
	}
	(acc, false)
}

fn exec_until_repetition(instructions :&[(Ins, i32)]) -> i32 {
	let r = exec_until_repetition_or_end(instructions);
	if !r.1 {
		panic!("Program terminated normally!");
	}
	r.0
}

fn find_instruction_to_flip(instructions :&mut [(Ins, i32)]) -> i32 {
	for i in 0..instructions.len() {
		let mut other = match instructions[i].0 {
			Ins::Jmp => Ins::Nop,
			Ins::Nop => Ins::Jmp,
			_ => continue
		};
		swap(&mut instructions[i].0, &mut other);
		let res = exec_until_repetition_or_end(&instructions);
		if !res.1 {
			return res.0;
		}
		swap(&mut instructions[i].0, &mut other);
	}
	panic!("No fix found");
}
