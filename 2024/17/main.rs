use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmp = parse(INPUT);
	println!("output: {}", cmp.output());
}

#[derive(Clone, PartialEq, Eq)]
struct Computer {
	register_a :i64,
	register_b :i64,
	register_c :i64,

	ip :usize,
	program :Vec<u8>,
}

fn parse(s :&str) -> Computer {
	let mut lines = s.trim().lines().map(str::trim);
	let reg = (&mut lines).take(3).map(|l| {
		let mut s = l.split(": ");
		_ = s.next().unwrap();
		i64::from_str(s.next().unwrap()).unwrap()
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
		//if false
			{ print!($($args),*); }
	};
}

impl Computer {
	fn step(&mut self) -> Option<Option<String>> {
		if self.ip >= self.program.len() {
			return None;
		}
		let opcode = self.program[self.ip];
		let op_lit = self.program[self.ip + 1] as i64;
		let op_combo = match op_lit {
			0..=3 | 7 => op_lit as i64,
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
				self.register_a = self.register_a / (1 << op_lit);
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
				output = Some(format!("{}", op_combo % 8));
				dprint!("--> out: {}\n", output.clone().unwrap());
			},
			6 => {
				// bdv
				self.register_b = self.register_a / (1 << op_lit);
				dprint!("--> bdv: b={}\n", self.register_b);
			},
			7 => {
				// cdv
				self.register_c = self.register_a / (1 << op_lit);
				dprint!("--> cdv: c={}\n", self.register_c);
			},
			_ => panic!("Unexpected opcode {opcode}"),
		}
		if !did_jump {
			self.ip += 2;
		}
		Some(output)
	}
	fn output(&self) -> String {
		let mut res = Vec::new();
		let mut cl = self.clone();
		while let Some(st) = cl.step() {
			if let Some(st) = st {
				res.push(st);
			}
		}
		res.join(",")
	}
}
