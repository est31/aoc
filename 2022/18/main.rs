use std::str::FromStr;
use std::collections::HashSet;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let coords = parse(INPUT);
	let a = surface_area(&coords);
	println!("Surface area: {a}");
	let ao = outside_surface_area(&coords);
	println!("Reachable surface area: {ao}");
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

fn for_one_neighbours([px, py, pz] :[i16; 3], mut f :impl FnMut(i16, i16, i16, i16)) {
	for x_offs in [-1, 0, 1] {
		let x = px + x_offs;

		for y_offs in [-1, 0, 1] {
			let y = py + y_offs;

			for z_offs in [-1, 0, 1] {
				let z = pz + z_offs;

				let sum = x_offs.abs() + y_offs.abs() + z_offs.abs();
				f(x, y, z, sum);
			}
		}
	}
}

fn area_filtered(coords :&[[i16; 3]], f :impl Fn(i16, i16, i16) -> bool) -> u32 {
	let set = coords.iter().copied().collect::<HashSet<_>>();
	let mut area = 0;
	for &pos in coords {
		for_one_neighbours(pos, |x, y, z, l1_dist| {
			// If the dist is zero, we are querying the cell itself.
			// If the dist is > 1, there is no "connection" to the
			// cell any more.
			if l1_dist != 1 { return; }
			if !f(x, y, z) { return; }
			area += !set.contains(&[x, y, z]) as u32;
		});
	}
	area
}

fn surface_area(coords :&[[i16; 3]]) -> u32 {
	area_filtered(coords, |_, _, _| true)
}

fn min_max(it :impl Iterator<Item = i16> + Clone) -> (i16, i16) {
	let min = it.clone().min().unwrap();
	let max = it.max().unwrap();
	(min, max)
}

fn outside_surface_area(coords :&[[i16; 3]]) -> u32 {
	let set = coords.iter().copied().collect::<HashSet<_>>();

	let mut surface = HashSet::new();
	let mut surface_ext = HashSet::new();
	for &pos in coords {
		for_one_neighbours(pos, |x, y, z, l1_dist| {
			// If the dist is zero, we are querying the cell itself.
			if l1_dist == 0 { return; }
			if set.contains(&[x, y, z]) { return; }

			surface_ext.insert([x, y, z]);
			// If the dist is > 1, there is no "connection" to the
			// cell any more.
			if l1_dist == 1 {
				surface.insert([x, y, z]);
			}
		});
	}

	let (x_min, x_max) = min_max(surface.iter().map(|[px, _py, _pz]| *px));
	let (y_min, y_max) = min_max(surface.iter().map(|[_px, py, _pz]| *py));
	let (z_min, z_max) = min_max(surface.iter().map(|[_px, _py, pz]| *pz));

	let mut to_handle = HashSet::new();
	to_handle.insert([x_min, y_min, z_min]);
	let mut outside = HashSet::new();
	while let Some(pos) = to_handle.iter().copied().next() {
		to_handle.remove(&pos);
		outside.insert(pos);
		for_one_neighbours(pos, |x, y, z, l1_dist| {
			// If the dist is zero, we are querying the cell itself.
			// If the dist is > 1, there is no "connection" to the
			// cell any more.
			if l1_dist != 1 { return; }
			let p = [x, y, z];
			let in_range = (x_min..=x_max).contains(&x)
				&& (y_min..=y_max).contains(&y)
				&& (z_min..=z_max).contains(&z);
			if !outside.contains(&p) && !set.contains(&p) && in_range {
				to_handle.insert(p);
			}
		});
	}


	area_filtered(coords, |x, y, z| outside.contains(&[x, y, z]))
}
