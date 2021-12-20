use super::*;

const EXAMPLE_INPUT :&str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

#[test]
fn test_parsing() {
	let (enhancement_algo, image) = parse_algo_input(EXAMPLE_INPUT);
	let s :String = image.pixels.iter()
		.map(|l| line_to_string(l) + "\n").collect();
	let algo_line = line_to_string(&enhancement_algo);
	assert_eq!(EXAMPLE_INPUT, format!("{}\n\n{}", algo_line, s))
}

#[test]
fn test() {
	let (enhancement_algo, mut image) = parse_algo_input(EXAMPLE_INPUT);
	assert_eq!(line_to_string(&image.pixels[0]), "#..#.");
	assert_eq!(image.count_pixels_lit(), 10);

	image.enhance(&enhancement_algo);
	assert_eq!(line_to_string(&image.pixels[0]), ".##.##.");
	assert_eq!(line_to_string(&image.pixels[1]), "#..#.#.");
	assert_eq!(line_to_string(&image.pixels[2]), "##.#..#");
	assert_eq!(line_to_string(&image.pixels[3]), "####..#");
	assert_eq!(line_to_string(&image.pixels[4]), ".#..##.");
	assert_eq!(line_to_string(&image.pixels[5]), "..##..#");
	assert_eq!(line_to_string(&image.pixels[6]), "...#.#.");
	assert_eq!(image.count_pixels_lit(), 24);

	image.enhance(&enhancement_algo);
	assert_eq!(line_to_string(&image.pixels[0]), ".......#.");
	assert_eq!(line_to_string(&image.pixels[1]), ".#..#.#..");
	assert_eq!(line_to_string(&image.pixels[2]), "#.#...###");
	assert_eq!(line_to_string(&image.pixels[3]), "#...##.#.");
	assert_eq!(line_to_string(&image.pixels[4]), "#.....#.#");
	assert_eq!(line_to_string(&image.pixels[5]), ".#.#####.");
	assert_eq!(line_to_string(&image.pixels[6]), "..#.#####");
	assert_eq!(line_to_string(&image.pixels[7]), "...##.##.");
	assert_eq!(line_to_string(&image.pixels[8]), "....###..");
	assert_eq!(image.count_pixels_lit(), 35);

	for _ in 0..(50-2) {
		image.enhance(&enhancement_algo);
	}
	assert_eq!(image.count_pixels_lit(), 3351);
}
