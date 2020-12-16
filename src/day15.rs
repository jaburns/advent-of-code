const MAX_ITERATIONS: usize = 30_000_000;

fn run_game(iterations: usize, start_data: &[u32]) -> u32 {
    let mut turns_for_nums = vec![std::u32::MAX; MAX_ITERATIONS];

    let mut turn = start_data.len() as u32;
    let mut prev = *start_data.last().unwrap();

    for i in 0..(start_data.len() - 1) {
        turns_for_nums[start_data[i] as usize] = i as u32;
    }

    while turn < iterations as u32 {
        let prev_turn = turns_for_nums[prev as usize];
        turns_for_nums[prev as usize] = turn - 1;

        prev = match prev_turn {
            std::u32::MAX => 0,
            i => turn - i - 1,
        };

        turn += 1;
    }

    prev
}

pub fn main() {
    let data = std::fs::read_to_string("data/day15.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let part1 = run_game(2020, &data);
    let part2 = run_game(30_000_000, &data);

    println!("\n{} {}\n", part1, part2);
}
