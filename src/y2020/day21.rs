use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

pub type PuzzleInput = Vec<Food>;

#[derive(Debug)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[aoc_generator(day21)]
pub fn parser(input: &str) -> PuzzleInput {
    lazy_static! {
        // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        static ref PATTERN: Regex = Regex::new(r"^(?P<ingredients>.*) \(contains (?P<allergens>.*)\)").unwrap();
    }

    input
        .lines()
        .map(|l| {
            let captures = PATTERN.captures(l).unwrap();
            let ingredients = captures
                .name("ingredients")
                .unwrap()
                .as_str()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<HashSet<_>>();
            let allergens = captures
                .name("allergens")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<HashSet<_>>();

            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

pub fn run(data: &PuzzleInput) -> Calculated {
    let mut known_ingredients = HashSet::<String>::new();
    let mut known_allergens = HashSet::<String>::new();

    for f in data {
        for i in &f.ingredients {
            known_ingredients.insert(i.to_string());
        }
        for a in &f.allergens {
            known_allergens.insert(a.to_string());
        }
    }

    let mut mapped_allergens = HashMap::<String, HashSet<String>>::new();
    let mut potentially_identified_ingredients = HashSet::<String>::new();

    for a in &known_allergens {
        let foods_definitely_containing_allergen = data
            .iter()
            .filter(|f| f.allergens.contains(a))
            .collect::<Vec<_>>();
        let first_food = foods_definitely_containing_allergen.first().unwrap();

        let mut ingredients_that_could_be_this_allergen = first_food.ingredients.clone();
        for f in &foods_definitely_containing_allergen[1..] {
            ingredients_that_could_be_this_allergen = ingredients_that_could_be_this_allergen
                .intersection(&f.ingredients)
                .map(|x| x.to_string())
                .collect::<HashSet<_>>();
        }

        for i in &ingredients_that_could_be_this_allergen {
            potentially_identified_ingredients.insert(i.to_string());
        }

        mapped_allergens.insert(a.to_string(), ingredients_that_could_be_this_allergen);
    }

    Calculated {
        known_ingredients,
        known_allergens,
        potentially_identified_ingredients,
        mapped_allergens
    }
}

pub struct Calculated {
    known_ingredients: HashSet::<String>,
    known_allergens: HashSet::<String>,
    potentially_identified_ingredients:HashSet::<String>,
    mapped_allergens: HashMap<String, HashSet<String>>
}

#[aoc(day21, part1)]
pub fn day21_part1(data: &PuzzleInput) -> usize {

    let info = run(data);

    let mut count = 0;
    for ingredient in info.known_ingredients {
        if !info.potentially_identified_ingredients.contains(&ingredient) {
            // This ingredient cannot be an allergen
            let foods_mentioning_ingredient = data.iter().filter(|food| food.ingredients.contains(&ingredient)).count();
            count += foods_mentioning_ingredient;
        }
    }
    count
}

#[aoc(day21, part2)]
pub fn day21_part2(data: &PuzzleInput) -> String {
    let info = run(data);

    // ingredient to allergen name
    let mut ingredient_to_allergen = HashMap::<String, String>::new();
    let mut allergen_to_ingredient = HashMap::<String, String>::new();
    
    while ingredient_to_allergen.len() < info.known_allergens.len() {
        for (allergen, ingredients) in &info.mapped_allergens {
            let unknown_ingredients = ingredients.iter().filter(|i| !ingredient_to_allergen.contains_key(*i)).collect::<Vec<_>>();

            if unknown_ingredients.len() == 1 {
                // Found!
                let i = unknown_ingredients.first().unwrap().to_string();
                ingredient_to_allergen.insert(i.to_string(), allergen.to_string());
                allergen_to_ingredient.insert(allergen.to_string(), i);
            }
        }
    }

    let mut sorted_known_allergens = info.known_allergens.iter().collect::<Vec<_>>();
    sorted_known_allergens.sort();
    sorted_known_allergens.iter().map(|allergen| {
        allergen_to_ingredient.get(&allergen.to_string()).unwrap()
    }).join(",")
}
