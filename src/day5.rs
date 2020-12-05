
fn parse_line(line: &str) -> String {
    String::from(line)
}

pub fn main() {
    let entries: Vec<_> = std::fs::read_to_string("data/day5.txt")
        .unwrap()
        .lines()
        .map(|x| parse_line(x.trim()))
        .collect();

    let part1 = &entries[0];
    let part2 = &entries[1];

    println!("{} {}", part1, part2);
}
