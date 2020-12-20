use crate::day19_gen::{rule_1_0, rule_2_0};
use pom::parser::*;

fn rule_1_0_wrapped<'a>() -> Parser<'a, char, ()> {
    sym('^').discard() * rule_1_0() * sym('$').discard()
}
fn rule_2_0_wrapped<'a>() -> Parser<'a, char, ()> {
    sym('^').discard() * rule_2_0() * sym('$').discard()
}

pub fn main() {
    let patterns = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split("\n\n")
        .skip(1)
        .map(|x| {
            x.lines()
                .map(|x| {
                    let mut ret = String::from(x);
                    ret.insert(0, '^');
                    ret.insert(ret.len(), '$');
                    ret.chars().collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .last()
        .unwrap();

    let part1 = patterns
        .iter()
        .filter_map(|x| rule_1_0_wrapped().parse(x).ok())
        .count();

    let part2 = patterns
        .iter()
        .filter_map(|x| rule_2_0_wrapped().parse(x).ok())
        .count();

    println!("{} {}", part1, part2);
}
