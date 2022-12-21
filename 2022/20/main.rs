use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;


fn main() {
	let nums = parse(INPUT);
	let s = grove_coords_sum(&nums);
	println!("Grove coords sum: {s}");
}

fn parse(input :&str) -> Vec<i16> {
	input.lines()
		.map(|l| l.trim())
		.map(|l| i16::from_str(l).unwrap())
		.collect::<Vec<_>>()
}

fn map_nums<T :Copy>(nums :&[(T, usize, usize)], head :usize, f :&mut impl FnMut(usize, T)) {
	let mut i = head;
	loop {
		let n = nums[i].0;
		f(i, n);
		i = nums[i].2;
		if i == head {
			break;
		}
	}
}

fn mix(nums :&[i16]) -> Vec<i16> {
	mix_n(nums, nums.len())
}

fn mix_n(nums :&[i16], len :usize) -> Vec<i16> {
	let mut nums = nums.iter()
		.enumerate()
		.map(|(i, n)| {
			let prev_idx = i.checked_sub(1)
				.unwrap_or(nums.len() - 1);
			(*n, prev_idx, (i + 1) % nums.len())
		})
		.collect::<Vec<_>>();
	let zero_idx = nums.iter()
		.enumerate()
		.find(|(_i, v)| v.0 == 0)
		.unwrap().0;
	/*let print = |nums :&[(_, _, _)], head :usize| {
		map_nums(nums, head, &mut |i, n| print!("{n} ({} {}), ", nums[i].1, nums[i].2));
		println!();
	};
	println!("------------------------\nInitial state:");
	print(&nums, zero_idx);
	println!();*/
	for i in 0..len {
		let ni = nums[i];

		if ni.0 == 0 {
			continue;
		}

		// First step: remove
		nums[ni.1].2 = ni.2;
		nums[ni.2].1 = ni.1;

		let move_amount = ni.0.abs() + (ni.0 < 0) as i16;

		let mut cur = i;
		//println!("\nMove {ni:?} by {move_amount}:");
		for _ in 0..move_amount {
			cur = if ni.0 < 0 {
				nums[cur].1
			} else {
				nums[cur].2
			};
		}
		let next_i = nums[cur].2;
		//println!("cur={cur}, ni={next_i}");
		nums[cur].2 = i;
		nums[next_i].1 = i;
		nums[i].1 = cur;
		nums[i].2 = next_i;
		//print(&nums, zero_idx);
	}
	let mut res = Vec::new();
	map_nums(&nums, zero_idx, &mut |_i, n| res.push(n));
	res
}

fn grove_coords_sum(nums :&[i16]) -> i16 {
	let mixed = mix(nums);
	let sum = [1000, 2000, 3000].iter()
		.map(|p| mixed[p % mixed.len()])
		//.map(|p| { println!("{p}"); p })
		.sum();
	sum
}
