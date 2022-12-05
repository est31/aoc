use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (stacks, cmds) = parse(INPUT);
	let top = exec_and_top(&stacks, &cmds);
	println!("top stacks: {top}");
}

fn parse(input :&str) -> (Vec<Vec<char>>, Vec<(u16, u16, u16)>) {
	let mut lines = input.lines();
	let mut stacks = Vec::<Vec<_>>::new();
	while let Some(line) = lines.next() {
		if line.is_empty() {
			continue;
		}
		if !line.contains('[') {
			// Numbers line
			break;
		}
		let mut si = 0;
		let mut spaces_since_last = 0;
		for ch in line.trim_end().chars() {
			match ch {
				' ' => spaces_since_last += 1,
				'[' | ']' => (),
				l if l.is_ascii_uppercase() => {
					if ![0, 1].contains(&(spaces_since_last % 4)) {
						panic!("invalid space count {spaces_since_last}");
					}
					si += spaces_since_last / 4;
					//println!("ch='{ch}', sp={spaces_since_last} si={si}");
					spaces_since_last = 0;
					while si >= stacks.len() {
						stacks.push(Vec::new());
					}
					stacks[si].push(ch);
					si += 1;
				},
				_ => panic!("invalid char '{ch}'"),
			}
		}
	}

	for st in stacks.iter_mut() {
		st.reverse();
	}

	assert_eq!(lines.next().unwrap(), "");

	let cmds = lines
		.map(|l| {
			let words = l.split_whitespace()
				.collect::<Vec<_>>();
			if let &["move", n, "from", from, "to", to] = &words[..] {
				let n = u16::from_str(n).unwrap();
				let from = u16::from_str(from).unwrap();
				let to = u16::from_str(to).unwrap();
				(n, from, to)
			} else {
				panic!("invalid line '{l}' with words {words:?}");
			}
		})
		.collect::<Vec<_>>();
	(stacks, cmds)
}

fn exec_and_top(stacks :&[Vec<char>], cmds: &[(u16, u16, u16)]) -> String {
	let mut stacks = stacks.to_vec();
	for &(n, from, to) in cmds {
		for _ in 0..n {
			let ch = stacks[from as usize - 1].pop().unwrap();
			stacks[to as usize - 1].push(ch);
		}
	}
	stacks.iter()
		.map(|st| st.last().unwrap())
		.collect::<_>()
}
