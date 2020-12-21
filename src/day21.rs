use std::collections::{HashSet, HashMap};

type Food = String;
type Allergen = String;
type DataLines = Vec<(HashSet<Food>, HashSet<Allergen>)>;

#[derive(Debug,Clone)]
struct InputData {
    lines: DataLines,
    all_foods: HashSet<Food>,
    all_allergens: HashSet<Allergen>,
}

fn load_data() -> InputData {
    let mut ret = InputData {
        lines: Vec::new(),
        all_foods: HashSet::new(),
        all_allergens: HashSet::new(),
    };

    for line in std::fs::read_to_string("data/day21.txt").unwrap().lines() {
        let mut datum = (HashSet::new(), HashSet::new());
        let chunks = line.split(" (contains ").collect::<Vec<_>>();

        for food in chunks[0].split(" ") {
            ret.all_foods.insert(String::from(food));
            datum.0.insert(String::from(food));
        }

        for allergen in chunks[1].replace(")", "").split(", ") {
            ret.all_allergens.insert(String::from(allergen));
            datum.1.insert(String::from(allergen));
        }

        ret.lines.push(datum);
    }

    ret
}

fn find_minimal_constraint_set(input: &InputData) -> HashMap<Allergen, HashSet<Food>> {
    let mut lines = input.lines.clone();
    lines.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());

    let mut found_allergens = HashSet::<Allergen>::new();
    let mut ret = HashMap::new();

    for l in lines {
        let len0 = found_allergens.len();
        for all in &l.1 {
            found_allergens.insert(all.clone());
            if !ret.contains_key(all) {
                ret.insert(all.clone(), HashSet::new());
            }
        }
        if found_allergens.len() > len0 {
            for all in &l.1 {
                for food in &l.0 {
                    ret.get_mut(all).unwrap().insert(food.clone());
                }
            }
        }
    }

    ret
}

fn mapping_produces_contradiction(test_allergen: &Allergen, test_food: &Food, input: &InputData) -> bool {
    for l in &input.lines {
        if l.1.contains(test_allergen) && !l.0.contains(test_food) {
            return true;
        }
    }

    false
}

fn solve_mapping(input: &InputData) -> HashMap<Allergen, Food> {
    let guesses = find_minimal_constraint_set(&input);
    let mut reduced_guesses = HashMap::<Allergen, HashSet<Food>>::new();

    for (allergen, guess_foods) in &guesses {
        reduced_guesses.insert(allergen.clone(), HashSet::new());
        for food in guess_foods {
            if !mapping_produces_contradiction(allergen, food, &input) {
                reduced_guesses.get_mut(allergen).unwrap().insert(food.clone());
            }
        }
    }

    let mut ret = HashMap::new();
    loop {
        let mut found_food = String::new();

        for (allergen, guess_foods) in &reduced_guesses {
            if guess_foods.len() == 1 {
                found_food = guess_foods.iter().last().unwrap().clone();
                ret.insert(allergen.clone(), found_food.clone());
                break;
            }
        }

        if found_food == "" {
            break;
        }

        for (_, guess_foods) in &mut reduced_guesses {
            guess_foods.remove(&found_food);
        }
    }

    ret
}

fn count_good_food_occurances(input: &InputData, bad_foods: &[Food]) -> u32 {
    let mut ret = 0u32;

    for l in &input.lines {
        ret += l.0.iter().filter(|x| !bad_foods.contains(x)).count() as u32;
    }
    
    ret
}

pub fn main() {
    let input = load_data();
    let mappings = solve_mapping(&input);

    let part1 = count_good_food_occurances(&input, mappings.values().map(|x| x.clone()).collect::<Vec<_>>().as_slice());

    let part2 = {
        let mut map_list = mappings.iter().collect::<Vec<_>>();
        map_list.sort_by(|(a,_), (b,_)| a.partial_cmp(b).unwrap());
        map_list.iter().map(|(_,x)| (*x).clone()).collect::<Vec<_>>().join(",")
    };

    println!("{} {}", part1, part2);
}
