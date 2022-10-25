use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (ts, vals) = parse(INPUT);
	let (id, wt) = min_bus_time(ts, &vals);
	println!("Product: {}", id * wt);
	let c = min_bus_cascade(&vals);
	println!("First cascade time: {}", c);
}

fn parse(input :&str) -> (u32, Vec<u32>) {
	let mut lines = input.lines();
	let l0 = lines.next().unwrap();
	let l1 = lines.next().unwrap();
	let v = u32::from_str(l0).unwrap();
	let w = l1.split(',')
		.map(|b| match b {
			"x" => 0,
			_ => u32::from_str(b).unwrap()
		})
		.collect::<Vec<_>>();
	(v, w)
}

fn min_bus_time(ts :u32, vals :&[u32]) -> (u32, u32) {
	let (v, wt) = vals.iter()
		.copied()
		.filter(|v| *v != 0)
		.map(|v| {
			let m = ts % v;
			let wt = if m == 0 {
				0
			} else {
				v - m
			};
			(v, wt)
		})
		.min_by_key(|(_v, wt)| {
			*wt
		})
		.unwrap();
	(v, wt)
}

fn min_bus_cascade(vals :&[u32]) -> u64 {
	let mut vals_dist = Vec::new();
	let mut dist = 0;
	for val in vals {
		if *val > 0 {
			let val = u64::from(*val);
			let offs = if dist % val == 0 {
				0
			} else {
				val.checked_sub(dist % val).unwrap()
			};
			vals_dist.push((val, offs));
		}
		dist += 1;
	}
	let mut mul = vals_dist[0].0;
	let mut offs = 0;
	for &(val, dist) in &vals_dist[1..] {
		//println!("val={val}, mul={mul}, offs={offs}");
		for m in 0.. {
			let v = (m * mul) + offs;
			if v % val == dist {
				//println!("    -> m={m}, v={v}, dist={dist}");
				offs = v;
				mul *= val;
				break;
			}
		}
	}
	//println!("-> {offs}");
	offs
}
