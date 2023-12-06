use super::*;

const EXAMPLE_INPUT :&str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn test() {
	let almanac = parse(EXAMPLE_INPUT);
	println!("{almanac:#?}");
	assert_eq!(almanac.lookup_seeds(), &[82, 43, 86, 35]);
	let closest_location = almanac.min_location_from_seed();
	assert_eq!(closest_location, 35);
	let closest_location = almanac.min_location_from_seed_bruteforce();
	assert_eq!(closest_location, 46);
	let mut locations = almanac.lookup_seeds_bruteforce();
	locations.sort();
	let locations_expected = &[46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
		58, 59, 60, 82, 83, 84, 86, 87, 88, 89, 94, 95, 96, 97, 98];
	assert_eq!(locations_expected, locations.as_slice());
	let mut locations_ranges = almanac.lookup_seeds_ranges()
		.into_iter()
		.flatten()
		.collect::<Vec<_>>();
	locations_ranges.sort();
	assert_eq!(locations_expected, locations_ranges.as_slice());
	let closest_location_ranges = almanac.min_location_from_seed_ranges();
	assert_eq!(closest_location_ranges, 46);
}
