use std::fmt::Write;

#[allow(clippy::char_lit_as_u8)]
const DOT: u8 = '.' as u8;

fn sum_part_numbers(line: &str, side0: &str, side1: &str) -> u64 {
    let mut sum = 0;

    let bytes = line.as_bytes();
    let bytes0 = side0.as_bytes();
    let bytes1 = side1.as_bytes();

    let mut digit_buf: [u8; 16] = Default::default();
    let mut digit_buf_idx: usize = 0;
    let mut valid = false;

    for (i, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            if digit_buf_idx == 0
                && i > 0
                && (bytes[i - 1] != DOT || bytes0[i - 1] != DOT || bytes1[i - 1] != DOT)
            {
                valid = true;
            }
            if bytes0[i] != DOT || bytes1[i] != DOT {
                valid = true;
            }
            digit_buf[digit_buf_idx] = ch as u8;
            digit_buf_idx += 1;
        } else if digit_buf_idx > 0 {
            if bytes[i] != DOT || bytes0[i] != DOT || bytes1[i] != DOT {
                valid = true;
            }
            if valid {
                let parsed = std::str::from_utf8(&digit_buf[0..digit_buf_idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                sum += parsed;
            }
            digit_buf_idx = 0;
            valid = false;
        }
    }
    if digit_buf_idx > 0 && valid {
        sum += std::str::from_utf8(&digit_buf[0..digit_buf_idx])
            .unwrap()
            .parse::<u64>()
            .unwrap();
    }

    sum
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut result = 0_u64;
    result += sum_part_numbers(lines[0], lines[1], lines[1]);
    result += sum_part_numbers(
        lines[lines.len() - 1],
        lines[lines.len() - 2],
        lines[lines.len() - 2],
    );
    for i in 1..(lines.len() - 1) {
        result += sum_part_numbers(lines[i], lines[i - 1], lines[i + 1])
    }
    write!(out, "{}", result).unwrap();
}

pub fn part2(_lines: &[&str], out: &mut String) {
    let result = 0;
    write!(out, "{}", result).unwrap();
}
