const EXAMPLE_INPUT :&str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

const EXAMPLE_OUTPUT :&str = r#"
Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
"#;

use super::*;

#[test]
fn test() -> Result<()> {

	let mut s = String::new();
	let (template, rules) = parse(EXAMPLE_INPUT)?;
	let mut p = template.to_string();

	s += &format!("Template:     {}\n", p);

	for st in 0..4 {
		p = rules.run_step(&p);
		s += &format!("After step {}: {}\n", st + 1, p);
	}

	println!("{}", s);
	assert_eq!(s.trim(), EXAMPLE_OUTPUT.trim());

	for st in 5..=10 {
		p = rules.run_step(&p);
		if st == 5 {
			assert_eq!(p.len(), 97);
		}
		if st == 10 {
			assert_eq!(p.len(), 3073);
			let occ = count_byte_occurences(&p.as_bytes());
			assert_eq!(occ['B' as usize], 1749);
			assert_eq!(occ['C' as usize], 298);
			assert_eq!(occ['H' as usize], 161);
			assert_eq!(occ['N' as usize], 865);
			let diff = compute_diff(&occ);
			assert_eq!(diff, 1588);
		}
	}

	Ok(())
}

#[test]
fn test_ext() -> Result<()> {
	let (template, rules) = parse(EXAMPLE_INPUT)?;
	let mut p = template.to_string();

	println!("Template:     {}\n", p);

	let mut p_ext = Polymer::from_str(&p);

	for st in 0..4 {
		p = rules.run_step(&p);
		p_ext = p_ext.run_step(&rules);
		println!("After step {}: {}\n", st + 1, p);
		assert_eq!(p_ext, Polymer::from_str(&p));
	}


	for st in 5..=40 {
		p_ext = p_ext.run_step(&rules);
		if st == 5 {
			assert_eq!(p_ext.len(), 97);
		}
		if st == 10 {
			assert_eq!(p_ext.len(), 3073);
			let occ = p_ext.count_byte_occurences();
			assert_eq!(occ['B' as usize], 1749);
			assert_eq!(occ['C' as usize], 298);
			assert_eq!(occ['H' as usize], 161);
			assert_eq!(occ['N' as usize], 865);
			let diff = compute_diff(&occ);
			assert_eq!(diff, 1588);
		}
		if st == 40 {
			let occ = p_ext.count_byte_occurences();
			assert_eq!(occ['B' as usize], 2192039569602);
			assert_eq!(occ['H' as usize], 3849876073);
		}
	}

	Ok(())
}
