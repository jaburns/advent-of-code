use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let crabs: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let max_coord = crabs.iter().fold(0i32, |a, x| a.max(*x));

    let fuels = (0..max_coord).map(|i| crabs.iter().map(|x| (x - i as i32).abs()).sum::<i32>());
    let min_fuel = fuels.fold(std::i32::MAX, |a, x| a.min(x));

    let result = min_fuel;
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let crabs: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let max_coord = crabs.iter().fold(0i32, |a, x| a.max(*x));

    let fuels = (0..max_coord).map(|i| {
        crabs
            .iter()
            .map(|x| {
                let delta = (x - i as i32).abs();
                delta * (delta + 1) / 2
            })
            .sum::<i32>()
    });
    let min_fuel = fuels.fold(std::i32::MAX, |a, x| a.min(x));

    let result = min_fuel;
    write!(out, "{}", result).unwrap();
}
