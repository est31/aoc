use super::*;

const EXAMPLE_INPUT :&str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[test]
fn test() {
	assert_eq!(count_1478("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), 2);
	assert_eq!(count_1478(EXAMPLE_INPUT), 26);
}

#[test]
fn test_full_deocde() {
	assert_eq!(find_number("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);

	let mut lines = EXAMPLE_INPUT.lines();
	assert_eq!(find_number(lines.next().unwrap()), 8394);
	assert_eq!(find_number(lines.next().unwrap()), 9781);
	assert_eq!(find_number(lines.next().unwrap()), 1197);
	assert_eq!(find_number(lines.next().unwrap()), 9361);
	assert_eq!(find_number(lines.next().unwrap()), 4873);
	assert_eq!(find_number(lines.next().unwrap()), 8418);
	assert_eq!(find_number(lines.next().unwrap()), 4548);
	assert_eq!(find_number(lines.next().unwrap()), 1625);
	assert_eq!(find_number(lines.next().unwrap()), 8717);
	assert_eq!(find_number(lines.next().unwrap()), 4315);
}
