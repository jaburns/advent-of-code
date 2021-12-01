fn fuel_from_mass(mass: i32) -> i32 {
    std::cmp::max(mass / 3 - 2, 0)
}

fn true_fuel_from_mass(mass: i32) -> i32 {
    let mut result = 0i32;
    let mut working_mass = mass;

    while working_mass > 0 {
        working_mass = fuel_from_mass(working_mass);
        result += working_mass;
    }

    result
}

fn get_fuel_using(masses: &[i32], calc: fn(i32) -> i32) -> i32 {
    masses.iter().map(|x| calc(*x)).sum()
}

pub fn main() {
    let masses: Vec<i32> = std::fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = get_fuel_using(&masses, fuel_from_mass);
    let result1 = get_fuel_using(&masses, true_fuel_from_mass);

    println!("{} {}", result0, result1);
}
