use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mnks = parse(INPUT);
	let bsns = get_monkey_business(&mnks);
	println!("Monkey business: {bsns}");
}

#[derive(Debug, Copy, Clone)]
enum Operation {
	Square,
	OffsetConst(u32),
	MulConst(u32),
}

#[derive(Debug, Clone)]
struct Monkey {
	starting :Vec<u32>,
	operation :Operation,
	test :(u32, usize, usize),
}

fn parse(input :&str) -> Vec<Monkey> {
	let mut lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let mut res = Vec::new();
	while let Some(line) = lines.next() {
		if !line.starts_with("Monkey ") { continue; }

		let starting_str = lines.next().unwrap();
		let starting_list = starting_str.split(": ").nth(1).unwrap();
		let starting = starting_list.split(", ")
			.map(|s| u32::from_str(s).unwrap())
			.collect::<Vec<_>>();

		let op_line = lines.next().unwrap();
		let op_str = op_line.split("old ").nth(1).unwrap();
		let operation = if op_str == "* old" {
			Operation::Square
		} else if op_str.starts_with("+") {
			let offs_str = op_str.split("+ ").nth(1).unwrap();
			Operation::OffsetConst(u32::from_str(offs_str).unwrap())
		} else if op_str.starts_with("*") {
			let prod_str = op_str.split("* ").nth(1).unwrap();
			Operation::MulConst(u32::from_str(prod_str).unwrap())
		} else {
			panic!("invalid operation line '{op_line}'");
		};

		let mod_line = lines.next().unwrap();
		let div_str = mod_line.split("divisible by ").nth(1).unwrap();
		let divisor = u32::from_str(div_str).unwrap();
		let true_line = lines.next().unwrap();
		let true_str = true_line.split("to monkey ").nth(1).unwrap();
		let true_monkey = usize::from_str(true_str).unwrap();
		let false_line = lines.next().unwrap();
		let false_str = false_line.split("to monkey ").nth(1).unwrap();
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

fn get_monkey_inspection_cnt(monkeys :&[Monkey]) -> Vec<u32> {
	let mut ins_cnt = vec![0; monkeys.len()];
	let mut monkeys = monkeys.to_vec();
	for _ri in 0..20 {
		for i in 0..monkeys.len() {
			let operation = monkeys[i].operation;
			let test = monkeys[i].test;
			let items = std::mem::take(&mut monkeys[i].starting);
			ins_cnt[i] += items.len() as u32;
			for item in items {
				let item = match operation {
					Operation::Square => item * item,
					Operation::OffsetConst(offs) => item + offs,
					Operation::MulConst(offs) => item * offs,
				};
				let item = item / 3;
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

fn get_monkey_business(monkeys :&[Monkey]) -> u32 {
	let ins_cnt = get_monkey_inspection_cnt(monkeys);
	let max = *ins_cnt.iter()
		.max()
		.unwrap();
	let max2 = *ins_cnt.iter()
		.filter(|v| **v != max)
		.max()
		.unwrap();
	max * max2
}
