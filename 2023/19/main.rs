use std::str::FromStr;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (in_, workflows, parts) = parse(INPUT);
	println!("rating sum: {}", rating_sum(in_, &workflows, &parts));
}

fn parse(input :&str) -> (usize, Vec<Workflow>, Vec<Part>) {
	let mut lines = input.lines();
	let mut workflows = Vec::new();
	while let Some(line) = lines.next() {
		if line.is_empty() {
			break;
		}
		let mut parts = line.split(['{', '}']);
		let name = parts.next().unwrap().to_string();
		let rules_str = parts.next().unwrap();
		let rules = rules_str.split(',')
			.map(|rule_str| Rule::parse(rule_str))
			.collect::<Vec<_>>();
		workflows.push(Workflow { name, rules, });
	}
	let mut hm = HashMap::new();
	for (idx, workflow) in workflows.iter().enumerate() {
		hm.insert(workflow.name.to_owned(), idx);
	}
	let in_ = hm.get("in").unwrap();
	for wf in workflows.iter_mut() {
		for rule in wf.rules.iter_mut() {
			if let Rule::Workflow { name: (name_str, name_idx) } |
					Rule::Check { name: (name_str, name_idx), .. } = rule {
				if let Some(idx) = hm.get(name_str) {
					*name_idx = Some(*idx);
				}
			}
		}
	}
	let parts = lines.map(|l| Part::parse(l))
		.collect::<Vec<_>>();
	(*in_, workflows, parts)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
	Check { category :char, lower_check :bool, limit :u32, name :(String, Option<usize>), },
	Workflow { name :(String, Option<usize>) },
	Accept,
	Reject,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
	name :String,
	rules :Vec<Rule>,
}

impl Rule {
	fn parse(rule :&str) -> Self {
		if rule == "A" {
			return Rule::Accept;
		}
		if rule == "R" {
			return Rule::Reject;
		}
		if !rule.contains(':') {
			return Rule::Workflow { name :(rule.to_string(), None) };
		}
		let lower_check = rule.contains('<');
		let mut parts = rule.split(['<', '>', ':']);
		let category_str = parts.next().unwrap();
		let category = category_str.chars().next().unwrap();
		let limit_str = parts.next().unwrap();
		let limit = u32::from_str(limit_str).unwrap();
		let name = parts.next().unwrap().to_owned();
		Rule::Check { category, lower_check, limit, name: (name, None), }
	}

}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Part {
	x :u32,
	m :u32,
	a :u32,
	s :u32,
}

impl Part {
	fn parse(part :&str) -> Self {
		let mut parts = part.split(['{', '}', '=', ','])
			.filter(|p| !p.is_empty());
		assert_eq!(parts.next().unwrap(), "x");
		let x = u32::from_str(parts.next().unwrap()).unwrap();
		assert_eq!(parts.next().unwrap(), "m");
		let m = u32::from_str(parts.next().unwrap()).unwrap();
		assert_eq!(parts.next().unwrap(), "a");
		let a = u32::from_str(parts.next().unwrap()).unwrap();
		assert_eq!(parts.next().unwrap(), "s");
		let s = u32::from_str(parts.next().unwrap()).unwrap();
		Part { x, m, a, s, }
	}
	fn rating(&self) -> u32 {
		self.x + self.m + self.a + self.s
	}
}

fn rating(start_workflow :usize, workflows :&[Workflow], part :&Part) -> u32 {
	let mut cur = &workflows[start_workflow];
	loop {
		//println!("cur is {cur:?}");
		for rule in cur.rules.iter() {
			match rule {
				Rule::Accept => return part.rating(),
				Rule::Reject => return 0,
				Rule::Workflow { name } => {
					cur = &workflows[name.1.unwrap()];
					break;
				},
				Rule::Check { category, lower_check, limit, name } => {
					let rating = match category {
						'x' => part.x,
						'm' => part.m,
						'a' => part.a,
						's' => part.s,
						_ => panic!("invalid category '{category}'"),
					};
					let do_jump = if *lower_check {
						rating < *limit
					} else {
						rating > *limit
					};
					if do_jump {
						if name.0 == "A" {
							return part.rating();
						}
						if name.0 == "R" {
							return 0;
						}
						//println!("  doing the jump to {name:?}");
						cur = &workflows[name.1.unwrap()];
						break;
					}
				},
			}
		}
	}
}

fn rating_sum(in_ :usize, workflows :&[Workflow], parts :&[Part]) -> u32 {
	parts.iter()
		.map(|p| rating(in_, workflows, p))
		.sum::<u32>()
}
