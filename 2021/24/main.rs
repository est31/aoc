use std::collections::hash_map::{HashMap, Entry};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse_commands(INPUT);
	let mask = find_mask(&cmds);

	println!("vvvvv---Masked program---vvvvv");
	for cmd in &mask {
		println!("{cmd}");
	}
	println!("^^^^^---Masked program---^^^^^");

	let abcs = extract_abcs(&cmds);

	println!("ABC tuples:");
	for (a, b, c) in &abcs {
		print!("({a}, {b}, {c}), ");
	}
	println!();
	let mut searcher = Searcher::new();
	let (min, max) = searcher.search_with_abcs(&abcs).unwrap();
	println!("Maximum input: {}", input_to_string(&max));
	println!("Minimum input: {}", input_to_string(&min));

}

type Int = i64;

fn input_to_string(input :&[Int]) -> String {
	input.iter().map(|v| v.to_string()).collect::<String>()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Val {
	X,
	Y,
	Z,
	W,
	Lit(Int),
	Placeholder,
}

impl Val {
	fn parse(input :&str) -> Self {
		match input {
			"x" => Val::X,
			"y" => Val::Y,
			"z" => Val::Z,
			"w" => Val::W,
			"*" => Val::Placeholder,
			_ => {
				let Ok(v) = Int::from_str(input) else {
					panic!("Can't parse value '{input}'");
				};
				Val::Lit(v)
			},
		}
	}
}

impl std::fmt::Display for Val {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Val::X => write!(f, "x"),
			Val::Y => write!(f, "y"),
			Val::Z => write!(f, "z"),
			Val::W => write!(f, "w"),
			Val::Lit(l) => write!(f, "{l}"),
			Val::Placeholder => write!(f, "*"),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Command {
	Inp(Val),
	Binop(Binop, Val, Val),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Binop {
	Add,
	Mul,
	Div,
	Mod,
	Eql,
	Idk,
}

impl std::fmt::Display for Binop {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Binop::Add => write!(f, "add"),
			Binop::Mul => write!(f, "mul"),
			Binop::Div => write!(f, "div"),
			Binop::Mod => write!(f, "mod"),
			Binop::Eql => write!(f, "eql"),
			Binop::Idk => write!(f, "***"),
		}
	}
}

impl std::fmt::Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Inp(a) => write!(f, "inp {a}"),
			Command::Binop(op, a, b) => write!(f, "{op} {a} {b}"),
		}
	}
}

fn parse_commands(input :&str) -> Vec<Command> {
	input.lines()
		.map(|l| l.split(';').next().unwrap())
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut words = l.split_whitespace();
			let word = words.next().unwrap();
			let p1 = words.next().map(|p| Val::parse(p));
			let p2 = words.next().map(|p| Val::parse(p));
			let op = match word {
				"inp" => return Command::Inp(p1.unwrap()),
				"add" => Binop::Add,
				"mul" => Binop::Mul,
				"div" => Binop::Div,
				"mod" => Binop::Mod,
				"eql" => Binop::Eql,
				_ => panic!("Unknown word: {word}"),
			};
			Command::Binop(op, p1.unwrap(), p2.unwrap())
		})
		.collect()
}

fn find_mask(input :&[Command]) -> Vec<Command> {
	let mut cmds = Vec::new();

	// First initialize with the first set of commands.
	let mut it = input.iter();
	while let Some(cmd) = it.next() {
		if !cmds.is_empty() && matches!(cmd, Command::Inp(_)) {
			// TODO mask cmd with cmds[0]
			break;
		}
		cmds.push(*cmd);
	}
	assert_eq!(cmds.len(), 18);

	// Now do the masking
	let mut cmd_idx = 1;
	for cmd in it {
		match cmd {
			Command::Inp(v) => {
				cmd_idx = 0;
				if let Command::Inp(c_v) = &mut cmds[cmd_idx] {
					if *c_v != *v {
						*c_v = Val::Placeholder;
					}
				} else {
					panic!("Can't mask '{}' from mask with '{cmd}'", cmds[cmd_idx]);
				}
			},
			Command::Binop(op, p1, p2) => {
				if let Command::Binop(c_op, c_p1, c_p2) = &mut cmds[cmd_idx] {
					if *c_op != *op {
						*c_op = Binop::Idk;
					}
					if *c_p1 != *p1 {
						*c_p1 = Val::Placeholder;
					}
					if *c_p2 != *p2 {
						*c_p2 = Val::Placeholder;
					}
				} else {
					panic!("Can't mask '{}' from mask with '{cmd}'", cmds[cmd_idx]);
				}
			},
		}
		cmd_idx += 1;
	}
	cmds
}

