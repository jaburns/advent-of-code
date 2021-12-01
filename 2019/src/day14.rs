use std::cmp::min;
use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Recipe {
    pub output_count: u64,
    pub ingredients: Vec<(String, u64)>,
}

type RecipeBook = HashMap<String, Recipe>;
type ChemStore = HashMap<String, u64>;

fn load_recipe_book_from_strings(txts: &[String]) -> RecipeBook {
    let mut result = RecipeBook::new();

    fn parse_ingredient(txt: &str) -> (String, u64) {
        let left_right: Vec<&str> = txt.split(" ").collect();
        (
            String::from(left_right[1]),
            left_right[0].parse::<u64>().unwrap(),
        )
    }

    for txt in txts {
        let left_right: Vec<&str> = txt.split("=>").collect();
        let input_list: Vec<&str> = left_right[0].split(",").map(|x| x.trim()).collect();
        let output = left_right[1].trim();

        let parsed_output = parse_ingredient(output);

        let ingredients = input_list.iter().map(|&x| parse_ingredient(x)).collect();

        result.insert(
            parsed_output.0,
            Recipe {
                output_count: parsed_output.1,
                ingredients: ingredients,
            },
        );
    }

    result
}

fn produce_fuel(out_quantity: u64, recipes: &RecipeBook, chem_store: &mut ChemStore) -> u64 {
    let mut needed = VecDeque::new();
    needed.push_back(("FUEL", out_quantity));
    let mut ore_used = 0;

    loop {
        match needed.pop_front() {
            Some(("ORE", mut amount)) => {
                let stored_chem = chem_store.entry(String::from("ORE")).or_insert(0);
                let store_used = min(amount, *stored_chem);
                *stored_chem -= store_used;
                amount -= store_used;

                ore_used += amount;
            }
            Some((chem, mut amount)) => {
                let stored_chem = chem_store.entry(String::from(chem)).or_insert(0);
                let store_used = min(amount, *stored_chem);
                *stored_chem -= store_used;
                amount -= store_used;

                if amount > 0 {
                    let recipe = recipes.get(chem).unwrap();
                    let multiplier = (amount - 1) / recipe.output_count + 1;

                    *stored_chem = recipe.output_count * multiplier - amount;

                    for (input_name, input_count) in &recipe.ingredients {
                        needed.push_back((&input_name, input_count * multiplier));
                    }
                }
            }
            None => return ore_used,
        };
    }
}

pub fn main() {
    let recipe_lines: Vec<String> = std::fs::read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let recipes = load_recipe_book_from_strings(&recipe_lines);
    let ore_per_fuel = produce_fuel(1, &recipes, &mut ChemStore::new());

    let base_fuel = 1_000_000_000_000 / ore_per_fuel;
    let mut chem_store = ChemStore::new();
    let mut ore_used = produce_fuel(base_fuel, &recipes, &mut chem_store);
    let mut fuel_produced = base_fuel;
    let mut chunk_size = 32768_u64;

    while chunk_size > 0 {
        let mut sim_store = chem_store.clone();
        let mut sim_ore_used = ore_used;
        let mut sim_fuel_produced = 0;

        loop {
            sim_ore_used += produce_fuel(chunk_size, &recipes, &mut sim_store);
            if sim_ore_used > 1_000_000_000_000 {
                break;
            }
            sim_fuel_produced += chunk_size;
        }

        fuel_produced += sim_fuel_produced;
        ore_used += produce_fuel(sim_fuel_produced, &recipes, &mut sim_store);
        chunk_size /= 2;
    }

    let result0 = ore_per_fuel;
    let result1 = fuel_produced;

    println!("{} {}", result0, result1);
}
