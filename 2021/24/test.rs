use super::*;

const EXAMPLE_1 :&str = "\
inp x
mul x -1
";

#[test]
fn test_1() {
	let cmds = parse_commands(EXAMPLE_1);
	let expected = &[
		Command::Inp(Val::X),
		Command::Binop(Binop::Mul, Val::X, Val::Lit(-1)),
	];
	assert_eq!(cmds, expected);

	let mut alu = Alu::new();
	assert_eq!(alu.run_cmds_with_input(&cmds, &[-1]), 0);
	assert_eq!(alu.x, 1);

	let mut alu = Alu::new();
	assert_eq!(alu.run_cmds_with_input(&cmds, &[-9]), 0);
	assert_eq!(alu.x, 9);

	let mut alu = Alu::new();
	assert_eq!(alu.run_cmds_with_input(&cmds, &[3, 4]), 0);
	assert_eq!(alu.x, -3);
}

fn execute_abcs(start_z :Int, abcs :&[Abc], inputs :&[Int]) -> Int {
	fn execute_abc(start_z :Int, (a, b, c) :Abc, input :Int) -> Int {
		let x = (input != ((start_z % 26) + b)) as Int;
		let z = (start_z / a) * (x * 25 + 1) + x * (input + c);
		z
	}
	let mut z = start_z;
	for (abc, input) in abcs.iter().zip(inputs.iter()) {
		z = execute_abc(z, *abc, *input);
	}
	z
}

#[test]
fn test_model_on_input() {
	// Ensure that our model of how things work is accurate.
	let cmds = parse_commands(INPUT);
	let abcs = extract_abcs(&cmds);

	let input = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5];
	let alu_result = Alu::new().run_cmds_with_input(&cmds, input);
	assert_eq!(execute_abcs(0, &abcs, input), alu_result);

	let input = &[6, 5, 7, 6, 5, 6, 5, 5, 1, 5, 2, 5, 1, 7];
	let alu_result = Alu::new().run_cmds_with_input(&cmds, input);
	assert_eq!(execute_abcs(0, &abcs, input), alu_result);

	for u in 1..10 {
		for v in 1..10 {
			for w in 1..10 {
				let input = &[u, 9 - u, 1, 1, v, 2, w, 5, 7, 2, 2, 7, 5, 1];
				let alu_result = Alu::new().run_cmds_with_input(&cmds, input);
				//println!("{alu_result}");
				assert_eq!(execute_abcs(0, &abcs, input), alu_result,
					"mismatch for {input:?}");
			}
		}
	}
}

#[test]
fn test_fitting_for_abc() {
	// Ensure that fitting search is sound, i.e. gives correct results

	let abc = (26, -12, 6);
	let wanted_z = 0;

	let mut zs = Vec::new();
	find_fitting_for_abc(wanted_z, abc, |input, z| {
		println!("{input} {z}");
		zs.push((input, z));
	});
	assert!(zs.len() > 0);
	for (input, z) in zs {
		assert_eq!(execute_abcs(z, &[abc], &[input]), wanted_z,
			"mismatch for input {input}, abc {abc:?}");
	}
}

#[test]
fn test_search() {
	// Ensure that fitting search is sound, i.e. gives correct results
	let cmds = parse_commands(INPUT);
	let abcs = extract_abcs(&cmds);

	let mut searcher = Searcher::new();
	for depth in (0..14).rev() {

		let abc = abcs[depth];
		let abcs = &abcs[depth..];

		println!("depth {depth} abc is {abc:?}");

		searcher.step(abc);

		let found_count = searcher.zs_that_terminate.len();
		println!(" -> found {found_count}");
		assert!(found_count > 0);

		for (z, (inp_min, inp_max)) in searcher.zs_that_terminate.iter() {
			assert_eq!(execute_abcs(*z, abcs, &inp_min), 0,
				"mismatch for input {inp_min:?}, abcs {abcs:?}");
			assert_eq!(execute_abcs(*z, abcs, &inp_max), 0,
				"mismatch for input {inp_max:?}, abcs {abcs:?}");
		}
	}
}
