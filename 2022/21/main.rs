use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mnks = parse(INPUT);
	let n = root_number(&mnks);
	println!("Root number: {n}");
	let hn = human_number(&mnks);
	println!("Human number: {hn}");
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

fn find_for(mnks :&HashMap<String, Monkey>, name :&str,
		found :&mut HashMap<String, i128>, human_unknown :bool) -> Option<i128> {
	if human_unknown && name == "humn" {
		return None;
	}
	let res = match &mnks[name] {
		Monkey::Num(n) => *n,
		Monkey::Binop(kind, a, b) => {
			if let Some(n) = found.get(name) {
				return Some(*n);
			}
			let av = find_for(mnks, a, found, human_unknown)?;
			let bv = find_for(mnks, b, found, human_unknown)?;
			let res = match kind {
				BinopKind::Add => av + bv,
				BinopKind::Sub => av - bv,
				BinopKind::Mul => av * bv,
				BinopKind::Div => av / bv,
			};
			found.insert(name.to_string(), res);
			res
		},
	};
	Some(res)
}

fn root_number(mnks :&HashMap<String, Monkey>) -> i128 {
	find_for(mnks, "root", &mut HashMap::new(), false).unwrap()
}

fn human_number_for(mnks :&HashMap<String, Monkey>, name :&str,
		goal :i128, found :&mut HashMap<String, i128>) -> i128 {
	if name == "humn" {
		return goal;
	}
	let Monkey::Binop(kind, left, right) = &mnks[name] else {
		panic!("monkey {name} does not contain human");
	};
	let mut found = HashMap::new();
	let lval = find_for(mnks, &left, &mut found, true);
	let rval = find_for(mnks, &right, &mut found, true);
	let (with_human, other) = match (lval, rval) {
		(None, Some(other)) => (left, other),
		(Some(other), None) => (right, other),
		_ => panic!("invalid arms: {lval:?} {rval:?}"),
	};
	let new_goal = match kind {
		BinopKind::Add => goal - other,
		BinopKind::Sub => match (lval, rval) {
			// goal = lval - rval
			(None, Some(other)) => goal + other,
			(Some(other), None) => other - goal,
			_ => unreachable!(),
		},
		BinopKind::Mul => {
			if goal % other != 0 {
				panic!("{goal} is not divisible by {other}!");
			}
			goal / other
		},
		BinopKind::Div => match (lval, rval) {
			// goal = lval / rval
			(None, Some(other)) => goal * other,
			(Some(other), None) => {
				if other % goal != 0 {
					panic!("{other} is not divisible by {goal}!");
				}
				other / goal
			},
			_ => unreachable!(),
		},
	};
	human_number_for(mnks, with_human, new_goal, &mut found)
}

fn human_number(mnks :&HashMap<String, Monkey>) -> i128 {
	let Monkey::Binop(_kind, left, right) = &mnks["root"] else {
		panic!("invalid root monkey");
	};
	let mut found = HashMap::new();
	let lval = find_for(mnks, &left, &mut found, true);
	let rval = find_for(mnks, &right, &mut found, true);
	let (with_human, other) = match (lval, rval) {
		(None, Some(other)) => (left, other),
		(Some(other), None) => (right, other),
		_ => panic!("invalid arms: {lval:?} {rval:?}"),
	};
	human_number_for(mnks, with_human, other, &mut found)
}
