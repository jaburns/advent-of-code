pub fn main() {
    // Parse input in to vec of pairs for each group:
    //     ( Number of lines, all lines joined in to single string )
    let groups = std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| {
            (
                x.split("\n").collect::<Vec<_>>().len(),
                String::from(x.replace("\n", "")),
            )
        })
        .collect::<Vec<_>>();

    // Sum all of the unique characters in each group.
    let part1 = groups
        .iter()
        .map(|(_, string)| {
            let mut x = string.chars().collect::<Vec<_>>();
            x.sort();
            x.dedup();
            x.len()
        })
        .sum::<usize>();

    // Sum the number of characters whose count equals the line count for each group.
    let part2 = groups
        .iter()
        .map(|(lines, string)| {
            ('a'..='z')
                .filter(|x| string.matches(*x).count() == *lines)
                .count()
        })
        .sum::<usize>();

    println!("{} {}", part1, part2);
}
