use std::str::FromStr;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let valves = parse(INPUT);
	let m = max_pressure_release(&valves);
	println!("Max pressure release: {m}");
}

fn parse(input :&str) -> HashMap<String, (u32, Vec<String>)> {
	input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.map(|l| {
			assert!(l.starts_with("Valve "));
			let mut words = l.splitn(3, " ");
			words.next().unwrap();
			let name = words.next().unwrap();

			let post = words.next().unwrap();
			let mut r = post.split(['=', ';']);
			r.next().unwrap();
			let rate = r.next().unwrap();
			let rate = u32::from_str(rate).unwrap();

			let post = r.next().unwrap();
			let mut list = if post.contains("valve ") {
				post.split("valve ")
			} else {
				post.split("valves ")
			};
			list.next().unwrap();
			let list = list.next().unwrap();
			let items = list.split(", ");
			let items = items
				.map(|item| item.to_owned())
				.collect::<Vec<_>>();

			(name.to_owned(), (rate, items))
		})
		.collect::<HashMap<_, _>>()
}

fn max_release_after(valves :&HashMap<String, (u32, Vec<String>)>, valve :&str, remaining :u16, memoized :&mut HashMap<(String, u16), u32>) -> u32 {
	if remaining == 0 {
		return 0;
	}

	if let Some(res) = memoized.get(&(valve.to_string(), remaining)) {
		return *res;
	}

	let vl = &valves[valve];

	let max_turned_off = vl.1.iter()
		.map(|nxt| max_release_after(valves, nxt, remaining - 1, memoized))
		.max()
		.unwrap_or(0);

	let flow_rate = vl.0;
	let res = if flow_rate > 0 {
		let prod = flow_rate * (remaining as u32 - 1);
		let max_turned_on = if remaining > 1 {
			vl.1.iter()
				.map(|nxt| max_release_after(valves, nxt, remaining - 2, memoized))
				.max()
				.unwrap_or(0)
		} else {
			0
		};
		(max_turned_on + prod).max(max_turned_on)
	} else {
		max_turned_off
	};
	memoized.insert((valve.to_owned(), remaining), res);
	res
}

fn max_pressure_release(valves :&HashMap<String, (u32, Vec<String>)>) -> u32 {
	let mut memoized = HashMap::new();
	max_release_after(valves, "AA", 30, &mut memoized)
}
