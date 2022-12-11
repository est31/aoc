use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mnks = parse(INPUT);
	let bsns = get_monkey_business(&mnks, false);
	println!("Monkey business 20 rounds: {bsns}");
	let bsns_10k = get_monkey_business(&mnks, true);
	println!("Monkey business 10k rounds: {bsns_10k}");
}

#[derive(Debug, Copy, Clone)]
enum Operation {
	Square,
	OffsetConst(u64),
	MulConst(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
	starting :Vec<u64>,
	operation :Operation,
	test :(u64, usize, usize),
}

fn get_and_split<'a>(mut lines :impl Iterator<Item = &'a str>, sp :&str) -> &'a str {
	let st = lines.next().unwrap();
	st.split(sp).nth(1).unwrap()
}

fn parse(input :&str) -> Vec<Monkey> {
	let mut lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut res = Vec::new();
	while let Some(line) = lines.next() {
		if !line.starts_with("Monkey ") { continue; }

		let starting_list = get_and_split(&mut lines, ": ");
		let starting = starting_list.split(", ")
			.map(|s| u64::from_str(s).unwrap())
			.collect::<Vec<_>>();

		let op_str = get_and_split(&mut lines, "old ");
		let operation = if op_str == "* old" {
			Operation::Square
		} else if op_str.starts_with("+") {
			let offs_str = op_str.split("+ ").nth(1).unwrap();
			Operation::OffsetConst(u64::from_str(offs_str).unwrap())
		} else if op_str.starts_with("*") {
			let prod_str = op_str.split("* ").nth(1).unwrap();
			Operation::MulConst(u64::from_str(prod_str).unwrap())
		} else {
			panic!("invalid operation str 'old {op_str}'");
		};

		let div_str = get_and_split(&mut lines, "divisible by ");
		let divisor = u64::from_str(div_str).unwrap();

		let true_str = get_and_split(&mut lines, "to monkey ");
		let true_monkey = usize::from_str(true_str).unwrap();

		let false_str = get_and_split(&mut lines, "to monkey ");
		let false_monkey = usize::from_str(false_str).unwrap();

		let test = (divisor, true_monkey, false_monkey);

		res.push(Monkey {
			starting,
			operation,
			test,
		});
	}
	res
}

fn get_monkey_inspection_cnt(monkeys :&[Monkey], super_modulo :bool) -> Vec<u64> {
	let mut ins_cnt = vec![0; monkeys.len()];
	let mut monkeys = monkeys.to_vec();
	let rounds = if super_modulo {
		10_000
	} else {
		20
	};
	let super_modulo = super_modulo.then(|| {
		monkeys.iter()
			.map(|m| m.test.0)
			.product::<u64>()
	});
	//println!("super modulo: {super_modulo:?}");
	for _ri in 0..rounds {
		for i in 0..monkeys.len() {
			let operation = monkeys[i].operation;
			let test = monkeys[i].test;
			let items = std::mem::take(&mut monkeys[i].starting);
			ins_cnt[i] += items.len() as u64;
			for item in items {
				let item = match operation {
					Operation::Square => item * item,
					Operation::OffsetConst(offs) => item + offs,
					Operation::MulConst(offs) => item * offs,
				};
				let item = if let Some(super_modulo) = super_modulo {
					item % super_modulo
				} else {
					item / 3
				};
				if item % test.0 == 0 {
					monkeys[test.1].starting.push(item);
				} else {
					monkeys[test.2].starting.push(item);
				}
			}
		}
	}
	ins_cnt
}

fn get_monkey_business(monkeys :&[Monkey], super_modulo :bool) -> u64 {
	let ins_cnt = get_monkey_inspection_cnt(monkeys, super_modulo);
	let max = *ins_cnt.iter()
		.max()
		.unwrap();
	let max2 = *ins_cnt.iter()
		.filter(|v| **v != max)
		.max()
		.unwrap();
	max * max2
}
