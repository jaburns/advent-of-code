#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

fn parse_rules(s: &str) -> Vec<Rule> {
    s.lines()
        .map(|x| {
            let chunks = x.split(": ").collect::<Vec<_>>();
            Rule {
                name: String::from(chunks[0]),
                ranges: chunks[1]
                    .split(" or ")
                    .map(|r| {
                        let parts = r
                            .split("-")
                            .map(|v| v.parse::<u32>().unwrap())
                            .collect::<Vec<_>>();
                        (parts[0], parts[1])
                    })
                    .collect(),
            }
        })
        .collect()
}

fn parse_ticket(s: &str) -> Vec<u32> {
    s.split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

fn invalid_values_sum(rules: &[Rule], ticket: &[u32]) -> u32 {
    let mut ret = 0u32;

    'top: for &n in ticket {
        for rule in rules {
            for &(min, max) in &rule.ranges {
                if n >= min && n <= max {
                    continue 'top;
                }
            }
        }
        ret += n;
    }

    ret
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day16.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<_>>();

    let rules = parse_rules(chunks[0].as_str());
    let _my_ticket = parse_ticket(chunks[1].lines().last().unwrap());
    let nearby_tickets = chunks[2]
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Vec<_>>();

    let part1 = nearby_tickets
        .iter()
        .map(|x| invalid_values_sum(&rules, x))
        .sum::<u32>();

    println!("{}", part1);
}
