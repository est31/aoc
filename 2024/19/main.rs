const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let twl = parse(INPUT);
	println!("num_possible: {}", twl.num_possible());
}

#[derive(Clone, PartialEq, Eq)]
struct Towels {
	avail :Vec<String>,
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
	Towels {
		avail,
		desired,
	}
}

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

impl Towels {
	fn num_possible(&self) -> usize {
		self.desired.iter()
			.filter(|des| is_possile(des, &self.avail))
			.count()
	}
}
