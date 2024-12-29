use std::collections::{HashSet, HashMap};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let gts = parse(INPUT);
	println!("gates output: {}", gts.eval());
	println!("swaps for correct: {}", gts.swaps_for_correct());
}

fn parse(s :&str) -> Gates {
	let intern = |nti :&mut HashMap<_, _>, id_to_name :&mut HashMap<_, _>, s :&str| {
		let nti_len = nti.len();
		let id = *nti.entry(s.to_owned()).or_insert(nti_len);
		id_to_name.insert(id, s.to_owned());
		id
	};
	let mut name_to_id = HashMap::new();
	let mut id_to_name = HashMap::new();
	let mut lines = s.trim()
		.lines()
		.map(str::trim);

	let mut inputs = HashMap::<usize, _>::new();
	while let Some(l) = lines.next() {
		if l.is_empty() {
			break;
		}
		let mut cmps = l.split(": ");
		let id = intern(&mut name_to_id, &mut id_to_name, cmps.next().unwrap());
		let val = match cmps.next().unwrap() {
			"1" => true,
			"0" => false,
			s => panic!("Invalid input value: {s}"),
		};
		inputs.insert(id, val);
	}

	let mut gates = HashMap::<usize, _>::new();
	while let Some(l) = lines.next() {
		let mut in_out = l.split(" -> ");
		let in_ = in_out.next().unwrap();
		let out = in_out.next().unwrap();
		let out_id = intern(&mut name_to_id, &mut id_to_name, out);
		let mut cmps = in_.split(" ");
		let l = cmps.next().unwrap();
		let l_id = intern(&mut name_to_id, &mut id_to_name, l);
		let binop_str = cmps.next().unwrap();
		let binop = match binop_str {
			"AND" => BinOp::And,
			"OR" => BinOp::Or,
			"XOR" => BinOp::Xor,
			_ => panic!("invalid: '{binop_str}'"),
		};
		let r = cmps.next().unwrap();
		let r_id = intern(&mut name_to_id, &mut id_to_name, r);
		gates.insert(out_id, (l_id, binop, r_id));
	}

	Gates {
		name_to_id,
		id_to_name,
		inputs,
		gates,
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum BinOp {
	And,
	Or,
	Xor,
}

impl BinOp {
	fn eval(&self, l :bool, r :bool) -> bool {
		match self {
			BinOp::And => l & r,
			BinOp::Or => l | r,
			BinOp::Xor => l ^ r,
		}
	}

}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

#[derive(Clone)]
struct Gates {
	#[allow(dead_code)]
	name_to_id :HashMap<String, usize>,
	id_to_name :HashMap<usize, String>,
	inputs :HashMap<usize, bool>,
	gates :HashMap<usize, (usize, BinOp, usize)>,
}

impl Gates {
	fn eval_one(&self, wire :usize, values :&mut HashMap<usize, bool>, on_stack :&mut HashSet<usize>) -> Option<bool> {
		if let Some(v) = values.get(&wire) {
			return Some(*v);
		}
		if !on_stack.insert(wire) {
			// cycle
			return None;
		}
		let (id_l, binop, id_r) = self.gates[&wire];
		let l = self.eval_one(id_l, values, on_stack)?;
		let r = self.eval_one(id_r, values, on_stack)?;
		let v = binop.eval(l, r);
		values.insert(wire, v);
		Some(v)
	}
	fn eval(&self) -> u64 {
		self.eval_with_inputs(self.inputs.clone()).unwrap()
	}
	fn eval_with_inputs(&self, inputs :HashMap<usize, bool>) -> Option<u64> {
		let mut values = inputs;
		let mut on_stack = HashSet::new();
		let mut names_sorted = self.id_to_name.iter()
			.map(|(id, name)| (*id, name.clone()))
			.collect::<Vec<_>>();
		names_sorted.sort_by_key(|tup| tup.1.clone());

		let mut res = 0;
		let mut sh = 0;
		for (id, name) in names_sorted.into_iter() {
			if !name.starts_with("z") {
				continue;
			}
			let val = self.eval_one(id, &mut values, &mut on_stack)? as u64;
			//dprint!("wire {name} is {val}\n");
			res |= val << sh;
			sh += 1;
		}
		//dprint!("res is 0b{res:b}\n");
		Some(res)
	}
	fn output_for(&self, x :u64, y :u64) -> Option<u64> {
		let bit_count = 45;
		let x_it = (0..bit_count).map(|sh| {
			(sh, x & (1 << sh) != 0)
		});
		let y_it = (0..bit_count).map(|sh| {
			(sh + bit_count, y & (1 << sh) != 0)
		});
		let inputs = x_it.chain(y_it)
			.collect::<HashMap<_, _>>();
		let res = self.eval_with_inputs(inputs)?;
		//dprint!("res is 0b{res:b}\n");
		Some(res)
	}
	fn find_errors(&self) -> Option<u32> {
		let mask = (1u64 << 45) - 1;
		let cnt = 1 << 3;

		let mut err_count = 0;
		for sh in 0..45 {
			for x in 0..cnt {
				for y in 0..cnt {
					let x = (x << sh) & mask;
					let y = (y << sh) & mask;
					let expected = x + y;
					let o = self.output_for(x, y)?;
					if o != expected {
						err_count += 1;
						//dprint!("found error: b{x:045b} + b{y:045b} = b{expected:045b}, got b{o:045b}\n");
					} else {
						//dprint!("expected b{o:045b}\n");
					}
				}
			}
		}
		Some(err_count)
	}
	fn swaps_for_correct(&self) -> String {
		let mut swapped = Vec::new();
		let mut errs_min = self.find_errors().unwrap();
		let mut cl = self.clone();
		// Simple greedy algorithm
		for (a_id, _) in self.gates.iter() {
			let a_name = &cl.id_to_name[&a_id];
			for (b_id, _) in self.gates.iter() {
				let b_name = &cl.id_to_name[&b_id];
				dprint!("  trying {} <-> {}\n", a_name, b_name);
				if a_id == b_id { continue }

				let tmp = cl.gates[&a_id];
				cl.gates.insert(*a_id, cl.gates[&b_id]);
				cl.gates.insert(*b_id, tmp);

				if let Some(errs_swapped) = cl.find_errors() {
					if errs_swapped < errs_min {
						dprint!("New swap pair {}<->{}: {errs_min} > {errs_swapped}\n", a_name, b_name);
						swapped.push(cl.id_to_name[&a_id].to_owned());
						swapped.push(cl.id_to_name[&b_id].to_owned());
						errs_min = errs_swapped;
						break;
					}
				}

				let tmp = cl.gates[&a_id];
				cl.gates.insert(*a_id, cl.gates[&b_id]);
				cl.gates.insert(*b_id, tmp);
			}
		}
		assert_eq!(cl.find_errors(), Some(0));
		swapped.sort();
		swapped.join(",")
	}
}
