use std::fmt::Write;

pub fn part1(raw_input: &str, out: &mut String) {
    let lines = raw_input.lines();
    let bit_count = lines.clone().next().unwrap().len();
    let line_count = lines.clone().count();

    let mut counts = vec![0u64; bit_count];

    for line in lines.clone() {
        for (i, b) in line.chars().enumerate() {
            counts[bit_count - 1 - i] += (b == '1') as u64;
        }
    }

    let gamma = counts
        .iter()
        .map(|x| *x > line_count as u64 / 2)
        .enumerate()
        .fold(0u64, |acc, (i, x)| acc | (x as u64) << i);

    let all_ones = 2u64.pow(bit_count as u32) - 1;

    let result = gamma * (!gamma & all_ones);

    write!(out, "{}", result).unwrap();
}

pub fn part2(raw_input: &str, out: &mut String) {
    let lines = raw_input.lines();
    let num_lines = lines.clone().count();
    let mut lines_vec: Vec<&str> = vec![""; num_lines];
    for (i, line) in lines.enumerate() {
        lines_vec[i] = line;
    }

    fn check(mut lines: Vec<&str>, keep_ones_if: impl Fn(u64, u64) -> bool) -> u64 {
        let mut idx: usize = 0;

        loop {
            let mut ones_count = 0u64;
            let mut zeroes_count = 0u64;

            for line in lines.iter() {
                if line.as_bytes()[idx] == b'1' {
                    ones_count += 1;
                } else {
                    zeroes_count += 1;
                }
            }

            lines.retain(|line| {
                keep_ones_if(ones_count, zeroes_count) == (line.as_bytes()[idx] == b'1')
            });

            if lines.len() < 2 {
                return lines[0]
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(0u64, |acc, (i, bit)| acc | ((bit == '1') as u64) << i);
            }

            idx += 1;
        }
    }

    let o2 = check(lines_vec.clone(), |ones, zeroes| ones >= zeroes);
    let co2 = check(lines_vec, |ones, zeroes| ones < zeroes);

    let result = o2 * co2;

    write!(out, "{}", result).unwrap();
}
