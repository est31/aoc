use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let mut cfg = ShipConfig::new();
	cfg.apply(INPUT);
	println!("Manhattan part 1: {}", cfg.manhattan());
	let mut cfg = ShipConfigAdv::new();
	cfg.apply(INPUT);
	println!("Manhattan part 2: {}", cfg.manhattan());
}

struct ShipConfig {
	dir :i32,
	x :i32,
	y :i32,
}

impl ShipConfig {
	fn new() -> Self {
		Self {
			dir : 90,
			x : 0,
			y : 0,
		}
	}
	fn normalize_dir(&mut self) {
		while self.dir < 0 {
			self.dir += 360;
		}
		self.dir = self.dir % 360;
	}
	fn go(&mut self, line :&str) {
		let line = line.trim();
		if line.is_empty() {
			return;
		}
		let mut chars = line.chars();
		let first = chars.next().unwrap();
		let num_str = chars.as_str();
		let num = i32::from_str(num_str).unwrap();
		match first {
			'N' => self.y += num,
			'S' => self.y -= num,
			'E' => self.x += num,
			'W' => self.x -= num,
			'L' => {
				self.dir -= num;
				self.normalize_dir();
			},
			'R' => {
				self.dir += num;
				self.normalize_dir();
			},
			'F' => match self.dir {
				0 => self.y += num,
				90 => self.x += num,
				180 => self.y -= num,
				270 => self.x -= num,
				_ => panic!("invalid dir {}", self.dir),
			},
			_ => panic!("invalid command '{first}'"),
		}
	}
	fn apply(&mut self, input :&str) {
		for l in input.lines() {
			self.go(l);
		}
	}
	fn manhattan(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}
}

struct ShipConfigAdv {
	wx :i32,
	wy :i32,
	x :i32,
	y :i32,
}

impl ShipConfigAdv {
	fn new() -> Self {
		Self {
			wx : 10,
			wy : 1,
			x : 0,
			y : 0,
		}
	}
	fn rotate_waypoint_90(&mut self) {
		let wx = self.wx;
		let wy = self.wy;

		self.wy = wx;
		self.wx = -wy;
		/*
		match (self.wx.signum(), self.wy.signum()) {
			(0..=1, 0..1) => {
				self.wy = wx;
				self.wx = -wy;
			},
			(-1, 0..=1) => {
				self.wy = wx;
				self.wx = wy;
			},
		}*/
	}
	fn go(&mut self, line :&str) {
		let line = line.trim();
		if line.is_empty() {
			return;
		}
		let mut chars = line.chars();
		let first = chars.next().unwrap();
		let num_str = chars.as_str();
		let num = i32::from_str(num_str).unwrap();
		match first {
			'N' => self.wy += num,
			'S' => self.wy -= num,
			'E' => self.wx += num,
			'W' => self.wx -= num,
			'L' | 'R' => {
				let dir = match first {
					'L' => num,
					'R' => 360 - num,
					_ => unreachable!(),
				};
				match dir {
					0 => (),
					90 => {
						self.rotate_waypoint_90();
					},
					180 => {
						self.rotate_waypoint_90();
						self.rotate_waypoint_90();
					},
					270 => {
						self.rotate_waypoint_90();
						self.rotate_waypoint_90();
						self.rotate_waypoint_90();
					},
					_ => panic!("invalid direction {dir}"),
				}
			},
			'F' => for _ in 0..num {
				self.x += self.wx;
				self.y += self.wy;
			},
			_ => panic!("invalid command '{first}'"),
		}
	}
	fn apply(&mut self, input :&str) {
		for l in input.lines() {
			self.go(l);
		}
	}
	fn manhattan(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}
}
