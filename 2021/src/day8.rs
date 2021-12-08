use std::collections::HashSet;
use std::fmt::Write;

fn parse_digit(dig: &str) -> u32 {
    let chars = dig.chars().collect::<HashSet<_>>();
    let len = chars.len();

    if len == 2 {
        return 1;
    }
    if chars.contains(&'a')
        && !chars.contains(&'b')
        && chars.contains(&'c')
        && chars.contains(&'d')
        && chars.contains(&'e')
        && !chars.contains(&'f')
        && chars.contains(&'g')
    {
        return 2;
    }
    if chars.contains(&'a')
        && !chars.contains(&'b')
        && chars.contains(&'c')
        && chars.contains(&'d')
        && !chars.contains(&'e')
        && chars.contains(&'f')
        && chars.contains(&'g')
    {
        return 3;
    }
    if len == 4 {
        return 4;
    }
    if chars.contains(&'a')
        && chars.contains(&'b')
        && !chars.contains(&'c')
        && chars.contains(&'d')
        && !chars.contains(&'e')
        && chars.contains(&'f')
        && chars.contains(&'g')
    {
        return 5;
    }
    if chars.contains(&'a')
        && chars.contains(&'b')
        && !chars.contains(&'c')
        && chars.contains(&'d')
        && chars.contains(&'e')
        && chars.contains(&'f')
        && chars.contains(&'g')
    {
        return 6;
    }
    if len == 3 {
        return 7;
    }
    if len == 7 {
        return 8;
    }
    return 9;
}

fn parse_number(num: &str) -> usize {
    let digits = num.trim().split(' ').map(parse_digit).rev();
    println!("{:?}", digits.collect::<Vec<_>>());
    0
}

pub fn part1(lines: &[&str], out: &mut String) {
    let result: usize = lines
        .iter()
        .map(|line| {
            let mut iter = line.split('|');
            iter.next();
            iter.next()
                .unwrap()
                .split(' ')
                .map(|x| x.trim().len())
                .filter(|&len| len == 2 || len == 4 || len == 3 || len == 7)
                .count()
        })
        .sum();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result: usize = lines
        .iter()
        .map(|line| {
            let mut iter = line.split('|');
            iter.next();
            parse_number(iter.next().unwrap())
        })
        .sum();

    write!(out, "{}", result).unwrap();
}
