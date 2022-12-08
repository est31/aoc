use std::collections::HashSet;
use std::iter::DoubleEndedIterator;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let grid = parse(INPUT);
	let ov = outside_visible(&grid);
	println!("outside visible: {}", ov);
	let scm = scenic_score_maximum(&grid);
	println!("max scenic score: {}", scm);
}

fn parse(input :&str) -> Vec<Vec<u8>> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			l.as_bytes()
				.iter()
				.map(|b| *b - b'0')
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn visibles_asc(it :impl Iterator<Item = u8> + Clone) -> Vec<(usize, u8)> {
	let mut max = None;
	let mut res = Vec::new();
	for (i, v) in it.enumerate() {
		let Some(m) = max else {
			max = Some(v);
			res.push((i, v));
			continue;
		};
		if v > m {
			max = Some(v);
			res.push((i, v));
		}
	}
	res
}

fn visibles(it :impl Iterator<Item = u8> + Clone + DoubleEndedIterator) -> Vec<(usize, u8)> {
	let (len, max_estim) = it.size_hint();
	assert_eq!(max_estim, Some(len));
	let mut res = visibles_asc(it.clone());
	let mut desc = visibles_asc(it.rev());
	desc.iter_mut()
		.for_each(|(i, _v)| {
			*i = len - 1 - *i;
		});
	res.extend_from_slice(&desc);
	res
}

fn outside_visible(grid :&[Vec<u8>]) -> usize {
	let visibles_horiz = grid.iter().enumerate()
		.map(|(j, l)| {
			visibles(l.iter().copied())
				.into_iter()
				.map(move |(i, v)| (i, j, v))
		})
		.flatten()
		.collect::<HashSet<_>>();
	let col_num = grid[0].len();
	let visibles_vert = (0..col_num)
		.map(|i| {
			let it = grid.iter().map(|l| l[i]);
			visibles(it)
				.into_iter()
				.map(move |(j, v)| (i, j, v))
		})
		.flatten()
		.collect::<HashSet<_>>();
	let visibles = visibles_vert.union(&visibles_horiz)
		.collect::<HashSet<_>>();
	//println!("{visibles:?}");
	visibles.len()
}

fn viewing_dist_asc(it :impl Iterator<Item = u8> + Clone) -> Vec<usize> {
	let mut max_st = Vec::new();
	let mut res = Vec::new();
	for (i, v) in it.enumerate() {
		while let Some((mi, mv)) = max_st.pop() {
			if mv >= v {
				max_st.push((mi, mv));
				break;
			}
		}
		let mut do_pop = false;
		let dist = if let Some((mi, mv)) = max_st.last() {
			if *mv == v {
				do_pop = true;
			}
			i - mi
		} else {
			i
		};
		if do_pop {
			max_st.pop();
		}
		max_st.push((i, v));
		res.push(dist);
	}
	res
}

fn viewing_distances(it :impl Iterator<Item = u8> + Clone + DoubleEndedIterator) -> Vec<(usize, usize)> {
	let (len, max_estim) = it.size_hint();
	assert_eq!(max_estim, Some(len));
	let asc = viewing_dist_asc(it.clone());
	let mut desc = viewing_dist_asc(it.rev());
	desc.reverse();
	asc.into_iter()
		.zip(desc.into_iter())
		.collect::<Vec<_>>()
}

fn scenic_score_maximum(grid :&[Vec<u8>]) -> u64 {
	let scores_horiz = grid.iter()
		.map(|l| viewing_distances(l.iter().copied()))
		.collect::<Vec<_>>();
	let col_num = grid[0].len();
	let scores_vert = (0..col_num)
		.map(|i| {
			let it = grid.iter().map(|l| l[i]);
			viewing_distances(it)
		})
		.collect::<Vec<_>>();
	let mut scores = Vec::new();
	for (j, lh) in scores_horiz.iter().enumerate() {
		for (i, lv) in scores_vert.iter().enumerate() {
			let (a, b) = lh[i];
			let (c, d) = lv[j];
			let score = a * b * c * d;
			//if score > 0 { println!("{i} {j}: {score}"); }
			scores.push(score);
		}
	}
	scores.into_iter()
		.max()
		.unwrap() as u64
}
