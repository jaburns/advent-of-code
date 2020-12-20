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

    fn create_map_by_id(rules: &[Rule]) -> HashMap<RuleId, Rule> {
        let mut ret = HashMap::new();

        for rule in rules {
            ret.insert(rule.id, rule.clone());
        }

        ret
    }

    fn get_regex(&self, rules_by_id: &HashMap<RuleId, Rule>) -> String {
        match &self.body {
            RuleBody::Value(chr) => String::from(*chr),
            RuleBody::SingleSub(xs) => xs.iter()
                .map(|x| rules_by_id[x].get_regex(rules_by_id))
                .collect::<Vec<String>>()
                .join(""),
            RuleBody::BranchSub(xs, ys) => {
                let mut s = String::from("(");
                s += xs.iter()
                    .map(|x| rules_by_id[x].get_regex(rules_by_id))
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str();
                s += "|";
                s += ys.iter()
                    .map(|x| rules_by_id[x].get_regex(rules_by_id))
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str();
                s += ")";
                s
            },
        }
    }
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rules = chunks[0]
        .iter()
        .map(|x| Rule::parse(x.as_str()))
        .collect::<Vec<_>>();

    let patterns = chunks[1]
        .iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let rules_by_id = Rule::create_map_by_id(&rules);
    let mut regex_src = rules_by_id[&0].get_regex(&rules_by_id);
    regex_src.insert(0, '^');
    regex_src += "$";

    let regex = Regex::new(regex_src.as_str()).unwrap();

    let mut matches = 0u32;
    for pattern in patterns {
        matches += regex.is_match(pattern.as_str()) as u32;
    }

    println!("{:?}", matches);
}
