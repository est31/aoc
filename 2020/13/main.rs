use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (ts, vals) = parse(INPUT);
	let (id, wt) = min_bus_time(ts, &vals);
	println!("Product: {}", id * wt);
}

fn parse(input :&str) -> (u32, Vec<u32>) {
	let mut lines = input.lines();
	let l0 = lines.next().unwrap();
	let l1 = lines.next().unwrap();
	let v = u32::from_str(l0).unwrap();
	let w = l1.split(',')
		.filter(|b| *b != "x")
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
