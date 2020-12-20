use std::collections::HashMap;

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

    fn evaluate(&self, rules_by_id: &HashMap<RuleId, Rule>) -> String {
        match &self.body {
            RuleBody::Value(chr) => String::from(*chr),
            RuleBody::SingleSub(xs) => xs.iter().map(|x| rules_by_id[x].evaluate(rules_by_id)).collect::<Vec<_>>().join(""),
            RuleBody::BranchSub(xs, ys) => String::new(),
        }
    }
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day19.test")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rules = chunks[0]
        .iter()
        .map(|x| Rule::parse(x.as_str()))
        .collect::<Vec<_>>();

    let rules_by_id = Rule::create_map_by_id(&rules);
    let part1 = rules_by_id[&0].evaluate( &rules_by_id );

    println!("{:?}", part1);
}
