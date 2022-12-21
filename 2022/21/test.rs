use super::*;

const EXAMPLE_INPUT :&str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

#[test]
fn test_1() {
	let mnks = parse(EXAMPLE_INPUT);
	println!("{mnks:?}");
	let rn = root_number(&mnks);
	assert_eq!(rn, 152);
}