/*

The program is made up of 14 segments/blocks that look like this
(obtained by the masking routine):

inp w
mul x 0
add x z
mod x 26
div z *
add x *
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y *
mul y x
add z y

Where * indicates a constant that is different in each segment/block.

We can add comments to this, using A, B, C for the constants,
and Z indicates the z from the prior segment (all other registers are zeroed
before first use in a segment):

inp w        ; Read number into w. Never modified in a segment, only read!
mul x 0
add x z
mod x 26
div z *      ; z = Z / A
add x *      ; x = (Z % 26) + B
eql x w
eql x 0      ; set x to 0 if w == x, otherwise to 1.
mul y 0
add y 25
mul y x
add y 1
mul z y      ; z = z * (x * 25 + 1)
mul y 0
add y w
add y *
mul y x
add z y      ; z = z + x * (w + C)

So in combination:

* Read into w
* set x = (w != (Z % 26) + B) as integer
* set z = (Z / A) * (x * 25 + 1) + x * (w + C)

A is either 1 or 26, and C is always positive. B is positive if A is 1, negative if A is 26.

Solve for Z (assume A is 1):

Z = (x * (w + C) - z) / (x * 25 + 1)

The division needs to actually work. x depends on Z yes, but we can just
put in both possible values, arrive at two Z's and then check whether they line up with the x we provided.

For the case of A being 26, we have:

Z = 26 * (x * (w + C) - z) / (x * 25 + 1) + s*R

where 0 <= R < 26 and s is 1 or -1, depending on the sign of the thing on the left. We still have to use the obtained Z to check whether the x we tried is consistent or not.
*/

type Abc = (Int, Int, Int);

fn extract_abcs(input :&[Command]) -> Vec<Abc> {
	let (mut a, mut b) = (0, 0);
	let mut abcs = Vec::new();
	for (i, cmd) in input.iter().enumerate() {
		let offs = i % 18;
		let v = if let Command::Binop(_, _, Val::Lit(v)) = cmd {
			Some(v)
		} else {
			None
		};
		//println!("{cmd}");
		match offs {
			4 => a = *v.unwrap(),
			5 => b = *v.unwrap(),
			15 => {
				let c = *v.unwrap();
				abcs.push((a, b, c));
			},
			_ => {},
		}
	}
	abcs
}

fn find_fitting_for_abc_input(end_z :Int, input :Int, (a, b, c) :Abc, mut f :impl FnMut(Int, Int)) {
	let get_x = |z| {
		(input != ((z % 26) + b)) as Int
	};
	match a {
		1 => {
			for x in 0..=1 {
				let em = end_z - x * (input + c);
				let ed = x * 25 + 1;
				if em % ed != 0 {
					continue;
				}
				let z = em / ed;
				if get_x(z) == x {
					f(input, z);
				}
			}
		},
		26 => {
			for x in 0..=1 {
				let em = end_z - x * (input + c);
				let ed = x * 25 + 1;
				if em % ed != 0 {
					continue;
				}
				let e = 26 * em / ed;
				//println!("  -> w={input} Z={end_z} e={e}");
				let (mul, range) = if e == 0 {
					(1, -25..26)
				} else {
					(e.signum(), 1..26)
				};
				for r in range {
					let z = e + mul * r;
					if get_x(z) == x {
						f(input, z);
					}
				}
			}
		},
		_ => panic!("unexpected A: {a}"),
	}
}

