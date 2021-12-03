use std::fmt::Write;

pub fn part1(data: &str, out: &mut String) {
    let ints: Vec<i64> = data.lines().map(|x| x.parse().unwrap()).collect();

    let result = ints.windows(2).filter(|win| win[1] > win[0]).count();

    write!(out, "{}", result).unwrap();
}

pub fn part2(data: &str, out: &mut String) {
    let ints: Vec<i64> = data.lines().map(|x| x.parse().unwrap()).collect();

    let sums: Vec<i64> = ints.windows(3).map(|xs| xs.iter().sum()).collect();

    let result = sums.windows(2).filter(|win| win[1] > win[0]).count();

    write!(out, "{}", result).unwrap();
}
