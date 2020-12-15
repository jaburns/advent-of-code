use std::collections::HashMap;

fn run_game(iterations: usize, start_data: &[usize], show_log: bool) -> usize {
    let mut turns_for_nums = HashMap::<usize, usize>::new();
    let mut turn = start_data.len();
    let mut prev = *start_data.last().unwrap();
    let log_rate = iterations / 100;

    for i in 0..(start_data.len() - 1) {
        turns_for_nums.insert(start_data[i], i);
    }

    while turn < iterations {
        if show_log && turn % log_rate == 0 {
            println!("Progress: {} %", 100 * turn / iterations);
        }

        let prev_turn = turns_for_nums.get(&prev).map(|x| *x);
        turns_for_nums.insert(prev, turn - 1);

        prev = match prev_turn {
            Some(i) => turn - i - 1,
            None => 0,
        };

        turn += 1;
    }

    prev
}

pub fn main() {
    let data = std::fs::read_to_string("data/day15.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = run_game(2020, &data, false);
    let part2 = run_game(30_000_000, &data, true);

    println!("\n{} {}\n", part1, part2);
}
