use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let twl = parse(INPUT);
	println!("num possible: {}", twl.num_possible());
	println!("sum possible: {}", twl.sum_possible());
}

#[derive(Clone, PartialEq, Eq)]
struct Towels {
	avail :Vec<String>,
	avail_w_prefix :HashMap<String, Vec<String>>,
	desired :Vec<String>,
}

fn parse(s :&str) -> Towels {
	let mut lines = s.trim()
		.lines()
		.map(str::trim);
	let avail = lines.next().unwrap()
		.split(", ")
		.map(|s| s.to_string())
		.collect::<Vec<_>>();
	assert!(lines.next().unwrap().is_empty());
	let desired = lines
		.map(|s| s.to_string())
		.collect::<Vec<String>>();

	let mut avail_w_prefix = HashMap::<_, Vec<_>>::new();
	for av in avail.iter() {
		let av = av.as_str();
		if av.len() == 1 {
			avail_w_prefix.entry(av.to_string()).or_default().push(av.to_string());
		} else {
			avail_w_prefix.entry(av[..2].to_string()).or_default().push(av.to_string());
		}
	}

	Towels {
		avail,
		avail_w_prefix,
		desired,
	}
}

macro_rules! dprint {
	($($args:expr),*) => {
		if false
			{ print!($($args),*); }
	};
}

#[allow(unused)]
fn is_possile(s :&str, avail :&[String]) -> bool {
	if s.is_empty() {
		return true;
	}
	for av in avail.iter() {
		if let Some(s_stripped) = s.strip_prefix(av) {
			if is_possile(s_stripped, avail) {
				return true;
			}
		}
	}
	false
}

fn count_ending_with<'a>(s :&'a str, avail :&[String], ending_with_count :&mut HashMap<&'a str, u64>) -> u64 {
	if let Some(e) = ending_with_count.get(s) {
		return *e;
	}
	//dprint!("    going with '{s}'\n");
	if s.is_empty() {
		//dprint!("    -> TRUE\n");
		return 1;
	}
	let mut sum = 0;
	for av_end in avail {
		let Some(s_stripped) = s.strip_prefix(av_end) else { continue };
		sum += count_ending_with(s_stripped, avail, ending_with_count);
	}
	ending_with_count.insert(s, sum);
	sum
}

impl Towels {
	fn num_possible(&self) -> usize {
		self.desired.iter()
			.filter(|des| {
				dprint!(" {des}\n");
				let p = count_ending_with(des, &self.avail, &mut HashMap::new());
				dprint!(" --> p: {p}\n");
				p > 0
			})
			.count()
	}
	fn sum_possible(&self) -> u64 {
		self.desired.iter()
			.map(|des| {
				dprint!(" {des}\n");
				let p = count_ending_with(des, &self.avail, &mut HashMap::new());
				dprint!(" --> p: {p}\n");
				p
			})
			.sum()
	}
}
