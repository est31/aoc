use std::collections::HashMap;
const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let system = parse(INPUT);
	println!("Path number: {}", system.path_count());
	println!("Path number ext: {}", system.path_count_ext());
}

fn parse(input :&str) -> CaveSystem {
	let lines = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());

	let mut names = HashMap::new();
	let mut caves = Vec::new();
	for l in lines {
		let mut components = l.split('-');
		let cave_1 = components.next().unwrap();
		let cave_2 = components.next().unwrap();
		let l = names.len();
		let cave_1_idx = *names.entry(cave_1).or_insert_with(|| {
			caves.push(Cave::empty_from_name(cave_1));
			l
		});
		let l = names.len();
		let cave_2_idx = *names.entry(cave_2).or_insert_with(|| {
			caves.push(Cave::empty_from_name(cave_2));
			l
		});
		caves[cave_1_idx].connected.push(cave_2_idx);
		caves[cave_2_idx].connected.push(cave_1_idx);
	}
	let start_idx = names[&"start"];
	let end_idx = names[&"end"];
	CaveSystem {
		caves,
		start_idx,
		end_idx,
	}
}

struct Cave {
	#[allow(unused)]
	name :String,
	small :bool,
	connected :Vec<usize>,
}

impl Cave {
	fn empty_from_name(name :&str) -> Self {
		Self {
			name : name.to_string(),
			small : name.starts_with(|c| char::is_ascii_lowercase(&c)),
			connected : Vec::new(),
		}
	}
}

struct CaveSystem {
	caves :Vec<Cave>,
	start_idx :usize,
	end_idx :usize,
}

impl CaveSystem {
	fn path_count(&self) -> usize {
		let mut visited = vec![0; self.caves.len()];
		self.path_count_inner(self.start_idx, &mut visited, false)
	}
	fn path_count_inner(&self, start :usize, visited :&mut [u16],
			may_visit_twice :bool) -> usize {
		if start == self.end_idx {
			if may_visit_twice {
				return 0;
			} else {
				return 1;
			}
		}
		let mut paths = 0;
		visited[start] += 1;
		for &c_i in self.caves[start].connected.iter() {
			let c = &self.caves[c_i];
			let mut may_visit_twice = may_visit_twice;
			if c.small && visited[c_i] > 0 {
				if may_visit_twice && c_i != self.start_idx {
					may_visit_twice = false;
				} else {
					continue;
				}
			}
			paths += self.path_count_inner(c_i, visited, may_visit_twice);
		}
		visited[start] -= 1;
		paths
	}
	fn path_count_ext(&self) -> usize {
		let mut visited = vec![0; self.caves.len()];
		self.path_count_inner(self.start_idx, &mut visited, true) +
		self.path_count_inner(self.start_idx, &mut visited, false)
	}
}
