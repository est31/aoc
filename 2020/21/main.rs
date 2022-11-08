use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let (hm, foods) = parse(INPUT);
	let num = count_no_allergens(&foods);
	println!("Appearances: {num}");
	let dangerous_ing_list = get_dangerous_ingredient_list(&hm, &foods);
	println!("Dangerous ingredients: {dangerous_ing_list}");
}

fn parse(input :&str) -> (HashMap<String, usize>, Vec<(Vec<usize>, Vec<usize>)>) {
	let mut hm = HashMap::new();
	let l = input.lines()
		.filter(|l| !l.is_empty())
		.map(|l| {
			let (first, second) = l.split_once(" (contains ").unwrap();
			let first_words = first.split_whitespace()
				.map(|w| {
					let len = hm.len();
					*hm.entry(w.to_string()).or_insert(len)
				})
				.collect::<Vec<_>>();
			let (second, _) = second.split_once(")").unwrap();
			let second_words = second.split(", ")
				.map(|w| {
					let len = hm.len();
					*hm.entry(w.to_string()).or_insert(len)
				})
				.collect::<Vec<_>>();
			(first_words, second_words)
		})
		.collect::<Vec<_>>();
	//println!("hm={hm:?}");
	(hm, l)
}

fn get_ingredients_for_allergens(list :&[(Vec<usize>, Vec<usize>)]) -> HashMap<usize, HashSet<usize>> {
	let mut ingredients_for_allergens = HashMap::<_, HashSet<usize>>::new();
	for (recipe, allergens) in list.iter() {
		for allergen in allergens {
			let recipe_hm :HashSet<_> = recipe.iter().copied().collect();
			let entry = ingredients_for_allergens.entry(*allergen);
			match entry {
				Entry::Occupied(mut o) => {
					let o = o.get_mut();
					*o = o.intersection(&recipe_hm)
						.copied()
						.collect::<HashSet<_>>();
				},
				Entry::Vacant(v) => {
					v.insert(recipe_hm);
				},
			}
		}
	}
	//println!("ing_for_allg={ingredients_for_allergens:?}");
	ingredients_for_allergens
}

fn get_no_allergens(list :&[(Vec<usize>, Vec<usize>)]) -> HashSet<usize> {
	let ingredients_for_allergens = get_ingredients_for_allergens(&list);
	let mut res = list.iter()
		.map(|(recipe, _allergens)| recipe.iter())
		.flatten()
		.copied()
		.collect::<HashSet<_>>();
	for (_allergen, ingredients) in ingredients_for_allergens {
		for ing in ingredients {
			res.remove(&ing);
		}
	}
	//println!("res={res:?}");
	res
}

fn count_no_allergens(list :&[(Vec<usize>, Vec<usize>)]) -> usize {
	let nalg = get_no_allergens(list);
	list.iter()
		.map(|(ingr, _allergens)| {
			ingr.iter()
				.filter(|v| nalg.contains(v))
				.count()
		})
		.sum()
}

fn get_dangerous_ingredient_list(hm :&HashMap<String, usize>, list :&[(Vec<usize>, Vec<usize>)]) -> String {
	let rev_hm = hm.iter()
		.map(|(k, v)| (v, k))
		.collect::<HashMap<_, _>>();
	let mut ing_vec = Vec::new();
	// TODO don't compute ingredients_for_allergens twice
	let no_allergens = get_no_allergens(&list);
	let mut ingredients_for_allergens = get_ingredients_for_allergens(&list);

	let mut added = HashSet::new();
	let mut change = true;

	//println!("ing_for_allg={ingredients_for_allergens:?}");
	while change {
		change = false;
		for (_allergen, ingredients) in ingredients_for_allergens.iter_mut() {
			if ingredients.len() == 1 {
				added.insert(*ingredients.iter().next().unwrap());
				continue;
			}
			ingredients.retain(|ing| {
				let found_added = added.contains(ing);
				change |= found_added;
				!found_added
			});
		}

		let mut rev_ing = HashMap::<_, usize>::new();
		for (_allergen, ingredients) in ingredients_for_allergens.iter() {
			for ing in ingredients.iter() {
				*rev_ing.entry(*ing).or_default() += 1;
			}
		}
		for (_allergen, ingredients) in ingredients_for_allergens.iter_mut() {
			if ingredients.len() == 1 {
				continue;
			}
			let unique = ingredients.iter()
				.copied()
				.find(|ing| *rev_ing.get(&ing).unwrap() == 1);
			let Some(unique) = unique else { continue };
			ingredients.clear();
			ingredients.insert(unique);
			change = true;
		}
	}
	//println!("ing_for_allg={ingredients_for_allergens:?}");

	for (allergen, ingredients) in ingredients_for_allergens {
		assert_eq!(ingredients.len(), 1);
		for ing in ingredients {
			if no_allergens.contains(&ing) {
				continue;
			}
			ing_vec.push((allergen, rev_hm.get(&ing).unwrap()));
		}
	}
	ing_vec.sort_by_key(|(allg, _ing)| rev_hm.get(allg).unwrap());
	let res = ing_vec.iter()
		.map(|(_all, ing)| ing.as_str())
		.collect::<Vec<_>>();
	res.join(",")
}
