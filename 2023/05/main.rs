use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Range;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

type Int = u64;

fn main() {
	let almanac = parse(INPUT);
	println!("closest location: {}", almanac.min_location_from_seed());
	//println!("closest location (ranges, bruteforce): {}", almanac.min_location_from_seed_bruteforce());
	println!("closest location (ranges): {}", almanac.min_location_from_seed_ranges());
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Map {
	source_start :Int,
	dest_start :Int,
	length :Int,
}

impl Map {
	fn convert(&self, source :Int) -> Option<Int> {
		let src_st = self.source_start;
		if (src_st..(self.length + src_st)).contains(&source) {
			Some(self.dest_start + source - src_st)
		} else {
			None
		}
	}
	fn dest_end(&self) -> Int {
		self.dest_start + self.length
	}
}

fn lookup_in_maps(num :Int, maps :&[Map]) -> Option<Int> {
	maps.iter().filter_map(|m| m.convert(num)).next()
}

fn lookup_ranges_in_maps(ranges :&[Range<Int>], maps :&[Map]) -> Vec<Range<Int>> {
	#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
	enum Event<'a> {
		InEnd,
		InStart,
		MapEnd,
		MapStart(&'a Map),
	}
	let map_events = maps.iter()
		.map(|map| [
			(map.source_start, Event::MapStart(map)),
			(map.source_start + map.length, Event::MapEnd),
		].into_iter())
		.flatten();
	let ranges = ranges.iter()
		.filter(|r| !r.is_empty());
	let mut events = ranges
		.map(|range| [(range.start, Event::InStart), (range.end, Event::InEnd)].into_iter())
		.flatten()
		.chain(map_events)
		.collect::<Vec<(Int, Event<'_>)>>();
	events.sort();

	// Now do the sweep
	let mut res = Vec::new();
	let mut opened_input: Option<Int> = None;
	let mut opened_map: Option<(Int, &Map)> = None;
	//println!("events: {events:?}");
	for (v, ev) in events {
		//println!("  at v={v}, event={ev:?}");
		match ev {
			Event::InStart => {
				assert_eq!(opened_input, None, "overlapping input range at {v}");
				opened_input = Some(v);
			}
			Event::InEnd => {
				let start = opened_input.take().unwrap();
				if let Some((m_st, map)) = opened_map {
					let start = start.max(m_st);
					let start_conv = map.convert(start).unwrap();
					let end_conv = map.convert(v).unwrap_or(map.dest_end());
					res.push(start_conv..end_conv);
				} else {
					res.push(start..v);
				}
			}
			Event::MapStart(map) => {
				assert_eq!(opened_map, None, "overlapping input range at {v}");
				opened_map = Some((v, map));
				if let Some(i_st) = &mut opened_input {
					res.push(*i_st..v);
					*i_st = v;
				}
			}
			Event::MapEnd => {
				let (start, map) = opened_map.take().unwrap();
				if let Some(i_st) = &mut opened_input {
					let start = start.max(*i_st);
					let start_conv = map.convert(start).unwrap();
					let end_conv = map.dest_end();
					res.push(start_conv..end_conv);
					*i_st = v;
				}
			}
		}
	}
	res
}

#[derive(Debug)]
struct Almanac {
	seeds :Vec<Int>,
	map :HashMap<String, (Vec<Map>, String)>,
}

impl Almanac {
	fn lookup_seeds(&self) -> Vec<Int> {
		self.lookup_seeds_generic(self.seeds.clone())
	}
	fn lookup_seeds_generic(&self, seeds :Vec<Int>) -> Vec<Int> {
		let seed = "seed".to_string();
		let mut src_category = &seed;
		let mut src = seeds;

		//println!("starting out with: {src:?}");
		while src_category != "location" {
			//println!("  {src_category} ({})", src.len());
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
	fn seeds_ranges(&self) -> Vec<Range<Int>> {
		self.seeds.chunks(2)
			.map(|ch| {
				let st = ch[0];
				let len = ch[1];
				st..(st + len)
			})
			.collect::<Vec<_>>()
	}
	fn lookup_seeds_bruteforce(&self) -> Vec<Int> {
		let seeds = self.seeds_ranges().into_iter()
			.flatten()
			.collect::<Vec<_>>();
		self.lookup_seeds_generic(seeds)
	}
	fn lookup_seeds_ranges(&self) -> Vec<Range<Int>> {
		let seeds_ranges = self.seeds_ranges();

		let seed = "seed".to_string();
		let mut src_category = &seed;
		let mut src = seeds_ranges;

		//println!("starting out with: {src:?}");
		while src_category != "location" {
			let mapping = self.map.get(src_category).unwrap();
			src = lookup_ranges_in_maps(&src, &mapping.0);
			//println!("{src_category} to {} becomes: {src:?}", mapping.1);
			src_category = &mapping.1;
		}
		src
	}
	fn min_location_from_seed(&self) -> Int {
		*self.lookup_seeds()
			.iter()
			.min()
			.unwrap()
	}
	#[allow(unused)]
	fn min_location_from_seed_bruteforce(&self) -> Int {
		*self.lookup_seeds_bruteforce()
			.iter()
			.min()
			.unwrap()
	}
	fn min_location_from_seed_ranges(&self) -> Int {
		self.lookup_seeds_ranges()
			.iter()
			.map(|r| r.start)
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
		.map(|c| Int::from_str(c.trim())
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
				.map(|c| Int::from_str(c.trim())
					.map_err(|e| format!("'{c}' is not a valid int: {e}"))
					.unwrap());
			let map = Map {
				dest_start: ints_it.next().unwrap(),
				source_start: ints_it.next().unwrap(),
				length: ints_it.next().unwrap(),
			};
			assert!(map.length != 0, "map length is 0");
			maps.push(map);
		}
		map.insert(src, (maps, dest));
	}

	Almanac {
		seeds,
		map,
	}
}
