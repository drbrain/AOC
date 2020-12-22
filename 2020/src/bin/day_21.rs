use anyhow::Result;

// use aoc2020::read;

// use nom::bytes::complete::*;
// use nom::character::complete::*;
// use nom::combinator::*;
// use nom::multi::*;
// use nom::sequence::*;
// use nom::IResult;
//
// use std::collections::HashMap;

fn main() -> Result<()> {
    //let input = read("./21.input")?;

    //println!("part A: {}", day_21_a(&input));
    //println!("part B: {}", day_21_b(&input));

    Ok(())
}

// fn day_21_a(input: &str) -> usize {
//     let foods = parse(input).unwrap().1;
//     let mut ingredient_allergens: HashMap<String, HashMap<String, usize>> = HashMap::new();
//     let mut allergen_counts: HashMap<String, usize> = HashMap::new();
//     let mut ingredient_counts: HashMap<String, usize> = HashMap::new();
//
//     for food in foods {
//         for allergen in food.allergens {
//             for ingredient in &food.ingredients {
//                 if let Some(m) = ingredient_allergens.get_mut(ingredient) {
//                     if let Some(c) = m.get(&allergen) {
//                         m.insert(allergen.clone(), c + 1);
//                     } else {
//                         m.insert(allergen.clone(), 1);
//                     }
//                 } else {
//                     let mut map: HashMap<String, usize> = HashMap::new();
//                     map.insert(allergen.clone(), 1);
//                     ingredient_allergens.insert(ingredient.to_string(), map);
//                 }
//             }
//
//             if let Some(c) = allergen_counts.get(&allergen) {
//                 allergen_counts.insert(allergen, c + 1);
//             } else {
//                 allergen_counts.insert(allergen, 1);
//             }
//         }
//
//         for ingredient in &food.ingredients {
//             if let Some(c) = ingredient_counts.get(ingredient) {
//                 ingredient_counts.insert(ingredient.clone(), c + 1);
//             } else {
//                 ingredient_counts.insert(ingredient.clone(), 1);
//             }
//         }
//     }
//
//     let allergen_free: Vec<&String> = ingredient_allergens
//         .iter()
//         .filter_map(|(ingredient, allergens)| {
//             if allergens
//                 .iter()
//                 .any(|(allergen, count)| allergen_counts.get(allergen).unwrap_or(&0) == count)
//             {
//                 None
//             } else {
//                 Some(ingredient)
//             }
//         })
//         .collect();
//
//     allergen_free
//         .iter()
//         .map(|ingredient| ingredient_counts.get(*ingredient).unwrap_or(&0))
//         .sum()
// }
//
// fn day_21_b(input: &str) -> String {
//     let foods = parse(input).unwrap().1;
//     let mut ingredient_allergens: HashMap<String, HashMap<String, usize>> = HashMap::new();
//     let mut allergen_ingredients: HashMap<String, HashMap<String, usize>> = HashMap::new();
//     let mut allergen_counts: HashMap<String, usize> = HashMap::new();
//     let mut ingredient_counts: HashMap<String, usize> = HashMap::new();
//
//     for food in foods {
//         for allergen in food.allergens {
//             for ingredient in &food.ingredients {
//                 if let Some(m) = ingredient_allergens.get_mut(ingredient) {
//                     if let Some(c) = m.get(&allergen) {
//                         m.insert(allergen.clone(), c + 1);
//                     } else {
//                         m.insert(allergen.clone(), 1);
//                     }
//                 } else {
//                     let mut map: HashMap<String, usize> = HashMap::new();
//                     map.insert(allergen.clone(), 1);
//                     ingredient_allergens.insert(ingredient.to_string(), map);
//                 }
//
//                 if let Some(m) = allergen_ingredients.get_mut(&allergen) {
//                     if let Some(c) = m.get(ingredient) {
//                         m.insert(ingredient.clone(), c + 1);
//                     } else {
//                         m.insert(ingredient.clone(), 1);
//                     }
//                 } else {
//                     let mut map: HashMap<String, usize> = HashMap::new();
//                     map.insert(ingredient.clone(), 1);
//                     allergen_ingredients.insert(allergen.to_string(), map);
//                 }
//             }
//
//             if let Some(c) = allergen_counts.get(&allergen) {
//                 allergen_counts.insert(allergen, c + 1);
//             } else {
//                 allergen_counts.insert(allergen, 1);
//             }
//         }
//
//         for ingredient in &food.ingredients {
//             if let Some(c) = ingredient_counts.get(ingredient) {
//                 ingredient_counts.insert(ingredient.clone(), c + 1);
//             } else {
//                 ingredient_counts.insert(ingredient.clone(), 1);
//             }
//         }
//     }
//
//     let allergen_free: Vec<&String> = ingredient_allergens
//         .iter()
//         .filter_map(|(ingredient, allergens)| {
//             if allergens
//                 .iter()
//                 .any(|(allergen, count)| allergen_counts.get(allergen).unwrap_or(&0) == count)
//             {
//                 None
//             } else {
//                 Some(ingredient)
//             }
//         })
//         .collect();
//
//     let allergenic: Vec<&String> = ingredient_allergens
//         .iter()
//         .filter_map(|(ingredient, allergens)| {
//             if allergens
//                 .iter()
//                 .any(|(allergen, count)| allergen_counts.get(allergen).unwrap_or(&0) == count)
//             {
//                 Some(ingredient)
//             } else {
//                 None
//             }
//         })
//         .collect();
//
//     let mut a_i = allergen_ingredients.clone();
//
//     for allergen in allergen_ingredients.keys() {
//         if let Some(ingredients) = a_i.get_mut(allergen) {
//             for ingredient in &allergen_free {
//                 ingredients.remove(*ingredient);
//             }
//         }
//     }
//
//     dbg!(a_i);
//     dbg!(&allergenic);
//
//     "".to_string()
// }
//
// #[derive(Debug)]
// struct Food {
//     ingredients: Vec<String>,
//     allergens: Vec<String>,
// }
//
// fn parse(input: &str) -> IResult<&str, Vec<Food>> {
//     separated_list1(tag("\n"), food)(input)
// }
//
// fn food(input: &str) -> IResult<&str, Food> {
//     map(
//         pair(
//             ingredients,
//             preceded(tag(" (contains "), terminated(allergens, tag(")"))),
//         ),
//         |(i, a)| Food {
//             ingredients: i,
//             allergens: a,
//         },
//     )(input)
// }
//
// fn ingredients(input: &str) -> IResult<&str, Vec<String>> {
//     separated_list1(tag(" "), map(recognize(alpha1), String::from))(input)
// }
//
// fn allergens(input: &str) -> IResult<&str, Vec<String>> {
//     separated_list1(tag(", "), map(recognize(alpha1), String::from))(input)
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_day_21_a() {
//         let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)";
//
//         assert_eq!(5, day_21_a(&input));
//     }
//
//     #[test]
//     fn test_day_21_b() {
//         let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)";
//
//         assert_eq!("mxmxvkd,sqjhc,fvjkl".to_string(), day_21_b(&input));
//     }
// }
