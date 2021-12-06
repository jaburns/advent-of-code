use std::fmt::Write;

// Solutions
// 350917
// 1592918715629

fn run_fish_sim(input: &str, days: u32) -> u64 {
    let mut fishes_by_day = [0_u64; 9];

    for day in input.split(',').map(|x| x.parse::<u8>().unwrap()) {
        fishes_by_day[day as usize] += 1;
    }

    for _ in 0..days {
        let zeroes = fishes_by_day[0];
        for i in 1..9 {
            fishes_by_day[i - 1] = fishes_by_day[i];
        }
        fishes_by_day[6] += zeroes;
        fishes_by_day[8] = zeroes;
    }

    fishes_by_day.iter().sum()
}

pub fn part1(lines: &[&str], out: &mut String) {
    write!(out, "{}", run_fish_sim(lines[0], 80)).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    write!(out, "{}", run_fish_sim(lines[0], 256)).unwrap();
}
