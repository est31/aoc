use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let coords = parse(INPUT);
	let a = surface_area(&coords);
	println!("Surface area: {a}");
}

fn parse(input :&str) -> Vec<[i16; 3]> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			let mut cs = l.split(',')
				.map(|s| i16::from_str(s).unwrap());
			std::array::from_fn(|_| cs.next().unwrap())
		})
		.collect::<Vec<_>>()
}

fn surface_area(coords :&[[i16; 3]]) -> u32 {
	let set = coords.iter().copied().collect::<HashSet<_>>();
	let mut area = 0;
	for &[px, py, pz] in coords {
		for x_offs in [-1, 0, 1] {
			let x = px + x_offs;

			for y_offs in [-1, 0, 1] {
				let y = py + y_offs;

				for z_offs in [-1, 0, 1] {
					let z = pz + z_offs;

					// If the sum is zero, we are querying the cell itself.
					// If the sum is > 1, there is no "connection" to the
					// cell any more.
					if x_offs.abs() + y_offs.abs() + z_offs.abs() != 1 {
						continue;
					}

					area += !set.contains(&[x, y, z]) as u32;
				}
			}
		}
	}
	area
}
