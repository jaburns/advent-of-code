use std::collections::HashMap;

use regex::Regex;

type RuleId = u32;

#[derive(Debug, Clone)]
struct Rule {
    id: RuleId,
    body: RuleBody,
}

#[derive(Debug, Clone)]
enum RuleBody {
    Value(char),
    SingleSub(Vec<RuleId>),
    BranchSub(Vec<RuleId>, Vec<RuleId>),
}

#[derive(Debug, Clone)]
struct RuleSet(HashMap<RuleId, Rule>);

impl Rule {
    pub fn parse(s: &str) -> Self {
        let id_rest = s.split(": ").collect::<Vec<_>>();
        let id = id_rest[0].parse::<u32>().unwrap();

        fn parse_num_vec(s: &str) -> Vec<u32> {
            s.split(" ")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        }

        Self {
            id: id,
            body: if id_rest[1].contains("|") {
                let parts = id_rest[1]
                    .split(" | ")
                    .map(parse_num_vec)
                    .collect::<Vec<_>>();
                RuleBody::BranchSub(parts[0].clone(), parts[1].clone())
            } else if id_rest[1].contains("\"") {
                RuleBody::Value(id_rest[1].replace("\"", "").chars().last().unwrap())
            } else {
                RuleBody::SingleSub(parse_num_vec(id_rest[1]))
            },
        }
    }
}

impl RuleSet {
    pub fn parse(lines: &[String]) -> Self {
        let mut map = HashMap::new();

        let rules = lines
            .iter()
            .map(|x| Rule::parse(x.as_str()))
            .collect::<Vec<_>>();

        for rule in rules {
            map.insert(rule.id, rule.clone());
        }

        Self(map)
    }

    fn get_rule_regex(&self, rule_id: RuleId) -> String {
        let rule = &self.0[&rule_id];
        match &rule.body {
            RuleBody::Value(chr) => String::from(*chr),
            RuleBody::SingleSub(xs) => xs
                .iter()
                .map(|x| self.get_rule_regex(*x))
                .collect::<Vec<String>>()
                .join(""),
            RuleBody::BranchSub(xs, ys) => {
                let mut s = String::from("(");
                s += xs
                    .iter()
                    .map(|x| self.get_rule_regex(*x))
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str();
                s += "|";
                s += ys
                    .iter()
                    .map(|x| self.get_rule_regex(*x))
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str();
                s += ")";
                s
            }
        }
    }

    pub fn get_root_regex(&self) -> Regex {
        let mut src = self.get_rule_regex(0);
        src.insert(0, '^');
        src += "$";
        Regex::new(src.as_str()).unwrap()
    }

    pub fn get_patched_root_regex(&self) -> Regex {
        let max_depth = 8;
        let mut src = String::new();

        src += "(";
        src += self.get_rule_regex(42).as_str();
        src += ")+";

        src += "(";
        for i in 1..max_depth {
            for _ in 0..i {
                src += self.get_rule_regex(42).as_str();
            }
            for _ in 0..i {
                src += self.get_rule_regex(31).as_str();
            }
            src += if i == max_depth - 1 { ")" } else { "|" };
        }

        src.insert(0, '^');
        src += "$";

        Regex::new(src.as_str()).unwrap()
    }
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ruleset = RuleSet::parse(&chunks[0]);

    let patterns = chunks[1].iter().map(String::from).collect::<Vec<_>>();

    let regex = ruleset.get_root_regex();
    let part1 = patterns.iter().filter(|x| regex.is_match(x)).count();

    let regex = ruleset.get_patched_root_regex();
    let part2 = patterns.iter().filter(|x| regex.is_match(x)).count();

    println!("{} {}", part1, part2);
}
