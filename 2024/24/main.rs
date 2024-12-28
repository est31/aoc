use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let gts = parse(INPUT);
	println!("gates output: {}", gts.eval());
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
		//if false
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
	fn eval_one(&self, wire :usize, values :&mut HashMap<usize, bool>) -> bool {
		if let Some(v) = values.get(&wire) {
			return *v;
		}
		let (id_l, binop, id_r) = self.gates[&wire];
		let l = self.eval_one(id_l, values);
		let r = self.eval_one(id_r, values);
		let v = binop.eval(l, r);
		values.insert(wire, v);
		v
	}
	fn eval(&self) -> u32 {
		let mut values = self.inputs.clone();
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
			let val = self.eval_one(id, &mut values) as u32;
			dprint!("wire {name} is {val}\n");
			res |= val << sh;
			sh += 1;
		}
		dprint!("res is 0b{res:b}\n");
		res
	}
}
