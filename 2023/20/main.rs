use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pl = parse(INPUT);
	println!("low high product: {}", low_high_product(&pl));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
	FlipFlop,
	Conjunction,
}

#[derive(Debug, Clone)]
struct Plan {
	starts :Vec<u16>,
	#[allow(dead_code)]
	interner :HashMap<String, u16>,
	#[allow(dead_code)]
	interned_names :Vec<String>,
	module_plan :HashMap<u16, (Kind, Vec<u16>)>,
}

fn parse(input :&str) -> Plan {
	let lines = input.lines();
	let mut interner = HashMap::new();
	let mut interned_names = Vec::new();
	let mut module_plan = HashMap::new();
	let mut starts = Vec::new();
	for line in lines {
		let mut cmps = line.split(" -> ");
		let (name, dests) = (cmps.next().unwrap(), cmps.next().unwrap());
		let dests_ints = dests.split(", ")
			.map(|name| {
				let len = interner.len() as u16;
				let v = *interner.entry(name.to_owned())
					.or_insert_with(|| {
						interned_names.push(name.to_owned());
						len
					});
				v
			})
			.collect::<Vec<u16>>();
		let kind = if name.starts_with("%") {
			Kind::FlipFlop
		} else if name.starts_with("&") {
			Kind::Conjunction
		} else if name == "broadcaster" {
			starts = dests_ints;
			continue;
		} else {
			panic!("invalid line: '{line}'");
		};
		let name = &name[1..];
		let len = interner.len() as u16;
		let v = *interner.entry(name.to_owned())
			.or_insert_with(|| {
				interned_names.push(name.to_owned());
				len
			});
		module_plan.insert(v, (kind, dests_ints));
	}
	Plan {
		starts,
		interner,
		interned_names,
		module_plan,
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum StateKind {
	// whether the flip flop is on or off
	FlipFlop(bool),
	// remembered input states
	Conjunction(HashMap<u16, bool>),
}

#[derive(Debug, Clone)]
struct States {
	pl :Plan,
	states :HashMap<u16, (StateKind, Vec<u16>)>,
}

impl States {
	fn push_button(&mut self) -> (u16, u16) {
		let mut pulses = VecDeque::new();
		for st in &self.pl.starts {
			pulses.push_back((false, *st, None));
		}
		let mut count_low = 0;
		let mut count_high = 0;
		while let Some((is_high, id, orig_id)) = pulses.pop_front() {
			println!("pulse {is_high} from {:?} to {}:{id}", orig_id.map(|id|&self.pl.interned_names[id as usize]), &self.pl.interned_names[id as usize]);
			if is_high {
				count_high += 1;
			} else {
				count_low += 1;
			}
			let Some((ref mut kind, outputs)) = self.states.get_mut(&id) else {
				// This module is not listed, some kind of output state.
				println!("  -> module not listed");
				continue
			};
			let output_high = match kind {
				StateKind::FlipFlop(mut b) => {
					if !is_high {
						b = !b;
					}
					println!("  -> flip-flop: {b}");
					b
				},
				StateKind::Conjunction(hm) => {
					/*if let Some(orig_id) = orig_id {
						*hm.get_mut(&orig_id).unwrap() = is_high;
					}*/
					println!("  -> conjunction: {hm:?}");
					!hm.iter()
						.all(|(_, is_high)| *is_high)
				},
			};
			println!("  -> sending output={output_high} to {} modules", outputs.len());
			for op_id in outputs.clone().iter() {
				// update conjunction state
				if let Some((ref mut kind, _outputs)) = self.states.get_mut(&op_id) {
					if let StateKind::Conjunction(ref mut hm) = kind {
						hm.get_mut(&id).unwrap();
					}
				}
				pulses.push_back((output_high, *op_id, Some(id)));
			}
		}
		(count_low + 1, count_high)
	}
}

fn low_high_product(pl :&Plan) -> u64 {
	let (sl, sh) = low_high(pl);
	sl * sh
}

fn low_high(pl :&Plan) -> (u64, u64) {
	let mut states = HashMap::new();
	for (id, md) in pl.module_plan.iter() {
		let kind = match md.0 {
			Kind::FlipFlop => StateKind::FlipFlop(false),
			Kind::Conjunction => StateKind::Conjunction(HashMap::new()),
		};
		states.insert(*id, (kind, md.1.clone()));
	}
	for (id, md) in pl.module_plan.iter() {
		for out_id in md.1.iter() {
			let Some(out_state) = states.get_mut(out_id) else {

				continue
			};
			match out_state.0 {
				StateKind::FlipFlop(_) => (),
				StateKind::Conjunction(ref mut hm) => {
					hm.insert(*id, false);
				},
			}
		}
	}
	let mut states = States {
		pl : pl.clone(),
		states,
	};
	println!("{states:?}");
	let mut sum_low = 0;
	let mut sum_high = 0;
	//const COUNT :usize = 1000;
	const COUNT :usize = 3;
	for _ in 0..COUNT {
		println!("\n###### BUTTON PUSH");
		let (cl, ch) = states.push_button();
		sum_low += cl as u64;
		sum_high += ch as u64;
		println!("low={cl} high={ch}");
	}

	(sum_low, sum_high)
}
