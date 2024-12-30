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

	let mut gates_hm = HashMap::<usize, _>::new();
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
		gates_hm.insert(out_id, (l_id, binop, r_id));
	}
	let mut gates = vec![None; id_to_name.len()];
	for (id, g) in gates_hm {
		gates[id] = Some(g);
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
	gates :Vec<Option<(usize, BinOp, usize)>>,
}

impl Gates {
	fn eval_one(&self, wire :usize, values :&mut Vec<Option<bool>>, check_on_stack :&mut impl FnMut(usize) -> bool) -> Option<bool> {
		if let Some(v) = values[wire] {
			return Some(v);
		}
		if check_on_stack(wire) {
			// cycle
			return None;
		}
		let (id_l, binop, id_r) = self.gates[wire].unwrap();
		let l = self.eval_one(id_l, values, check_on_stack)?;
		let r = self.eval_one(id_r, values, check_on_stack)?;
		let v = binop.eval(l, r);
		values[wire] = Some(v);
		Some(v)
	}
	fn eval(&self) -> u64 {
		let inputs = self.mk_inputs();
		self.eval_with_inputs_nc(inputs)
	}
	fn mk_inputs(&self) -> Vec<Option<bool>> {
		let mut inputs = vec![None; self.id_to_name.len()];
		for (inp, b) in self.inputs.iter() {
			inputs[*inp] = Some(*b);
		}
		inputs
	}
	fn eval_with_inputs_opt(&self, inputs :Vec<Option<bool>>) -> Option<u64> {
		let mut on_stack = HashSet::new();
		self.eval_with_inputs_inner(inputs, &mut |w| !on_stack.insert(w))
	}
	fn eval_with_inputs_nc(&self, inputs :Vec<Option<bool>>) -> u64 {
		self.eval_with_inputs_inner(inputs, &mut |_| false).unwrap()
	}
	fn eval_with_inputs_inner(&self, inputs :Vec<Option<bool>>, check_on_stack :&mut impl FnMut(usize) -> bool) -> Option<u64> {
		let mut values = inputs;
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
			let val = self.eval_one(id, &mut values, check_on_stack)? as u64;
			//dprint!("wire {name} is {val}\n");
			res |= val << sh;
			sh += 1;
		}
		//dprint!("res is 0b{res:b}\n");
		Some(res)
	}
	fn output_for(&self, x :u64, y :u64) -> Option<u64> {
		let bit_count = 45;
		let mut inputs = vec![None; self.id_to_name.len()];
		for sh in 0..bit_count {
			inputs[sh] = Some(x & (1 << sh) != 0);
			inputs[sh + bit_count] = Some(y & (1 << sh) != 0);
		}
		let res = self.eval_with_inputs_nc(inputs);
		//dprint!("res is 0b{res:b}\n");
		Some(res)
	}
	fn find_errors(&self, errs_min :(u64, u32)) -> Option<(u64, u32)> {
		let mask = (1u64 << 45) - 1;
		let cnt = 4;

		let inputs = self.mk_inputs();
		self.eval_with_inputs_opt(inputs)?;

		let mut error_mask = 0;
		for sh in 0..45 {
			let x = 1;
			let y = 1;
			let x_sh = (x << sh) & mask;
			let y_sh = (y << sh) & mask;
			let expected = x_sh + y_sh;
			let o = self.output_for(x_sh, y_sh)?;
			if o != expected {
				error_mask |= x_sh;
				// Don't even tolerate it if the err count is only *reached*
				if error_mask & (!errs_min.0) != 0 || error_mask == errs_min.0 {
					//dprint!(" sh: {sh} fast reject");
					return None;
				}
			}
		}
		let fast_err_cnt = error_mask;

		let mut err_count = 0;

		for sh in 0..45 {
			for x in 0..cnt {
				for y in 0..cnt {
					if (x, y) == (1, 1) { continue }
					if (x, y) == (0, 0) && sh > 0 { continue }
					let x_sh = (x << sh) & mask;
					let y_sh = (y << sh) & mask;
					let expected = x_sh + y_sh;
					let o = self.output_for(x_sh, y_sh)?;
					if o != expected {
						err_count += 1;
						if err_count > errs_min.1 {
							//dprint!(" sh: {sh}, x: {x}, y: {y}");
							return None;
						}
					}
				}
			}
		}
		Some((fast_err_cnt, err_count))
	}
	fn swap(&mut self, a_id :usize, b_id :usize) {
		self.gates.swap(a_id, b_id);
	}
	fn swaps_for_correct(&self) -> String {
		let mut swapped = Vec::new();
		let mut errs_min = self.find_errors((u64::MAX, u32::MAX)).unwrap();
		let mut cl = self.clone();

		let mut names_sorted = self.id_to_name.iter()
			.filter(|(id, _name)| self.gates[**id].is_some())
			.map(|(id, name)| (*id, name.clone()))
			.collect::<Vec<_>>();
		names_sorted.sort_by_key(|tup| tup.1.clone());
		names_sorted.reverse();

		let mut swaps_elig = Vec::new();
		// First: find all eligible swaps, i.e. those that improve the situation
		for (a_off, &(a_id, ref a_name)) in names_sorted.iter().enumerate() {
			if a_off + 1 == names_sorted.len() {
				break;
			}
			dprint!("  swaps for {a_id}:{} is {:?}\n", a_name, cl.gates[a_id].unwrap());
			for &(b_id, ref b_name) in names_sorted[(a_off + 1)..].iter() {
				if a_id == b_id { continue }
				cl.swap(a_id, b_id);

				if let Some(errs_swapped) = cl.find_errors(errs_min) {
					if errs_swapped.1 < errs_min.1 && errs_swapped.0 <= errs_min.0 {
						dprint!("New swap pair {}<->{}: {errs_min:?} > {errs_swapped:?}\n", a_name, b_name);
						swaps_elig.push((errs_swapped, a_id, b_id));
					}
				}

				cl.swap(a_id, b_id);
			}
		}
		swaps_elig.sort();
		dprint!("eligible swaps: {} count: {swaps_elig:?}\n", swaps_elig.len());

		// Now do a repeated search for the best pair to swap.
		for i in 0..4 {
			let mut swaps_selecting = Vec::new();
			for &(_, a_id, b_id) in swaps_elig.iter() {
				let a_name = &self.id_to_name[&a_id];
				let b_name = &self.id_to_name[&b_id];
				//dprint!("  trying {a_id}:{} <-> {b_id}:{}", a_name, b_name);

				cl.swap(a_id, b_id);

				if let Some(errs_swapped) = cl.find_errors(errs_min) {
					//dprint!(" -> {errs_swapped:?}\n");
					if errs_swapped.1 < errs_min.1 && errs_swapped.0 <= errs_min.0 {
						dprint!("New swap pair {}<->{}: {errs_min:?} > {errs_swapped:?}\n", a_name, b_name);
						swaps_selecting.push((errs_swapped, a_id, b_id));
					}
				}

				cl.swap(a_id, b_id);
			}
			swaps_selecting.sort();
			dprint!("available swaps: {} count: {swaps_selecting:?}\n", swaps_selecting.len());
			let (errs_swapped, a_id, b_id) = swaps_selecting[0];
			let a_name = cl.id_to_name[&a_id].to_owned();
			let b_name = cl.id_to_name[&b_id].to_owned();
			dprint!("ADDING SWAP PAIR i:{i} {}<->{}: {errs_min:?} > {errs_swapped:?}\n", a_name, b_name);
			errs_min = errs_swapped;
			cl.swap(a_id, b_id);
			swapped.push(a_name);
			swapped.push(b_name);
		}
		dprint!("errs_min: {errs_min:?}, swapped: [{swapped:?}]\n");
		//assert_eq!(cl.find_errors((0, 0)), Some((0, 0)));
		swapped.sort();
		swapped.join(",")
	}
}
