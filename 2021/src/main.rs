use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const NUM_DAYS: usize = 19;

#[global_allocator]
pub static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

type PartFn = fn(&[&str], &mut String);

enum DayFn {
    Separate(PartFn, PartFn),
    Combined(PartFn),
}

static DAY_FUNCS: [DayFn; NUM_DAYS] = [
    DayFn::Separate(day1::part1, day1::part2),
    DayFn::Separate(day2::part1, day2::part2),
    DayFn::Separate(day3::part1, day3::part2),
    DayFn::Separate(day4::part1, day4::part2),
    DayFn::Separate(day5::part1, day5::part2),
    DayFn::Separate(day6::part1, day6::part2),
    DayFn::Separate(day7::part1, day7::part2),
    DayFn::Separate(day8::part1, day8::part2),
    DayFn::Separate(day9::part1, day9::part2),
    DayFn::Combined(day10::parts_1_and_2),
    DayFn::Separate(day11::part1, day11::part2),
    DayFn::Separate(day12::part1, day12::part2),
    DayFn::Separate(day13::part1, day13::part2),
    DayFn::Separate(day14::part1, day14::part2),
    DayFn::Separate(day15::part1, day15::part2),
    DayFn::Combined(day16::parts_1_and_2),
    DayFn::Combined(day17::parts_1_and_2),
    DayFn::Separate(day18::part1, day18::part2),
    DayFn::Combined(day19::parts_1_and_2),
];

fn main() {
    let data_ext = if std::env::args().any(|x| x == "--test") {
        "test"
    } else {
        "txt"
    };

    let args: Vec<String> = std::env::args().filter(|x| x != "--test").collect();

    let day = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        NUM_DAYS
    };

    let data = std::fs::read_to_string(&format!("data/day{}.{}", day, data_ext)).unwrap();
    let data_lines: Vec<&str> = data.trim().lines().map(|x| x.trim()).collect();

    let mut out_str = String::with_capacity(256);

    match DAY_FUNCS[day - 1] {
        DayFn::Separate(f1, f2) => {
            println!();
            println!("Day {} - Part One", day);

            let reg = stats_alloc::Region::new(GLOBAL);
            let start_time = Instant::now();
            f1(&data_lines, &mut out_str);
            let delta_time = start_time.elapsed();
            let stats = reg.change();

            println!("Solution: {}", out_str);
            println!("Time (μs): {}", delta_time.as_micros());
            println!(
                "Memory (bytes): {}",
                stats.bytes_allocated + stats.bytes_reallocated.max(0) as usize
            );
            println!("Allocations: {}", stats.allocations + stats.reallocations);

            out_str.clear();

            println!();
            println!("Day {} - Part Two", day);

            let reg = stats_alloc::Region::new(GLOBAL);
            let start_time = Instant::now();
            f2(&data_lines, &mut out_str);
            let delta_time = start_time.elapsed();
            let stats = reg.change();

            println!("Solution: {}", out_str);
            println!("Time (μs): {}", delta_time.as_micros());
            println!(
                "Memory (bytes): {}",
                stats.bytes_allocated + stats.bytes_reallocated.max(0) as usize
            );
            println!("Allocations: {}", stats.allocations + stats.reallocations);
            println!();
        }
        DayFn::Combined(f) => {
            println!();
            println!("Day {} - Parts One and Two", day);

            let reg = stats_alloc::Region::new(GLOBAL);
            let start_time = Instant::now();
            f(&data_lines, &mut out_str);
            let delta_time = start_time.elapsed();
            let stats = reg.change();

            println!("Solution: {}", out_str);
            println!("Time (μs): {}", delta_time.as_micros());
            println!(
                "Memory (bytes): {}",
                stats.bytes_allocated + stats.bytes_reallocated.max(0) as usize
            );
            println!("Allocations: {}", stats.allocations + stats.reallocations);
            println!();
        }
    }
}
