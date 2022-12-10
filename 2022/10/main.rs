use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let (sum, rendered) = render(&cmds);
	println!("signal strength sum: {sum}");
	println!("rendered:");
	println!("{rendered}");
}

#[derive(Debug)]
enum Cmd {
	Addx(i32),
	Nop,
}

fn parse(input :&str) -> Vec<Cmd> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			if l == "noop" {
				Cmd::Nop
			} else if l.starts_with("addx ") {
				let mut words = l.split_whitespace();
				let (Some(_), Some(v)) = (words.next(), words.next()) else {
					panic!("invalid addx line '{l}'");
				};
				let amnt = i32::from_str(v).unwrap();
				Cmd::Addx(amnt)
			} else {
				panic!("invalid line '{l}'");
			}
		})
		.collect::<Vec<_>>()
}

fn render(cmds :&[Cmd]) -> (u32, String) {
	let mut pc = 0;
	let mut x_val = 1;
	let mut sum = 0;
	const HEIGHT :usize = 6;
	const WIDTH :usize = 40;
	let mut fields = vec![vec![false; WIDTH]; HEIGHT];
	let mut drawn_x = 0;
	let mut drawn_y = 0;
	'outer: for cmd in cmds {
		let pc_inc = match cmd {
			Cmd::Addx(_amnt) => 2,
			Cmd::Nop => 1,
		};
		for _ in 0..pc_inc {
			pc += 1;
			if pc > 240 {
				break 'outer;
			}
			if [20, 60, 100, 140, 180, 220].contains(&pc) {
				let strength = pc * x_val;
				//println!("Strength at {pc} is: {pc}*{x_val}={strength}");
				sum += strength;
			}
			if [drawn_x - 1, drawn_x, drawn_x + 1].contains(&x_val) {
				fields[drawn_y as usize][drawn_x as usize] = true;
			}
			drawn_x += 1;
			if drawn_x as usize == WIDTH {
				assert_eq!((pc as usize) % WIDTH, 0);
				drawn_x = 0;
				drawn_y += 1;
			}
		}
		if let Cmd::Addx(amnt) = cmd {
			x_val += amnt;
		}
	}

	let sum = sum.try_into().unwrap();
	let rendered = fields.iter()
		.map(|l| {
			l.iter()
				.map(|b| if *b { "#" } else { "." })
				.chain(["\n"].into_iter())
				.collect::<String>()
		})
		.collect::<String>();
	(sum, rendered)
}
