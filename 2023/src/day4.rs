use std::{collections::HashSet, fmt::Write};

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let (colon, pipe) = (lines[0].find(':').unwrap(), lines[0].find('|').unwrap());
    let mut winners = HashSet::new();
    let mut copies = vec![1; lines.len()];
    let mut result_1 = 0;
    let mut result_2 = 0;

    for (i, line) in lines.iter().enumerate() {
        winners.clear();

        for n in line[(colon + 1)..pipe].split_whitespace() {
            let x = n.parse::<u32>().unwrap();
            winners.insert(x);
        }

        let mut count = 0;
        for n in line[(pipe + 1)..].split_whitespace() {
            let x = n.parse::<u32>().unwrap();
            if winners.contains(&x) {
                count += 1;
            }
        }

        result_1 += (1 << count) >> 1;

        let mul = copies[i];
        for cp in &mut copies[(i + 1)..(i + 1 + count)] {
            *cp += mul;
            result_2 += mul;
        }
    }

    write!(out, "{}  {}", result_1, result_2).unwrap();
}
