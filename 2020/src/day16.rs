use std::collections::HashSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct RuleId(usize);

#[derive(Debug, Clone)]
struct Rule {
    id: RuleId,
    name: String,
    ranges: Vec<(u64, u64)>,
}

fn parse_rules(s: &str) -> Vec<Rule> {
    s.lines()
        .enumerate()
        .map(|(i, x)| {
            let chunks = x.split(": ").collect::<Vec<_>>();
            Rule {
                id: RuleId(i),
                name: String::from(chunks[0]),
                ranges: chunks[1]
                    .split(" or ")
                    .map(|r| {
                        let parts = r
                            .split("-")
                            .map(|v| v.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        (parts[0], parts[1])
                    })
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct NumberPos(usize);

#[derive(Debug, Clone)]
struct Ticket {
    numbers: Vec<(NumberPos, u64)>,
}

impl Ticket {
    pub fn parse(s: &str) -> Self {
        Self {
            numbers: s
                .split(",")
                .enumerate()
                .map(|(i, x)| (NumberPos(i), x.parse::<u64>().unwrap()))
                .collect(),
        }
    }

    pub fn is_valid(&self, rules: &[Rule]) -> bool {
        self.sum_invalid_values(rules) == 0
    }

    pub fn sum_invalid_values(&self, rules: &[Rule]) -> u64 {
        let mut ret = 0u64;

        'top: for &(_pos, n) in &self.numbers {
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

    pub fn get_rule_breakers(&self, rules: &[Rule]) -> Vec<(NumberPos, RuleId)> {
        self.numbers
            .iter()
            .flat_map(|&(pos, n)| {
                let mut breakers = Vec::new();

                'top: for rule in rules.iter() {
                    for &(min, max) in &rule.ranges {
                        if n >= min && n <= max {
                            continue 'top;
                        }
                    }
                    breakers.push((pos, rule.id));
                }

                breakers
            })
            .collect()
    }

    pub fn number_at_pos(&self, pos: NumberPos) -> u64 {
        self.numbers.iter().find(|(p, _val)| *p == pos).unwrap().1
    }
}

fn find_departure_rules_product(
    rules: &[Rule],
    nearby_tickets: &[Ticket],
    my_ticket: &Ticket,
) -> u64 {
    let constraints = nearby_tickets
        .iter()
        .flat_map(|x| x.get_rule_breakers(rules))
        .collect::<Vec<_>>();

    // For each number position, a set of rules which might apply
    let mut scratch = vec![rules.iter().map(|x| x.id).collect::<HashSet<_>>(); rules.len()];

    // For each number position, the singular rule which must apply
    let mut solution = vec![RuleId(999); rules.len()];

    // Apply initial constraints
    for &(NumberPos(np), rule_id) in &constraints {
        if scratch[np].contains(&rule_id) {
            scratch[np].remove(&rule_id);
        }
    }

    // Note any sets of one, save them to the solution, and clear them from all other sets
    loop {
        let mut solved_rule_id = None as Option<RuleId>;

        for np in 0..scratch.len() {
            if scratch[np].len() == 1 {
                let id_val = scratch[np].drain().last().unwrap();
                solved_rule_id = Some(id_val);
                solution[np] = id_val;
                break;
            }
        }

        match &solved_rule_id {
            Some(rule_id) => {
                for np in 0..scratch.len() {
                    scratch[np].remove(rule_id);
                }
            }
            None => break,
        }
    }

    // Invert the relationship in `solution`, giving a mapping from rule to ticket number position
    let mut pos_for_rule = vec![NumberPos(999); rules.len()];
    for i in 0..solution.len() {
        let RuleId(rid) = solution[i];
        pos_for_rule[rid] = NumberPos(i);
    }

    // Compute the product for the final result
    let mut product = 1;
    for rule in rules {
        if rule.name.starts_with("departure") {
            let RuleId(idx) = rule.id;
            product *= my_ticket.number_at_pos(pos_for_rule[idx]);
        }
    }

    product
}

pub fn main() {
    let chunks = std::fs::read_to_string("data/day16.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<_>>();

    let rules = parse_rules(chunks[0].as_str());
    let my_ticket = Ticket::parse(chunks[1].lines().last().unwrap());
    let nearby_tickets = chunks[2]
        .lines()
        .skip(1)
        .map(Ticket::parse)
        .collect::<Vec<_>>();

    let part1 = nearby_tickets
        .iter()
        .map(|x| x.sum_invalid_values(&rules))
        .sum::<u64>();

    let valid_tickets = nearby_tickets
        .iter()
        .filter(|x| x.is_valid(&rules))
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let part2 = find_departure_rules_product(&rules, valid_tickets.as_slice(), &my_ticket);

    println!("{} {}", part1, part2);
}
