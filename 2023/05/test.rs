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
}
