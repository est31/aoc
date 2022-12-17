const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let cmds = parse(INPUT);
	let h = tower_height(&cmds);
	println!("Tower height: {h}");
}

fn parse(input :&str) -> Vec<bool> {
	let line = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.next()
		.unwrap();
	line.chars()
		.map(|ch| {
			match ch {
				'<' => false,
				'>' => true,
				_ => panic!("Unexpected char '{ch}'"),
			}
		})
		.collect::<Vec<_>>()
}

const TOWERS :&[&[&[bool]]] = &[
	&[&[true, true, true, true]],
	&[
		&[false, true, false].as_slice(),
		&[true, true, true].as_slice(),
		&[false, true, false].as_slice(),
	],
	&[
		&[true, true, true].as_slice(),
		&[false, false, true].as_slice(),
		&[false, false, true].as_slice(),
	],
	&[
		&[true].as_slice(),
		&[true].as_slice(),
		&[true].as_slice(),
		&[true].as_slice(),
	],
	&[
		&[true, true].as_slice(),
		&[true, true].as_slice(),
	],
];

type Tower = &'static [&'static [bool]];

fn can_move_down(t :Tower, fields :&[[bool; 7]], tx :usize, ty :usize) -> bool {
	if ty == 0 {
		return false;
	}
	let ty = ty - 1;
	!does_collide(t, fields, tx, ty)
}

fn does_collide(t :Tower, fields :&[[bool; 7]], tx :usize, ty :usize) -> bool {
	if ty >= fields.len() - 1 {
		return false;
	}
	for (f_line, t_line) in fields[ty..].iter().zip(t.iter()) {
		for (f, t) in f_line[tx..].iter().zip(t_line.iter()) {
			if *f && *t {
				return true;
			}
		}
	}
	false
}

fn add_at(t :Tower, fields :&mut Vec<[bool; 7]>, tx :usize, ty :usize) {
	let t_height = t.len();
	for _ in fields.len()..(ty + t_height + 1) {
		fields.push([false; 7]);
	}
	for (f_line, t_line) in &mut fields[ty..].iter_mut().zip(t.iter()) {
		for (f, t) in f_line[tx..].iter_mut().zip(t_line.iter()) {
			*f |= *t;
		}
	}
}

fn tower_height(cmds :&[bool]) -> u32 {
	let mut fields = vec![[false; 7]];
	const LIMIT :usize = 2022;

	let mut cmds_iter = cmds.iter().cycle();

	for ctr in 0..LIMIT {
		let t = TOWERS[ctr % TOWERS.len()];

		let mut x = 2;
		let mut y = fields.len() + 2;

		loop {
			let move_right = *cmds_iter.next().unwrap();
			/*if ctr <= 10 {
				println!("Falling: x={x} y={y} right={move_right}");
			}*/
			let nx = if move_right {
				if x + t[0].len() < 7 {
					x + 1
				} else {
					x
				}
			} else {
				if x > 0 {
					x - 1
				} else {
					x
				}
			};
			if !does_collide(t, &fields, nx, y) {
				x = nx;
			}
			if can_move_down(t, &fields, x, y) {
				y -= 1;
			} else {
				break;
			}
		}

		add_at(t, &mut fields, x, y);
		/*if ctr >= 10 { continue; }
		println!("\nFields:");
		print_fields(&fields[fields.len().saturating_sub(50)..]);
		println!("------------------\n");*/
	}
	(fields.len() - 1) as _
}

/*
fn print_fields(fields :&[[bool; 7]]) {
	for line in fields.iter().rev() {
		let line_str = line
			.iter()
			.map(|b| if *b { '#' } else { '.' })
			.collect::<String>();
		println!("|{line_str}|");
	}
	println!("+-------+");
}
*/
