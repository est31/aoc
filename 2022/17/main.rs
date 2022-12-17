use std::collections::hash_map::{HashMap, Entry};

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

const LIMIT_1 :u128 = 2022;
const LIMIT_2 :u128 = 1000000000000;

fn main() {
	let cmds = parse(INPUT);
	let h = tower_height(&cmds, LIMIT_1);
	println!("Tower height: {h}");
	let huge_h = tower_height(&cmds, LIMIT_2);
	println!("Huge tower height: {huge_h}");
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

fn tower_height(cmds :&[bool], limit :u128) -> u128 {
	let mut fields = vec![[false; 7]];

	let mut cmds_ctr = 0;
	let mut cmds_iter = cmds.iter().cycle();

	let mut last_height = 0;
	let mut seen = HashMap::<_, (u128, usize)>::new();
	let mut time_travelled = false;

	let mut ctr = 0;
	let mut height_offs = 0;

	while ctr < limit {
		let ti = ctr % TOWERS.len() as u128;
		if ti == 0 && time_travelled == false {
			let height = fields.len() - 1;
			let top_profile = core::array::from_fn::<_, 7, _>(|i| {
				fields.iter()
					.rev()
					.enumerate()
					.find(|(_, line)| line[i])
					.map(|(i, _)| i)
					.unwrap_or(0)
			});
			//println!("tower {ti} cmd {} diff {} top profile {top_profile:?}", cmds_ctr % cmds.len(), height - last_height);
			let entry = seen.entry((ti, cmds_ctr % cmds.len(), height - last_height, top_profile));
			match entry {
				Entry::Occupied(o) => {
					let ctr_diff = ctr - o.get().0;
					let height_diff = height - o.get().1;
					let applications = (limit - ctr) / ctr_diff;
					height_offs = (applications as u128) * (height_diff as u128);
					ctr += applications * ctr_diff;
					time_travelled = true;
					/*println!("Found cycle with ctr diff {ctr_diff} and height diff {height_diff}. \
						Apply it {applications} times.");*/
				},
				Entry::Vacant(v) => {
					v.insert((ctr, height));
				},
			}
			last_height = height;
		}
		let t = TOWERS[ti as usize];

		let mut x = 2;
		let mut y = fields.len() + 2;

		loop {
			let move_right = *cmds_iter.next().unwrap();
			cmds_ctr += 1;
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
		ctr += 1;
	}
	fields.len() as u128 - 1 + height_offs
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
