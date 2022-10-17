use std::str::FromStr;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let valid = count_valid(INPUT, false);
	println!("valid passports: {valid}");

	let valid = count_valid(INPUT, true);
	println!("really valid passports: {valid}");
}

fn count_valid(input :&str, extra_validation :bool) -> usize {
	let kvpairs = input.lines()
		.map(|l| {
			let l = l.trim();
			if l.is_empty() {
				Box::new([""].into_iter()) as Box<dyn Iterator<Item = &str>>
			} else {
				Box::new(l.trim().split_whitespace()) as Box<dyn Iterator<Item = &str>>
			}
		})
		.flatten();
	let mut fields = 0;
	let mut valid_passports = 0;
	for pair in kvpairs {
		//println!("  {pair}");
		if pair.is_empty() {
			//println!("fields: {fields}");
			if fields == 0b1111111 {
				valid_passports += 1;
			}
			fields = 0;
			continue;
		}
		let mut pit = pair.split(':');
		let field_name = pit.next().unwrap();
		if field_name == "cid" {
			continue;
		}
		let idx = match field_name {
			"byr" => 0,
			"iyr" => 1,
			"eyr" => 2,
			"hgt" => 3,
			"hcl" => 4,
			"ecl" => 5,
			"pid" => 6,
			"cid" => 7,
			_ => panic!("Unknown field: {field_name}"),
		};

		if extra_validation {
			let val = pit.next().unwrap();
			if !field_valid(field_name, val) {
				//println!("invalid field: {pair}");
				continue;
			}
		}

		fields |= 1 << idx;
	}
	if fields == 0b1111111 {
		valid_passports += 1;
	}
	valid_passports
}

fn field_valid(field_name :&str, val :&str) -> bool {
	let limits = match field_name {
		"byr" => Some(1920..=2002),
		"iyr" => Some(2010..=2020),
		"eyr" => Some(2020..=2030),
		_ => None,
	};
	if let Some(limits) = limits {
		// TODO use if let chains once they are stable
		if val.len() != 4 { return false; }
		let Ok(v) = u16::from_str(val) else {
			return false
		};
		if !limits.contains(&v) {
			return false;
		}
	}
	if field_name == "hgt" {
		let limits = if val.ends_with("cm") {
			150..=193
		} else if val.ends_with("in") {
			59..=76
		} else {
			return false
		};
		let num = &val[..val.len() - 2];
		let Ok(num) = u16::from_str(num) else {
			return false
		};
		if !limits.contains(&num) {
			return false;
		}
	} else if field_name == "hcl" {
		let mut vci = val.chars();
		if vci.next() != Some('#') || val.len() != 7 {
			return false;
		}
		if !vci.all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c)) {
			return false;
		}
	} else if field_name == "ecl" {
		match val {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
			_ => return false,
		}
	} else if field_name == "pid" {
		if val.len() != 9 {
			return false;
		}
		if !val.chars().all(|c| ('0'..='9').contains(&c)) {
			return false;
		}
	}
	true
}
