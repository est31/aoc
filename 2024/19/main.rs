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

fn get_possile_hm<const EXIT_FIRST :bool>(s :&str, avail_hm :&HashMap<String, Vec<String>>, avail :&[String], at_start :bool) -> u64 {
	dprint!("    going with '{s}'\n");
	if s.is_empty() {
		dprint!("    -> TRUE\n");
		return 1;
	}
	let mut sum = 0;
	if at_start {
		let options = if s.len() == 1 {
			vec![s]
		} else {
			vec![&s[..2], &s[..1]]
		};
		let avail_hm_it = options.into_iter()
			.filter_map(|ndl| avail_hm.get(ndl))
			.map(|v| v.iter())
			.flatten();
		for av in avail_hm_it {
			let Some(s_stripped) = s.strip_prefix(av) else { continue };
			sum += get_possile_hm::<EXIT_FIRST>(s_stripped, avail_hm, avail, !at_start);
			if sum > 0 && EXIT_FIRST {
				return sum;
			}
		}
	} else {
		for av_end in avail {
			let Some(s_stripped) = s.strip_suffix(av_end) else { continue };
			sum += get_possile_hm::<EXIT_FIRST>(s_stripped, avail_hm, avail, !at_start);
			if sum > 0 && EXIT_FIRST {
				return sum;
			}
		}
	}
	sum
}

impl Towels {
	fn num_possible(&self) -> usize {
		self.desired.iter()
			.filter(|des| {
				dprint!(" {des}\n");
				let p = get_possile_hm::<true>(des, &self.avail_w_prefix, &self.avail, false);
				dprint!(" --> p: {p}\n");
				p > 0
			})
			.count()
	}
	fn sum_possible(&self) -> u64 {
		self.desired.iter()
			.map(|des| {
				dprint!(" {des}\n");
				let p = get_possile_hm::<false>(des, &self.avail_w_prefix, &self.avail, false);
				dprint!(" --> p: {p}\n");
				p
			})
			.sum()
	}
}