fn find_fitting_for_abc(end_z :Int, abc :Abc, mut f :impl FnMut(Int, Int)) {
	for input in 1..=9 {
		find_fitting_for_abc_input(end_z, input, abc, &mut f);
	}
}


struct Searcher {
	zs_that_terminate :HashMap<Int, (Vec<Int>, Vec<Int>)>,
}

impl Searcher {
	fn new() -> Self {
		let mut zs_that_terminate = HashMap::new();
		zs_that_terminate.insert(0, (vec![], vec![]));
		Self {
			zs_that_terminate,
		}
	}
	fn step(&mut self, abc :Abc) {
		let mut new_zs = HashMap::<_, (Vec<_>, Vec<_>)>::new();
		for (z, (inp_list_min, inp_list_max)) in self.zs_that_terminate.iter() {
			find_fitting_for_abc(*z, abc, |input, z| {
				let mut n_l_min = inp_list_min.clone();
				n_l_min.insert(0, input);
				let mut n_l_max = inp_list_max.clone();
				n_l_max.insert(0, input);
				match new_zs.entry(z) {
					Entry::Occupied(mut o) => {
						let (min, max) = o.get().clone();
						let min = min.min(n_l_min);
						let max = max.max(n_l_max);
						o.insert((min, max));
					},
					Entry::Vacant(v) => {
						v.insert((n_l_min, n_l_max));
					},
				}
			});
		}
		self.zs_that_terminate = new_zs;
	}
	fn search_with_abcs(&mut self, abcs :&[Abc]) -> Option<(&[Int], &[Int])> {
		for abc in abcs.iter().rev() {
			self.step(*abc);
		}
		self.maximum_for_zero()
	}
	fn maximum_for_zero(&self) -> Option<(&[Int], &[Int])> {
		self.zs_that_terminate.get(&0)
			.map(|(v0, v1)| (v0.as_slice(), v1.as_slice()))
	}
}


#[derive(Copy, Clone)]
#[cfg(test)]
struct Alu {
	x :Int,
	y :Int,
	w :Int,
	z :Int,
}

#[cfg(test)]
impl Alu {
	fn new() -> Self {
		Self {
			x : 0,
			y : 0,
			w : 0,
			z : 0,
		}
	}
	fn get_mut(&mut self, val :Val) -> &mut Int {
		match val {
			Val::X => &mut self.x,
			Val::Y => &mut self.y,
			Val::Z => &mut self.z,
			Val::W => &mut self.w,
			Val::Lit(_) | Val::Placeholder => panic!("Didn't expect '{val}' at lhs"),
		}
	}
	fn get(&self, val :Val) -> Int {
		match val {
			Val::X => self.x,
			Val::Y => self.y,
			Val::Z => self.z,
			Val::W => self.w,
			Val::Lit(v) => v,
			Val::Placeholder => panic!("Didn't expect placeholder!"),
		}
	}
	fn do_cmd(&mut self, cmd :Command, input :Option<Int>) -> bool {
		match cmd {
			Command::Inp(val) => {
				*self.get_mut(val) = input.expect("Tried to read input but none provided");
				true
			},
			Command::Binop(op, v1, v2) => {
				let val_2 = self.get(v2);
				let val_1 = self.get_mut(v1);
				let res = match op {
					Binop::Add => (*val_1).checked_add(val_2),
					Binop::Mul => (*val_1).checked_mul(val_2),
					Binop::Div => Some(*val_1 / val_2),
					Binop::Mod => Some(*val_1 % val_2),
					Binop::Eql => Some((*val_1 == val_2) as Int),
					Binop::Idk => None,
				};
				let Some(res) = res else {
					panic!("Couldn't execute binop '{cmd}' with inputs '{val_1}' and '{val_2}'");
				};
				*val_1 = res;
				false
			},
		}
	}
	fn run_cmds_with_input(&mut self, cmds :&[Command], input :&[Int]) -> Int {
		let mut input_iter = input.iter().peekable();
		for cmd in cmds {
			if self.do_cmd(*cmd, input_iter.peek().map(|i| **i)) {
				input_iter.next();
			}
		}
		self.z
	}
}
