use arrayvec::{ArrayString, ArrayVec};
use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let pairs = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .zip(
            lines[1]
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<i64>().unwrap()),
        )
        .collect::<ArrayVec<_, 4>>();

    let mut result_1 = 1_i64;
    for &(time, dist) in pairs.iter() {
        result_1 *= get_ways_to_beat(time, dist)
    }

    let time_2 = parse_bad_kerning_number(&lines[0][9..]);
    let dist_2 = parse_bad_kerning_number(&lines[1][9..]);

    let result_2 = get_ways_to_beat(time_2, dist_2);

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn parse_bad_kerning_number(line: &str) -> i64 {
    let mut chars = ArrayString::<32>::new();
    for digit in line.chars().filter(|x| x.is_ascii_digit()) {
        chars.push(digit);
    }
    chars.parse::<i64>().unwrap()
}

fn get_ways_to_beat(time: i64, dist: i64) -> i64 {
    let sqrt = ((time * time - 4 * dist) as f64).sqrt();
    let lower = (time as f64 - sqrt + 1e-9) * 0.5;
    let upper = (time as f64 + sqrt - 1e-9) * 0.5;
    upper as i64 - lower as i64
}
