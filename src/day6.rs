use std::collections::{HashMap, HashSet};

fn unique_chars_in_any_lines(lines: &[String]) -> usize {
    let mut answers = HashSet::<char>::new();

    for line in lines {
        for ch in line.chars() {
            answers.insert(ch);
        }
    }
    answers.len()
}

fn chars_in_all_lines(lines: &[String]) -> usize {
    let mut answers = HashMap::<char, usize>::new();

    for line in lines {
        for ch in line.chars() {
            if !answers.contains_key(&ch) {
                answers.insert(ch, 1);
            } else {
                *answers.get_mut(&ch).unwrap() += 1;
            }
        }
    }

    answers
        .iter()
        .fold(0usize, |acc, (_, &v)| acc + (v == lines.len()) as usize)
}

pub fn main() {
    let lines = std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .lines()
        .map(|x| String::from(x.trim()))
        .collect::<Vec<_>>();

    let groups = lines.split(|x| x.trim().len() < 1).collect::<Vec<_>>();

    let part1: usize = groups.iter().map(|&x| unique_chars_in_any_lines(x)).sum();

    let part2: usize = groups.iter().map(|&x| chars_in_all_lines(x)).sum();

    println!("{} {}", part1, part2);
}
