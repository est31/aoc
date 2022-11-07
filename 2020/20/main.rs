use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let tiles = parse(INPUT);
	let product = corner_product(&tiles);
	println!("Product: {product}");
	let reconstructed = reconstruct_image(&tiles);
	let tr_nt_mn = count_true_not_sea_monster(&reconstructed);
	println!("Not sea monster: {tr_nt_mn}");
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

fn corner_product(tiles :&[Tile]) -> u64 {
	let corners = hm_corners(tiles).1;
	corners.into_iter()
		.map(|c| c as u64)
		.product()
}

fn get_l_list(tile :&[Vec<bool>]) -> [Vec<bool>; 4] {
	let l0 = tile[0].clone();
	let l1 = tile.last().unwrap().clone();
	let l2 = tile.iter().map(|l| l[0]).collect::<Vec<_>>();
	let l3 = tile.iter().map(|l| *l.last().unwrap()).collect::<Vec<_>>();
	[l0, l1, l2, l3]
}

fn hm_corners(tiles :&[Tile]) -> (HashMap<Vec<bool>, Vec<u16>>, Vec<u16>) {
	let mut hm = HashMap::<_, Vec<u16>>::new();
	for tile in tiles {
		let l_list = get_l_list(&tile.1);
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
	let corners = v.iter()
		.filter(|&(_id, num)| *num == 4)
		.map(|(id, _num)| *id)
		.collect::<Vec<_>>();
	(hm, corners)
}

fn mirror_y(tile :&mut [Vec<bool>]) {
	tile.reverse();
}

fn mirror_x(tile :&mut [Vec<bool>]) {
	for line in tile.iter_mut() {
		line.reverse();
	}
}

fn rotate_90(tile :&mut [Vec<bool>]) {
	let mut rotated = Vec::new();
	for y in 0 .. tile.len() {
		let line = tile.iter()
			.rev()
			.map(|tl| tl[y])
			.collect::<Vec<_>>();
		rotated.push(line);
	}
	for (line, rt) in tile.iter_mut().zip(rotated.into_iter()) {
		*line = rt;
	}
}

fn is_top_left_like(tile :&[Vec<bool>], tl :&[bool], is_top :bool) -> bool {
	if is_top {
		assert_eq!(tile[0].len(), tl.len());
		&tile[0] == &tl
	} else {
		assert_eq!(tile.len(), tl.len());
		tile.iter()
			.zip(tl.iter())
			.all(|(line, tl)| line[0] == *tl)
	}
}

fn rotate_until(tile :&[Vec<bool>], check :impl Fn(&[Vec<bool>]) -> bool) -> Vec<Vec<bool>> {
	let mut tile = tile.to_vec();
	for _ in 0..4 {
		if check(&tile) {
			return tile;
		}

		mirror_y(&mut tile);
		if check(&tile) {
			return tile;
		}
		mirror_y(&mut tile);

		mirror_x(&mut tile);
		if check(&tile) {
			return tile;
		}
		mirror_x(&mut tile);

		rotate_90(&mut tile);
	}
	panic!("No rotation/mirroring found!");
}

fn rotate_until_top_left_like(tile :&[Vec<bool>], tl :&[bool], is_top :bool) -> Vec<Vec<bool>> {
	rotate_until(tile, |tile| is_top_left_like(tile, tl, is_top))
}

fn strip_borders(tile :&mut Vec<Vec<bool>>) {
	tile.remove(0);
	tile.remove(tile.len() - 1);
	for line in tile.iter_mut() {
		line.remove(0);
		line.remove(line.len() - 1);
	}
}

fn reconstruct_image(tiles :&[Tile]) -> Vec<Vec<bool>> {
	let (hm, corners) = hm_corners(tiles);
	let smallest_corner_id = corners.iter().min().unwrap();

	//println!("Adding tile {smallest_corner_id} as start corner tile");

	let top_left_tile = tiles.iter()
		.find(|(id, _)| id == smallest_corner_id)
		.map(|(_, tile)| tile)
		.unwrap();
	let l_list = get_l_list(&top_left_tile);
	let free_sides = l_list.iter()
		.filter(|l| hm.get(*l).unwrap().len() == 1)
		.collect::<Vec<_>>();
	assert_eq!(free_sides.len(), 2);
	let top = free_sides[0];
	let left = free_sides[1];
	let mut top_rev = top.clone();
	let mut left_rev = left.clone();
	top_rev.reverse();
	left_rev.reverse();

	let mut top_left_tile = rotate_until(&top_left_tile, |tile| {
		let tm = is_top_left_like(tile, &top, true) || is_top_left_like(tile, &top_rev, true);
		let lm = is_top_left_like(tile, &left, false) || is_top_left_like(tile, &left_rev, false);
		//println!("  {tm} {lm}");
		tm && lm
	});

	let mut l1 = top_left_tile.last().unwrap().clone();
	let mut l3 = top_left_tile.iter().map(|l| *l.last().unwrap()).collect::<Vec<_>>();

	strip_borders(&mut top_left_tile);

	let mut res = top_left_tile;

	let mut added = HashSet::new();
	added.insert(smallest_corner_id);

	let mut is_top = false;

	loop {
		let next_l = if is_top {
			&l1
		} else {
			&l3
		};

		let tile_id = hm.get(next_l)
			.unwrap()
			.iter()
			.filter(|id| !added.contains(id))
			.collect::<Vec<_>>();
		assert_eq!(tile_id.len(), 1);

		let tile_id = tile_id[0];
		added.insert(tile_id);
		//println!("Adding tile {tile_id} with is_top={is_top}");

		let tile = tiles.iter()
			.find(|(id, _)| id == tile_id)
			.map(|(_, tile)| tile)
			.unwrap();

		// Now rotate so that next_l matches
		let mut tile = rotate_until_top_left_like(tile, next_l, is_top);

		let new_l1 = tile.last().unwrap().clone();
		let new_l3 = tile.iter().map(|l| *l.last().unwrap()).collect::<Vec<_>>();

		strip_borders(&mut tile);

		// Add tile to res
		if is_top {
			res.extend_from_slice(&tile);
		} else {
			let start = res.len() - tile.len();
			for (line, tl) in res[start..].iter_mut().zip(tile.iter_mut()) {
				line.extend_from_slice(&tl);
			}
		}

		if is_top {
			l1 = new_l1.clone();
		}

		is_top = hm.get(&new_l3).unwrap().len() == 1;

		if is_top {
			if hm.get(&new_l1).unwrap().len() == 1 {
				// We are done!
				assert_eq!(added.len(), tiles.len());
				break;
			}
		} else {
			l3 = new_l3;
		}
	}

	assert_eq!(res.len() * res[0].len(), tiles.len() * 8 * 8);
	res
}

const SEA_MONSTER :&str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #
";

fn sea_monster_positions() -> Vec<(usize, usize)> {
	SEA_MONSTER.lines()
		.skip(1)
		.enumerate()
		.map(|(y, l)| {
			l.chars()
				.enumerate()
				.filter(|(_x, ch)| *ch == '#')
				.map(move |(x, _ch)| (x, y))
		})
		.flatten()
		.collect::<Vec<_>>()
}

fn disappear_monsters(tiles :&mut [Vec<bool>]) {
	let mps = sea_monster_positions();
	for yo in 0..tiles.len() {
		for xo in 0..tiles[0].len() {
			let matches = mps.iter()
				.all(|(x, y)| {
					let Some(line) = tiles.get(y + yo) else { return false };
					let Some(tile) = line.get(x + xo) else { return false };
					*tile
				});
			if matches {
				// Found one! :)
				//println!("Found a monster :)");
				for (x, y) in mps.iter() {
					tiles[y + yo][x + xo] = false;
				}
			}
		}
	}
}

fn count_true_not_sea_monster(tiles :&[Vec<bool>]) -> u32 {
	let mut tiles = tiles.to_vec();

	for _ in 0..4 {
		disappear_monsters(&mut tiles);

		mirror_y(&mut tiles);
		disappear_monsters(&mut tiles);
		mirror_y(&mut tiles);

		mirror_x(&mut tiles);
		disappear_monsters(&mut tiles);
		mirror_x(&mut tiles);

		rotate_90(&mut tiles);
	}

	let true_count = tiles.iter()
		.map(|l| l.iter())
		.flatten()
		.filter(|b| **b)
		.count();
	true_count as u32
}
