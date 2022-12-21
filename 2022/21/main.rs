use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let mnks = parse(INPUT);
	let n = root_number(&mnks);
	println!("Root number: {n}");
}

#[derive(Debug, Copy, Clone)]
enum BinopKind {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone)]
enum Monkey {
	Num(i128),
	Binop(BinopKind, String, String),
}

fn parse(input :&str) -> HashMap<String, Monkey> {
	input.lines()
		.map(|l| l.trim())
		.map(|l| {
			let mut components = l.split(": ");
			let name = components.next().unwrap();
			let after = components.next().unwrap();
			let mnk = if let Ok(num) = i128::from_str(after) {
				Monkey::Num(num)
			} else {
				let mut words = after.split_whitespace();
				let first = words.next().unwrap();
				let op = words.next().unwrap();
				let second = words.next().unwrap();
				let op = match op {
					"+" => BinopKind::Add,
					"-" => BinopKind::Sub,
					"*" => BinopKind::Mul,
					"/" => BinopKind::Div,
					_ => panic!("invalid binop '{op}' in line '{l}'!"),
				};
				Monkey::Binop(op, first.to_string(), second.to_string())
			};
			(name.to_string(), mnk)
		})
		.collect::<HashMap<_, _>>()
}

fn find_for(mnks :&HashMap<String, Monkey>, name :&String, found :&mut HashMap<String, i128>) -> i128 {
	match &mnks[name] {
		Monkey::Num(n) => *n,
		Monkey::Binop(kind, a, b) => {
			if let Some(n) = found.get(name) {
				return *n;
			}
			let av = find_for(mnks, a, found);
			let bv = find_for(mnks, b, found);
			let res = match kind {
				BinopKind::Add => av + bv,
				BinopKind::Sub => av - bv,
				BinopKind::Mul => av * bv,
				BinopKind::Div => av / bv,
			};
			found.insert(name.clone(), res);
			res
		},
	}
}

fn root_number(mnks :&HashMap<String, Monkey>) -> i128 {
	find_for(mnks, &"root".to_string(), &mut HashMap::new())
}
