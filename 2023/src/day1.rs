use itertools::Either;
use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let result_1: u32 = lines
        .iter()
        .map(|s| {
            let a = s
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let b = s
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            10 * a + b
        })
        .sum();

    let result_2: u32 = lines
        .iter()
        .map(|s| {
            let a = find_digit(s, false);
            let b = find_digit(s, true);
            10 * a + b
        })
        .sum();

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn find_digit(string: &str, reverse: bool) -> u32 {
    const DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let range = if reverse {
        Either::Left((0..string.len()).rev())
    } else {
        Either::Right(0..string.len())
    };
    for i in range {
        let x = string.as_bytes()[i];
        if x.is_ascii_digit() {
            #[allow(clippy::char_lit_as_u8)]
            return (x - ('0' as u8)) as u32;
        }
        for (dig, digit) in DIGITS.iter().enumerate() {
            if i + digit.len() <= string.len() && &string[i..(i + digit.len())] == *digit {
                return dig as u32 + 1;
            }
        }
    }
    panic!()
}
