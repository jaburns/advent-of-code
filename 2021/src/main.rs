use std::time::Instant;

mod day1;
mod day2;
mod day3;

const NUM_DAYS: usize = 3;

#[global_allocator]
pub static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

type PartFn = fn(&[&str], &mut String);

static DAY_FUNCS: [(PartFn, PartFn); NUM_DAYS] = [
    (day1::part1, day1::part2),
    (day2::part1, day2::part2),
    (day3::part1, day3::part2),
];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        NUM_DAYS
    };

    let data = std::fs::read_to_string(&format!("data/day{}.txt", day)).unwrap();
    let data_lines: Vec<&str> = data.lines().collect();

    let mut out_str = String::with_capacity(256);

    println!();
    println!("Day {} - Part One", day);

    let reg = stats_alloc::Region::new(GLOBAL);
    let start_time = Instant::now();
    DAY_FUNCS[day - 1].0(&data_lines, &mut out_str);
    let delta_time = Instant::now() - start_time;
    let stats = reg.change();

    println!("Solution: {}", out_str);
    println!("Time (us): {}", delta_time.as_micros());
    println!("{:#?}", stats);

    out_str.clear();

    println!();
    println!("Day {} - Part Two", day);

    let reg = stats_alloc::Region::new(GLOBAL);
    let start_time = Instant::now();
    DAY_FUNCS[day - 1].1(&data_lines, &mut out_str);
    let delta_time = Instant::now() - start_time;
    let stats = reg.change();

    println!("Solution: {}", out_str);
    println!("Time (us): {}", delta_time.as_micros());
    println!("{:#?}", stats);
}
