use super::*;

const EXAMPLE_INPUT_1 :&str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

#[test]
fn test_1() {
	let system = parse(EXAMPLE_INPUT_1);
	assert_eq!(system.path_count(), 10);
	assert_eq!(system.path_count_ext(), 36);
}

const EXAMPLE_INPUT_2 :&str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

#[test]
fn test_2() {
	let system = parse(EXAMPLE_INPUT_2);
	assert_eq!(system.path_count(), 19);
	assert_eq!(system.path_count_ext(), 103);
}

const EXAMPLE_INPUT_3 :&str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

#[test]
fn test_3() {
	let system = parse(EXAMPLE_INPUT_3);
	assert_eq!(system.path_count(), 226);
	assert_eq!(system.path_count_ext(), 3509);
}
