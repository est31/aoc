use super::*;

const EXAMPLE_INPUT :&str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

#[test]
fn test_1() {
	let mnks = parse(EXAMPLE_INPUT);
	let ins_cnt = get_monkey_inspection_cnt(&mnks, false);
	assert_eq!(ins_cnt, [101, 95, 7, 105]);
	let bsns = get_monkey_business(&mnks, false);
	assert_eq!(bsns, 10605);
}

#[test]
fn test_2() {
	let mnks = parse(EXAMPLE_INPUT);
	let ins_cnt = get_monkey_inspection_cnt(&mnks, true);
	assert_eq!(ins_cnt, [52166, 47830, 1938, 52013]);
	let bsns = get_monkey_business(&mnks, true);
	assert_eq!(bsns, 2713310158);
}
