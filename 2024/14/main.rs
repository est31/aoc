use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let pvs = parse(INPUT);
	println!("safety factor after 100 seconds: {}", safety_factor_100(&pvs));
}

#[derive(Copy, Clone)]
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

struct Scene {
	pvs :Vec<PosVel>,
	width :usize,
	height :usize,
}

impl Scene {
	fn safety_factor_wh(&self) -> u32 {
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
		q_cnts.into_iter().product()
	}

	fn one_sec(&mut self) {
		for pv in self.pvs.iter_mut() {
			pv.pos.0 += pv.vel.0;
			pv.pos.1 += pv.vel.1;
			pv.pos.0 = pv.pos.0.rem_euclid(self.width as i32);
			pv.pos.1 = pv.pos.1.rem_euclid(self.height as i32);
		}
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
