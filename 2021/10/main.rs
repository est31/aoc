const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	println!("Syntax score: {}", compute_syntax_score(INPUT));
	println!("Middle score: {}", compute_middle_score(INPUT));
}

fn compute_syntax_score(input :&str) -> u32 {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let (illegal, _exp) = find_illegal_and_expected(l);
			match illegal {
				Some(')') => 3,
				Some(']') => 57,
				Some('}') => 1197,
				Some('>') => 25137,
				Some(c) => panic!("invalid illegal char '{}'", c),
				None => 0,
			}
		})
		.sum()
}

fn compute_middle_score(input :&str) -> u64 {
	let mut scores = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.filter_map(|l| {
			let (illegal, exp) = find_illegal_and_expected(l);
			if illegal.is_some() {
				return None;
			}
			let mut acc = 0;
			for ch in exp.iter().rev() {
				acc *= 5;
				let inc = match ch {
					')' => 1,
					']' => 2,
					'}' => 3,
					'>' => 4,
					_ => panic!("invalid expected char '{}'", ch),
				};
				acc += inc;
			}
			if acc == 0 {
				return None;
			}
			Some(acc)
		})
		.collect::<Vec<_>>();
	scores.sort();
	scores[scores.len() / 2]
}

fn find_illegal_and_expected(line :&str) -> (Option<char>, Vec<char>) {
	let mut expected_stack = Vec::new();
	let mut illegal = None;
	for ch in line.chars() {
		match ch {
			'(' => expected_stack.push(')'),
			'{' => expected_stack.push('}'),
			'[' => expected_stack.push(']'),
			'<' => expected_stack.push('>'),
			')' | ']' | '}' | '>' => {
				let exp = expected_stack.get(expected_stack.len().wrapping_sub(1));
				if exp != Some(&ch) {
					illegal = Some(ch);
					break;
				}
				expected_stack.pop();
			},
			c => panic!("invalid char '{}'", c),
		}
	}
	(illegal, expected_stack)
}
