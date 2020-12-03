#[derive(Debug)]
struct Rule {
    min: u32,
    max: u32,
    letter: char,
}

impl Rule {
    pub fn new(text: &str) -> Self {
        let parts: Vec<&str> = text.split(' ').collect();
        let min_max_part: Vec<&str> = parts[0].split('-').collect();

        Self {
            min: min_max_part[0].parse().unwrap(),
            max: min_max_part[1].parse().unwrap(),
            letter: parts[1].chars().nth(0).unwrap(),
        }
    }

    pub fn validates_password_1(&self, pass: &str) -> bool {
        let count = pass
            .chars()
            .fold(0u32, |a, x| a + (x == self.letter) as u32);
        count >= self.min && count <= self.max
    }

    pub fn validates_password_2(&self, pass: &str) -> bool {
        let a = pass.chars().nth(self.min as usize - 1).unwrap();
        let b = pass.chars().nth(self.max as usize - 1).unwrap();
        a == self.letter && b != self.letter || a != self.letter && b == self.letter
    }
}

fn parse_line(line: &str) -> (Rule, String) {
    let parts: Vec<&str> = line.split(':').collect();
    (Rule::new(parts[0]), String::from(parts[1].trim()))
}

pub fn main() {
    let entries: Vec<_> = std::fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        .map(|x| parse_line(x.trim()))
        .collect();

    let part1_count = entries.iter().fold(0u32, |a, (rule, pass)| {
        a + rule.validates_password_1(pass.as_str()) as u32
    });

    let part2_count = entries.iter().fold(0u32, |a, (rule, pass)| {
        a + rule.validates_password_2(pass.as_str()) as u32
    });

    println!("{} {}", part1_count, part2_count);
}
