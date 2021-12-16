use std::fmt::Display;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct StrErr(String);

impl<T :Display> From<T> for StrErr {
	fn from(v :T) -> Self {
		StrErr(format!("{}", v))
	}
}

type Result<T> = std::result::Result<T, StrErr>;

fn main() -> Result<()> {
	let p = parse_packet(INPUT)?;
	println!("Version sum: {}", p.version_sum());
	println!("Value: {}", p.compute_value());
	Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Packet {
	version :u16,
	id :u16,
	kind :PacketKind,
}

#[derive(PartialEq, Eq, Debug)]
enum PacketKind {
	Literal(u64),
	Operator(Vec<Packet>),
}

impl Packet {
	fn version_sum(&self) -> u64 {
		let sub_sum = if let PacketKind::Operator(ps) = &self.kind {
			ps.iter().map(|p| p.version_sum()).sum()
		} else {
			0
		};
		self.version as u64 + sub_sum
	}
	fn compute_value(&self) -> u64 {
		match &self.kind {
			PacketKind::Literal(v) => *v,
			PacketKind::Operator(p) => match self.id {
				0 => p.iter().map(Packet::compute_value).sum(),
				1 => p.iter().map(Packet::compute_value).product(),
				2 => p.iter().map(Packet::compute_value).min().unwrap(),
				3 => p.iter().map(Packet::compute_value).max().unwrap(),
				4 => unreachable!(),
				5 => (p[0].compute_value() > p[1].compute_value()) as u64,
				6 => (p[0].compute_value() < p[1].compute_value()) as u64,
				7 => (p[0].compute_value() == p[1].compute_value()) as u64,
				k => panic!("Invalid kind {}", k),
			},
		}
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		//print!($($args),*);
	};
}

struct BitReader<'a> {
	input :&'a [u8],
	bit_offs :usize,
}

impl<'a> BitReader<'a> {
	fn from_str(s :&'a str) -> Self {
		Self {
			input : s.as_bytes(),
			bit_offs : 0,
		}
	}
	fn read_bit(&mut self) -> Result<u16> {
		dprint!("  Reading bit no {}", self.bit_offs);
		let byte_offs = self.bit_offs / 4;
		let offs_in_byte = self.bit_offs % 4;

		let b = self.input[byte_offs];
		let v = match b {
			b'0'..=b'9' => b - b'0',
			b'A'..=b'F' => b - b'A' + 10,
			b'a'..=b'a' => b - b'a' + 10,
			c => Err(format!("Invalid hex '{}'", c as char))?,
		};
		dprint!(" b {:04b}", v);
		let r = (v >> (3 - offs_in_byte)) & 1;

		self.bit_offs += 1;

		dprint!(" => {}\n", r);
		Ok(r.into())
	}
	fn read<const B :usize>(&mut self) -> Result<u16> {
		dprint!("Reading {} bits...\n", B);
		let mut r = 0;
		for o in 0..B {
			r += self.read_bit()? << (B - 1 - o);
		}
		dprint!("Result of read: {:b}\n", r);
		Ok(r)
	}
	fn read_packet(&mut self) -> Result<Packet> {
		let version = self.read::<3>()?;
		let id = self.read::<3>()?;
		let kind;
		match id {
			4 => {
				let mut v = 0u64;

				while {
					v = v.checked_shl(4).unwrap();
					let r = self.read::<5>()?;
					v += r as u64 & 0b1111;
					r & (1 << 4) != 0
				} {}
				kind = PacketKind::Literal(v);
			},
			_ => {
				let mut packets = Vec::new();
				let b = self.read_bit()?;
				match b {
					0 => {
						let total_length = self.read::<15>()? as usize;
						let end_offs = self.bit_offs + total_length;
						while self.bit_offs < end_offs {
							packets.push(self.read_packet()?);
						}
					},
					1 => {
						let num_sub_packets = self.read::<11>()? as usize;
						while packets.len() < num_sub_packets {
							packets.push(self.read_packet()?);
						}
					},
					_ => unreachable!(),
				}
				kind = PacketKind::Operator(packets);
			},
		}
		Ok(Packet {
			version,
			id,
			kind,
		})
	}
}

fn parse_packet(input :&str) -> Result<Packet> {
	let mut rdr = BitReader::from_str(input);
	Ok(rdr.read_packet()?)
}
