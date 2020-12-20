use pom::parser::*;
use crate::day19_gen::rule_0;

fn rule_0_wrapped<'a>() -> Parser<'a, char, ()> {
    sym('^').discard() * rule_0() * sym('$').discard()
}

pub fn main() {
    let patterns = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split("\n\n")
        .skip(1)
        .map(|x| x.lines().map(|x| {
            let mut ret = String::from(x);
            ret.insert(0, '^');
            ret.insert(ret.len(), '$');
            ret.chars().collect::<Vec<_>>()
        }).collect::<Vec<_>>())
        .last()
        .unwrap();

    let part1 = patterns.iter().filter_map(|x| rule_0_wrapped().parse(x).ok()).count();

    println!("{:?}", part1);
}