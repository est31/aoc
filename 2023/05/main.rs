use std::str::FromStr;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let almanac = parse(INPUT);
	println!("closest location: {}", almanac.min_location_from_seed());
}

#[derive(Copy, Clone, Debug)]
struct Map {
	source_start :u64,
	dest_start :u64,
	length :u64,
}

impl Map {
	fn convert(&self, source :u64) -> Option<u64> {
		let src_st = self.source_start;
		if (src_st..(self.length + src_st)).contains(&source) {
			Some(self.dest_start + source - src_st)
		} else {
			None
		}
	}
}

fn lookup_in_maps(num :u64, maps :&[Map]) -> Option<u64> {
	maps.iter().filter_map(|m| m.convert(num)).next()
}

#[derive(Debug)]
struct Almanac {
	seeds :Vec<u64>,
	map :HashMap<String, (Vec<Map>, String)>,
}

impl Almanac {
	fn lookup_seeds(&self) -> Vec<u64> {
		let seed = "seed".to_string();
		let mut src_category = &seed;
		let mut src = self.seeds.clone();

		//println!("starting out with: {src:?}");
		while src_category != "location" {
			let mapping = self.map.get(src_category).unwrap();
			src = src.into_iter()
				.map(|src_num| {
					lookup_in_maps(src_num, &mapping.0)
						// default to identity if not covered by source ranges
						.unwrap_or(src_num)
				})
				.collect::<Vec<_>>();
			//println!("{src_category} to {} becomes: {src:?}", mapping.1);
			src_category = &mapping.1;
		}
		src
	}
	fn min_location_from_seed(&self) -> u64 {
		*self.lookup_seeds()
			.iter()
			.min()
			.unwrap()
	}
}

fn parse(input :&str) -> Almanac {
	let mut lines = input.lines()
		.map(|l| l.trim());
	let seeds = lines.next().unwrap()
		.split(':')
		.nth(1)
		.unwrap()
		.split_whitespace()
		.map(|c| u64::from_str(c.trim())
			.map_err(|e| format!("'{c}' is not a valid int: {e}"))
			.unwrap())
		.collect::<Vec<_>>();
	assert_eq!(lines.next().unwrap(), "");
	let mut map = HashMap::new();
	while let Some(l) = lines.next() {
		let mapping_name = l.split(' ').next().unwrap();
		let mut cs = mapping_name.split("-to-");
		let src = cs.next().unwrap().to_owned();
		let dest = cs.next().unwrap().to_owned();
		let mut maps = Vec::new();
		while let Some(l) = lines.next() {
			if l.is_empty() {
				break;
			}
			let mut ints_it = l.split_whitespace()
				.map(|c| u64::from_str(c.trim())
					.map_err(|e| format!("'{c}' is not a valid int: {e}"))
					.unwrap());
			let map = Map {
				dest_start: ints_it.next().unwrap(),
				source_start: ints_it.next().unwrap(),
				length: ints_it.next().unwrap(),
			};
			maps.push(map);
		}
		map.insert(src, (maps, dest));
	}

	Almanac {
		seeds,
		map,
	}
}
