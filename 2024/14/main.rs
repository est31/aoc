use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pvs = parse(INPUT);
	println!("safety factor after 100 seconds: {}", safety_factor_100(&pvs));
	print_loop(&pvs);
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct PosVel {
	pos :(i32, i32),
	vel :(i32, i32),
}

fn parse(s :&str) -> Vec<PosVel> {
	fn parse_tuple(s :&str) -> (i32, i32) {
		let mut cs = s.split(['=', ',']);
		cs.next().unwrap();
		let x = cs.next().unwrap();
		let y = cs.next().unwrap();
		let x = i32::from_str(x).unwrap();
		let y = i32::from_str(y).unwrap();
		(x, y)
	}
	s.trim()
		.lines()
		.map(|l|{
			let mut comps = l.split(' ');
			let pos = parse_tuple(comps.next().unwrap());
			let vel = parse_tuple(comps.next().unwrap());
			PosVel { pos, vel }
		})
		.collect::<Vec<_>>()
}

#[derive(Clone)]
struct Scene {
	pvs :Vec<PosVel>,
	width :usize,
	height :usize,
}

impl Scene {
	fn quadrant_counts(&self) -> [u32; 4] {
		let mut q_cnts = [0u32; 4];
		let m_x = self.width as i32 / 2;
		let m_y = self.height as i32 / 2;

		for pv in self.pvs.iter() {
			use core::cmp::Ordering::*;
			let p = pv.pos;
			let qi = match (p.0.cmp(&m_x), p.1.cmp(&m_y)) {
				// Not in any quadrant
				(_, Equal) | (Equal, _) => continue,
				(Less, Less) => 0,
				(Less, Greater) => 1,
				(Greater, Less) => 2,
				(Greater, Greater) => 3,
			};
			q_cnts[qi] += 1;
		}
		q_cnts
	}
	fn safety_factor_wh(&self) -> u32 {
		self.quadrant_counts().into_iter().product()
	}

	fn one_sec(&mut self) {
		for pv in self.pvs.iter_mut() {
			pv.pos.0 += pv.vel.0;
			pv.pos.1 += pv.vel.1;
			pv.pos.0 = pv.pos.0.rem_euclid(self.width as i32);
			pv.pos.1 = pv.pos.1.rem_euclid(self.height as i32);
		}
	}
	fn pos_hm(&self) -> HashMap<(i32, i32), u32> {
		self.pvs.iter()
			.map(|pv| {
				(pv.pos.0, pv.pos.1)
			})
			.fold(HashMap::<_, u32>::new(), |mut hm, p| {
				*hm.entry(p).or_default() += 1;
				hm
			})
	}
	fn print(&self) {
		let hm = self.pos_hm();
		for y in 0..(self.height as i32) {
			for x in 0..(self.width as i32) {
				if let Some(v) = hm.get(&(x, y)) {
					if *v < 10 {
						print!("{v}");
					} else {
						print!("X");
					}
				} else {
					print!(".");
				}
			}
			println!();
		}
	}
	fn secs_until_cycle(&self) -> usize {
		// https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
		let mut tortoise = self.clone();
		let mut hare = self.clone();
		loop {
			tortoise.one_sec();
			hare.one_sec();
			hare.one_sec();
			if tortoise.pvs == hare.pvs {
				break;
			}
		}

		let mut mu = 0;
		tortoise = self.clone();
		while tortoise.pvs != hare.pvs {
			tortoise.one_sec();
			hare.one_sec();
			mu += 1;
		}

		let mut lam = 1;
		hare = tortoise.clone();
		hare.one_sec();
		while tortoise.pvs != hare.pvs {
			hare.one_sec();
			lam += 1;
		}
		mu + lam
	}
}

fn safety_factor_100_wh(pvs :&[PosVel], width :usize, height :usize) -> u32 {
	let mut scene = Scene { pvs: pvs.to_vec(), width, height };
	let secs = 100;
	for _ in 0..secs {
		scene.one_sec();
	}
	scene.safety_factor_wh()
}

fn safety_factor_100(pvs :&[PosVel]) -> u32 {
	safety_factor_100_wh(pvs, 101, 103)
}

fn print_loop_wh(pvs :&[PosVel], width :usize, height :usize) {
	let mut scene = Scene { pvs: pvs.to_vec(), width, height };
	let secs = scene.secs_until_cycle();
	for sec in 0..secs {
		scene.one_sec();
		let qcs = scene.quadrant_counts();
		// Usually the christmas tree is not in the center, use that
		let left_qs = qcs[0] + qcs[1];
		let right_qs = qcs[2] + qcs[3];
		if (left_qs * 2 < right_qs) || (left_qs > right_qs * 2) {
			println!("\nAfter sec {}", sec + 1);
			scene.print();
		}
	}
}

fn print_loop(pvs :&[PosVel]) {
	print_loop_wh(pvs, 101, 103)
}
