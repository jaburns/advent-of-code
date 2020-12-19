
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
        let id_rest = s.split(":").collect::<Vec<_>>();
        let id = id_rest[0].parse::<u32>().unwrap();

        Self {
            id: id,
            body: RuleBody::Value('a')
        }
    }
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rules = chunks[0].iter().map(|x| Rule::parse(x.as_str())).collect::<Vec<_>>();

    println!("{:?}", rules);
}
