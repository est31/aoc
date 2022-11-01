use std::collections::HashMap;
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let tiles = parse(INPUT);
	let product = edge_product(&tiles);
	println!("Product: {product}");
}

type Tile = (u16, Vec<Vec<bool>>);

fn parse(input :&str) -> Vec<Tile> {
	let lines = input.lines();
	let mut res = Vec::new();
	let mut tile_id = None;
	let mut tile_lines = Vec::new();
	for line in lines {
		if line.starts_with("Tile") {
			let id_str = line.split([' ', ':']).nth(1).unwrap();
			let id = u16::from_str(id_str).unwrap();
			tile_id = Some(id);
			continue;
		}
		if line.is_empty() {
			let tls = std::mem::take(&mut tile_lines);
			res.push((tile_id.take().unwrap(), tls));
			continue;
		}
		let tl = line.chars()
			.map(|c| match c {
				'#' => true,
				'.' => false,
				_ => panic!(),
			})
			.collect::<Vec<_>>();
		tile_lines.push(tl);
	}
	res.push((tile_id.take().unwrap(), tile_lines));
	res
}

fn edge_product(tiles :&[Tile]) -> u64 {
	let mut hm = HashMap::<_, Vec<u16>>::new();
	for tile in tiles {
		let l0 = tile.1[0].clone();
		let l1 = tile.1.last().unwrap().clone();
		let l2 = tile.1.iter().map(|l| l[0]).collect::<Vec<_>>();
		let l3 = tile.1.iter().map(|l| *l.last().unwrap()).collect::<Vec<_>>();
		let l_list = [l0, l1, l2, l3];
		let mut l_list_rev = l_list.clone();
		for l in l_list_rev.iter_mut() {
			l.reverse();
		}
		for l in l_list.into_iter().chain(l_list_rev.into_iter()) {
			hm.entry(l).or_default().push(tile.0);
		}
	}
	let mut v = HashMap::<_, usize>::new();
	for (_, ids) in hm.iter() {
		assert!(ids.len() < 3, "ids: {ids:?}");
		if let &[id] = &ids[..] {
			*v.entry(id).or_default() += 1;
		}
	}
	v.iter()
		.filter(|&(_id, num)| *num == 4)
		.map(|(id, _num)| *id as u64)
		.product()
}
