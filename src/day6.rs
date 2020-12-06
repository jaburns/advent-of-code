use std::collections::HashMap;

fn chars_in_all_lines(lines: &Vec<String>) -> usize {
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
    let groups = std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.split("\n").map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = groups
        .iter()
        .map(|x| {
            let mut chs = x.iter().map(|x| x.chars()).flatten().collect::<Vec<char>>();
            chs.sort();
            chs.dedup();
            chs.len()
        })
        .sum::<usize>();

    let part2 = groups
        .iter()
        .map(chars_in_all_lines)
        .sum::<usize>();

    println!("{} {}", part1, part2);
}