use std::str::Lines;

pub fn main(raw_input: &str) -> (i64, i64) {
    let lines = raw_input.lines();

    let part1 = calc_power_consumption(lines.clone()) as i64;
    let part2 = calc_life_support(lines) as i64;

    (part1, part2)
}

fn calc_power_consumption(lines: Lines) -> u64 {
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

    gamma * (!gamma & all_ones)
}

fn calc_life_support(lines: Lines) -> u64 {
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

    let lines: Vec<&str> = lines.collect();

    let o2 = check(lines.clone(), |ones, zeroes| ones >= zeroes);
    let co2 = check(lines, |ones, zeroes| ones < zeroes);

    o2 * co2
}
