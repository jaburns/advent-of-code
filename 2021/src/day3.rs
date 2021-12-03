use std::collections::VecDeque;

pub fn part2(raw_input: &str) -> u64 {
    let lines: Vec<_> = raw_input
        .lines()
        .map(|line| (line, line.chars().collect::<VecDeque<_>>()))
        .collect();

    fn check(mut lines: Vec<(&str, VecDeque<char>)>, keep_ones: impl Fn(u64, u64) -> bool) -> u64 {
        loop {
            let mut ones_count = 0u64;
            let mut zeroes_count = 0u64;

            for (_, line) in lines.iter() {
                if line[0] == '1' {
                    ones_count += 1;
                } else {
                    zeroes_count += 1;
                }
            }

            lines.retain(|(_, line)| keep_ones(ones_count, zeroes_count) == (line[0] == '1'));

            if lines.len() < 2 {
                return lines[0]
                    .0
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(0u64, |acc, (i, bit)| acc | ((bit == '1') as u64) << i);
            }

            for (_, line) in lines.iter_mut() {
                line.pop_front().unwrap();
            }
        }
    }

    let o2 = check(lines.clone(), |ones, zeroes| ones >= zeroes);
    let co2 = check(lines, |ones, zeroes| ones < zeroes);

    o2 * co2
}

pub fn main(raw_input: &str) -> (i64, i64) {
    let lines = raw_input.lines();
    let bit_count = lines.clone().next().unwrap().len();
    let line_count = lines.clone().count();

    let power_consumption = {
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

        (gamma * (!gamma & all_ones)) as i64
    };

    let life_support = part2(raw_input) as i64;

    (power_consumption, life_support)
}
