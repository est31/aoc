use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sum = eval_lines(INPUT);
	println!("Sum: {sum}");
	let sum = eval_lines_2(INPUT);
	println!("Sum v2: {sum}");
}

fn eval_lines(input :&str) -> u64 {
	input.lines()
		.map(|l| {
			let tokens = tokenize(l);
			//println!("{tokens:?}");
			let mut pos = 0;
			eval_tokens(&mut pos, &tokens)
		})
		.sum()
}

#[derive(Clone, Copy, Debug)]
enum Token<'a> {
	OpenBr,
	CloseBr,
	Plus,
	Mul,
	Lit(u64),
	Unparsed(&'a str),
}

fn tokenize(l :&str) -> Vec<Token<>> {
	let comps = l.split_whitespace()
		.collect::<Vec<_>>();
	let comps = comps.into_iter()
		.map(|comp| {
			let comps = comp.split('(')
				.map(|s| [Token::Unparsed(s)])
				.collect::<Vec<_>>();
			comps.join(&Token::OpenBr).into_iter()
		})
		.flatten()
		.map(|tok| {
			if let Token::Unparsed(comp) = tok {
				let comps = comp.split(')')
					.map(|s| [Token::Unparsed(s)])
					.collect::<Vec<_>>();
				comps.join(&Token::CloseBr).into_iter()
			} else {
				vec![tok].into_iter()
			}
		})
		.flatten()
		.filter_map(|tok| {
			let ret = if let Token::Unparsed(comp) = tok {
				match comp {
					"" => return None,
					"+" => Token::Plus,
					"*" => Token::Mul,
					_ => Token::Lit(u64::from_str(comp).unwrap()),
				}
			} else {
				tok
			};
			Some(ret)
		})
		.collect::<Vec<_>>();
	comps
}

fn eval_tokens(pos :&mut usize, toks :&[Token<'_>]) -> u64 {
	//println!("  {pos}");
	let mut last_op_is_mul = None;

	let mut acc = None;
	while let Some(tok) = toks.get(*pos) {
		*pos += 1;
		//println!("    {pos}");
		let v = match tok {
			Token::Mul => {
				last_op_is_mul = Some(true);
				continue;
			},
			Token::Plus => {
				last_op_is_mul = Some(false);
				continue;
			},
			Token::OpenBr => eval_tokens(pos, toks),
			Token::CloseBr => break,
			Token::Lit(v) => *v,
			Token::Unparsed(s) => panic!("Unexpected unparsed token '{s}'"),
		};
		if let Some(acc_v) = acc.take() {
			let op = if last_op_is_mul.take().unwrap() {
				|a, b| a * b
			} else {
				|a, b| a + b
			};
			acc = Some(op(acc_v, v));
		} else {
			acc = Some(v);
		}
	}
	acc.unwrap()
}

fn eval_tokens_2(pos :&mut usize, toks :&[Token<'_>], stop_at_mul :bool, dont_eat_close :bool) -> u64 {
	//println!("  {pos}");

	let mut acc = None;
	while let Some(tok) = toks.get(*pos) {
		*pos += 1;
		//println!("    {pos} {acc:?}");
		let v = match tok {
			Token::Mul => {
				if stop_at_mul {
					*pos -= 1;
					break;
				}
				let w = eval_tokens_2(pos, toks, false, true);
				acc = Some(acc.unwrap() * w);
				continue;
			},
			Token::Plus => {
				let w = eval_tokens_2(pos, toks, true, true);
				acc = Some(acc.unwrap() + w);
				continue;
			},
			Token::OpenBr => eval_tokens_2(pos, toks, false, false),
			Token::CloseBr => {
				if dont_eat_close {
					*pos -= 1;
				}
				break;
			},
			Token::Lit(v) => *v,
			Token::Unparsed(s) => panic!("Unexpected unparsed token '{s}'"),
		};
		assert_eq!(acc, None);
		acc = Some(v);
	}
	//println!("  -> {}", acc.unwrap());
	acc.unwrap()
}

fn eval_lines_2(input :&str) -> u64 {
	input.lines()
		.map(|l| {
			let tokens = tokenize(l);
			//println!("{tokens:?}");
			let mut pos = 0;
			eval_tokens_2(&mut pos, &tokens, false, false)
		})
		.sum()
}
