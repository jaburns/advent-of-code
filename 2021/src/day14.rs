use hashbrown::HashMap;
use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    simulate_chain_and_get_disparity(10, lines, out);
}

pub fn part2(lines: &[&str], out: &mut String) {
    simulate_chain_and_get_disparity(40, lines, out);
}

type RuleSet = HashMap<(char, char), char>;

type PairCounts = HashMap<(char, char), u64>;

fn simulate_chain_and_get_disparity(iterations: u32, lines: &[&str], out: &mut String) {
    let mut rules = RuleSet::default();
    for line in lines[2..].iter() {
        parse_rule(&mut rules, line);
    }

    let chars = lines[0].chars().collect::<Vec<_>>();

    let mut counts = PairCounts::default();
    for pair in chars.windows(2) {
        *counts.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        apply_rules_to_pair_counts(&rules, &mut counts);
    }

    let result = get_max_element_count_difference(chars[0], &counts);
    write!(out, "{}", result).unwrap();
}

fn parse_rule(rules: &mut RuleSet, line: &str) {
    let mut parts = line.split(" -> ");

    let mut input_pair = parts.next().unwrap().chars();
    let input_0 = input_pair.next().unwrap();
    let input_1 = input_pair.next().unwrap();

    let out = parts.next().unwrap().chars().next().unwrap();

    rules.insert((input_0, input_1), out);
}

fn apply_rules_to_pair_counts(rules: &RuleSet, pair_counts: &mut PairCounts) {
    let mut new_pair_counts = PairCounts::default();

    for (pair, count) in pair_counts.iter() {
        let new_char = *rules.get(pair).unwrap();

        *new_pair_counts.entry((pair.0, new_char)).or_insert(0) += count;
        *new_pair_counts.entry((new_char, pair.1)).or_insert(0) += count;
    }

    pair_counts.clone_from(&new_pair_counts);
}

fn get_max_element_count_difference(first_element: char, pair_counts: &PairCounts) -> u64 {
    let mut counts = HashMap::<char, u64>::new();
    counts.insert(first_element, 1);
    for ((_, b), count) in pair_counts.iter() {
        *counts.entry(*b).or_insert(0) += *count;
    }

    let mut counts: Vec<u64> = counts.into_iter().map(|(_, x)| x).collect();
    counts.sort_unstable();
    counts[counts.len() - 1] - counts[0]
}
