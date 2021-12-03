use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let ints: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let result = ints.windows(2).filter(|win| win[1] > win[0]).count();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let ints: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let sums: Vec<i64> = ints.windows(3).map(|xs| xs.iter().sum()).collect();

    let result = sums.windows(2).filter(|win| win[1] > win[0]).count();

    write!(out, "{}", result).unwrap();
}

use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
