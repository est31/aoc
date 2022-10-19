use super::*;

const EXAMPLE_INPUT :&str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

#[test]
fn test_1() {
	let mut instructions = parse(EXAMPLE_INPUT);
	let v = exec_until_repetition(&instructions);
	assert_eq!(v, 5);
	let w = find_instruction_to_flip(&mut instructions);
	assert_eq!(w, 8);
}
