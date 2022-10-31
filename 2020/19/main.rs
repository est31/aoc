use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let rules_msgs = parse_rules_msgs(INPUT);
	let valid = find_valid_count(&rules_msgs.0, &rules_msgs.1);
	println!("Valid msgs: {valid}");
}

enum Rule {
	Or(Vec<u16>, Vec<u16>),
	And(Vec<u16>),
	Terminal(u8),
}

fn parse_num_list(input :&str) -> Vec<u16> {
	input.split_whitespace()
		.filter(|c| !c.is_empty())
		.map(|c| u16::from_str(c).unwrap())
		.collect::<Vec<_>>()
}

fn parse_rules_msgs(input :&str) -> (HashMap<u16, Rule>, Vec<String>) {
	let mut lines = input.lines();

	let mut rules = HashMap::new();

	while let Some(line) = lines.next() {
		if line.is_empty() {
			break;
		}

		let mut cmps = line.split(':');
		let rule_num = u16::from_str(cmps.next().unwrap()).unwrap();
		let rule_str = cmps.next().unwrap();
		let rule = if rule_str.contains("\"") {
			let mut cmps = rule_str.split("\"");
			_ = cmps.next();
			let terminal = cmps.next().unwrap();
			assert_eq!(terminal.len(), 1);
			Rule::Terminal(terminal.as_bytes()[0])
		} else if rule_str.contains("|") {
			let mut cmps = rule_str.split("|");
			let list_1 = parse_num_list(cmps.next().unwrap());
			let list_2 = parse_num_list(cmps.next().unwrap());
			Rule::Or(list_1, list_2)
		} else {
			let list = parse_num_list(rule_str);
			Rule::And(list)
		};
		rules.insert(rule_num, rule);
	}

	let msgs = lines
		.map(|l| l.to_string())
		.collect::<Vec<_>>();

	(rules, msgs)
}

fn find_valid_and_offsets(indices :&[u16], rules :&HashMap<u16, Rule>, msg :&[u8]) -> HashSet<usize> {
	let mut offsets = HashSet::new();
	offsets.insert(0);
	for ridx in indices.iter() {
		let mut new_offsets = HashSet::new();
		for off in std::mem::take(&mut offsets) {
			let adds = find_valid_offsets(*ridx, rules, &msg[off..]).into_iter()
				.map(|o| off + o);
			new_offsets.extend(adds);
		}
		offsets = new_offsets;
	}
	/*println!("  AND offsets: {offsets:?} for: '{}' and: {indices:?}",
		std::str::from_utf8(msg).unwrap());*/
	offsets
}

fn find_valid_offsets(rule_idx :u16, rules :&HashMap<u16, Rule>, msg :&[u8]) -> HashSet<usize> {
	let rule = rules.get(&rule_idx).unwrap();
	match rule {
		Rule::Or(either, or) => {
			let mut either = find_valid_and_offsets(either, rules, msg);
			let or = find_valid_and_offsets(or, rules, msg);
			either.extend(or.into_iter());
			either
		},
		Rule::And(list) => {
			find_valid_and_offsets(list, rules, msg)
		},
		Rule::Terminal(ch) => {
			if msg[0] == *ch {
				[1].into_iter().collect()
			} else {
				HashSet::new()
			}
		},
	}
}

fn is_valid(rules :&HashMap<u16, Rule>, msg :&str) -> bool {
	let msg = msg.as_bytes();
	let offsts = find_valid_offsets(0, rules, msg)
		.into_iter()
		.collect::<Vec<_>>();
	let expected = &[msg.len()];
	let ret = &offsts == expected;
	//println!("{offsts:?} vs {expected:?} => {ret}");
	ret
}

fn find_valid_count(rules :&HashMap<u16, Rule>, msgs :&[String]) -> usize {
	msgs.iter()
		.filter(|m| is_valid(rules, m))
		.count()
}
