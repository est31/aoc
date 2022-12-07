use super::*;

const EXAMPLE_INPUT :&str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

";

#[test]
fn test_1() {
	let dh = parse(EXAMPLE_INPUT);
	assert_eq!(dir_and_small_dir_size(&dh, &mut |_| ()), (48381165, 95437));
}

#[test]
fn test_2() {
	let dh = parse(EXAMPLE_INPUT);
	assert_eq!(smallest_that_frees(&dh), 24933642);
}
