use hashbrown::HashMap;
use std::fmt::Write;

type RuleSet = HashMap<(char, char), char>;

fn parse_rule(rules: &mut RuleSet, line: &str) {
    let mut parts = line.split(" -> ");

    let mut input_pair = parts.next().unwrap().chars();
    let input_0 = input_pair.next().unwrap();
    let input_1 = input_pair.next().unwrap();

    let out = parts.next().unwrap().chars().next().unwrap();

    rules.insert((input_0, input_1), out);
}

fn apply_rules_to_chain(rules: &RuleSet, chain: &[char]) -> Vec<char> {
    let mut ret: Vec<char> = chain
        .windows(2)
        .flat_map(|win| {
            if let Some(rule) = rules.get(&(win[0], win[1])) {
                vec![win[0], *rule]
            } else {
                vec![win[0]]
            }
        })
        .collect();

    ret.push(chain[chain.len() - 1]);

    ret
}

fn count_chain_elements(chain: &[char]) -> Vec<(char, u64)> {
    let mut counts = HashMap::<char, u64>::new();
    for ch in chain {
        *counts.entry(*ch).or_insert(0) += 1;
    }
    counts.into_iter().collect()
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut chain: Vec<char> = lines[0].chars().collect();
    let mut rules = RuleSet::default();

    for line in lines[2..].iter() {
        parse_rule(&mut rules, line);
    }

    for _ in 0..10 {
        chain = apply_rules_to_chain(&rules, &chain);
    }

    let mut counts = count_chain_elements(&chain);

    counts.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let result = counts[0].1 - counts[counts.len() - 1].1;

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
