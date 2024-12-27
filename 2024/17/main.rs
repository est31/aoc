use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmp = parse(INPUT);
	println!("output: {}", cmp.output());
	println!("lowest A for quine: {}", cmp.lowest_a_for_quine_build());
}

#[derive(Clone, PartialEq, Eq)]
struct Computer {
	register_a :u64,
	register_b :u64,
	register_c :u64,

	ip :usize,
	program :Vec<u8>,
}

fn parse(s :&str) -> Computer {
	let mut lines = s.trim().lines().map(str::trim);
	let reg = (&mut lines).take(3).map(|l| {
		let mut s = l.split(": ");
		_ = s.next().unwrap();
		u64::from_str(s.next().unwrap()).unwrap()
	})
	.collect::<Vec<_>>();

	assert_eq!(lines.next().unwrap(), "");

	let program = lines.next().unwrap();
	let program = program.strip_prefix("Program: ").unwrap();

	let program = program.split(',')
		.map(|v| u8::from_str(v).unwrap())
		.collect::<Vec<_>>();

	Computer {
		register_a : reg[0],
		register_b : reg[1],
		register_c : reg[2],
		ip :0,
		program,
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

impl Computer {
	fn step(&mut self) -> Option<Option<u8>> {
		if self.ip >= self.program.len() {
			return None;
		}
		let opcode = self.program[self.ip];
		let op_lit = self.program[self.ip + 1] as u64;
		let op_combo = match op_lit {
			0..=3 | 7 => op_lit as u64,
			4 => self.register_a,
			5 => self.register_b,
			6 => self.register_c,
			_ => panic!("Unexpected operand {op_lit}"),
		};
		dprint!("opcode {opcode}: op_lit: {op_lit} op_cmb: {op_combo} ");
		let mut output = None;
		let mut did_jump = false;
		match opcode {
			0 => {
				// adv
				self.register_a = self.register_a / (1 << op_combo);
				dprint!("--> adv: a={}\n", self.register_a);
			},
			1 => {
				// bxl
				self.register_b = self.register_b ^ op_lit;
				dprint!("--> bxl: b={}\n", self.register_b);
			},
			2 => {
				// bst
				self.register_b = op_combo % 8;
				dprint!("--> bst: b={}\n", self.register_b);
			},
			3 => {
				// jnz
				if self.register_a == 0 {
					dprint!("--> jnz: a: 0\n");
				} else {
					dprint!("--> jnz: a: {} -> jump to {op_lit}\n", self.register_a);
					self.ip = op_lit as usize;
					did_jump = true;
				}
			},
			4 => {
				// bxc
				self.register_b = self.register_b ^ self.register_c;
				dprint!("--> bxc: b={}\n", self.register_b);
			},
			5 => {
				// out
				output = Some((op_combo % 8) as u8);
				dprint!("--> out: {}\n", output.clone().unwrap());
			},
			6 => {
				// bdv
				self.register_b = self.register_a / (1 << op_combo);
				dprint!("--> bdv: b={}\n", self.register_b);
			},
			7 => {
				// cdv
				self.register_c = self.register_a / (1 << op_combo);
				dprint!("--> cdv: c={}\n", self.register_c);
			},
			_ => panic!("Unexpected opcode {opcode}"),
		}
		if !did_jump {
			self.ip += 2;
		}
		Some(output)
	}
	fn output_mut_inner(&mut self, v :&mut Vec<u8>) {
		while let Some(st) = self.step() {
			if let Some(st) = st {
				v.push(st);
			}
		}
	}
	fn output_mut(&mut self) -> Vec<u8> {
		let mut res = Vec::new();
		self.output_mut_inner(&mut res);
		res
	}
	fn output(&self) -> String {
		self.clone().output_mut()
			.into_iter()
			.map(|v| format!("{v}"))
			.collect::<Vec<_>>()
			.join(",")
	}
	#[allow(unused)]
	fn lowest_a_for_quine_bf(&self) -> u64 {
		let mut cl = self.clone();
		let mut tmp = Vec::new();
		let mut largest_len = 0;
		for a in 0.. {
			if a % 10_000_000 == 0 {
				print!("a: {} ll: {largest_len}\n", a / 1_000_000);
				if a == 10_000_000 {
					//break;
				}
				if largest_len > self.program.len() {
					break;
				}
			}
			cl.register_a = a;
			cl.register_b = self.register_b;
			cl.register_c = self.register_c;
			cl.ip = self.ip;
			cl.output_mut_inner(&mut tmp);
			largest_len = largest_len.max(tmp.len());
			if &tmp == &self.program {
				return a;
			}
			tmp.clear();
		}
		panic!("not found")
	}
	fn lowest_a_for_quine_build(&self) -> u64 {
		let mut cl = self.clone();
		let mut tmp = Vec::new();
		let mut v = 0;
		'outer: for len in 2..=self.program.len() {
			for next_a in 0..256 {
				let a = v + next_a;
				cl.register_a = a;
				cl.register_b = self.register_b;
				cl.register_c = self.register_c;
				cl.ip = self.ip;
				tmp.clear();
				cl.output_mut_inner(&mut tmp);
				dprint!("    len: {len}, a: {a}, tmp: {tmp:?}\n");
				if len <= tmp.len() && self.program.ends_with(&tmp) {
					dprint!("        -> continue\n");
					if tmp.len() == self.program.len() {
						return a;
					}
					v = a * 8;
					continue 'outer;
				}
			}
			panic!("couldn't find any number to add here. len: {len}");
		}
		panic!("not found");
	}
}
