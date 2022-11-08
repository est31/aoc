use super::*;

const EXAMPLE_INPUT_1 :&str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

#[test]
fn test_1() {
	let (hm, foods) = parse(EXAMPLE_INPUT_1);
	println!("foods={foods:?}");
	let nalg = get_no_allergens(&foods);
	assert_eq!(nalg.len(), 4);
	assert!(nalg.contains(&hm.get("kfcds").unwrap()));
	assert!(nalg.contains(&hm.get("nhms").unwrap()));
	assert!(nalg.contains(&hm.get("sbzzf").unwrap()));
	assert!(nalg.contains(&hm.get("trh").unwrap()));
}

#[test]
fn test_2() {
	let (hm, foods) = parse(EXAMPLE_INPUT_1);
	let list = get_dangerous_ingredient_list(&hm, &foods);
	assert_eq!(list, "mxmxvkd,sqjhc,fvjkl");
}
