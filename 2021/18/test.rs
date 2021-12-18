use super::*;

#[test]
fn test_sums_simple() -> Result<()> {
	let sum = calculate_sum_lines("[1,1]\n[2,2]\n[3,3]\n[4,4]");
	assert_eq!(sum, Num::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")?);

	let sum = calculate_sum_lines("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]");
	assert_eq!(sum, Num::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")?);

	let sum = calculate_sum_lines("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]");
	assert_eq!(sum, Num::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")?);
	Ok(())
}

#[test]
fn test_sum_steps() -> Result<()> {
	let mut sum = Num::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")?;
	sum.add(Num::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")?);
	assert_eq!(sum, Num::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")?);
	Ok(())
}

const EXAMPLE_INPUT :&str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[test]
fn test_sum() -> Result<()> {
	let sum = calculate_sum_lines(EXAMPLE_INPUT);
	assert_eq!(sum, Num::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")?);
	assert_eq!(sum.magnitude(), 4140);
	Ok(())
}

#[test]
fn test_max_sum() -> Result<()> {
	let sum = max_sum(EXAMPLE_INPUT)?;
	assert_eq!(sum, Some(3993));
	Ok(())
}

#[test]
fn test_magnitude() -> Result<()> {
	let n = Num::parse("[[9,1],[1,9]]")?;
	assert_eq!(n.magnitude(), 129);
	let n = Num::parse("[[1,2],[[3,4],5]]")?;
	assert_eq!(n.magnitude(), 143);
	let n = Num::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?;
	assert_eq!(n.magnitude(), 1384);
	let n = Num::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")?;
	assert_eq!(n.magnitude(), 445);
	let n = Num::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")?;
	assert_eq!(n.magnitude(), 791);
	let n = Num::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")?;
	assert_eq!(n.magnitude(), 1137);
	let n = Num::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?;
	assert_eq!(n.magnitude(), 3488);
	Ok(())
}

#[test]
fn test_split() -> Result<()> {
	let mut n = Num::parse("10")?;
	assert!(n.maybe_split());
	assert_eq!(n, Num::parse("[5,5]")?);
	let mut n = Num::parse("11")?;
	assert!(n.maybe_split());
	assert_eq!(n, Num::parse("[5,6]")?);
	let mut n = Num::parse("12")?;
	assert!(n.maybe_split());
	assert_eq!(n, Num::parse("[6,6]")?);

	let mut n = Num::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]")?;
	assert!(n.maybe_split());
	assert_eq!(n, Num::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")?);
	assert!(n.maybe_split());
	assert_eq!(n, Num::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")?);

	Ok(())
}

#[test]
fn test_explode() -> Result<()> {
	let mut n = Num::parse("[[[[[9,8],1],2],3],4]")?;
	assert!(n.maybe_explode());
	assert_eq!(n, Num::parse("[[[[0,9],2],3],4]")?);

	let mut n = Num::parse("[7,[6,[5,[4,[3,2]]]]]")?;
	assert!(n.maybe_explode());
	assert_eq!(n, Num::parse("[7,[6,[5,[7,0]]]]")?);

	let mut n = Num::parse("[[6,[5,[4,[3,2]]]],1]")?;
	assert!(n.maybe_explode());
	assert_eq!(n, Num::parse("[[6,[5,[7,0]]],3]")?);

	let mut n = Num::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?;
	assert!(n.maybe_explode());
	assert_eq!(n, Num::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?);

	let mut n = Num::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?;
	assert!(n.maybe_explode());
	assert_eq!(n, Num::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")?);
	Ok(())
}
