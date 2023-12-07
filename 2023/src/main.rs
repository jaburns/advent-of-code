#![feature(array_chunks)]

use nix::time::{clock_gettime, ClockId};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[global_allocator]
pub static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

static DAY_FUNCS: &[fn(&[&str], &mut String)] = &[
    day1::parts_1_and_2,
    day2::parts_1_and_2,
    day3::parts_1_and_2,
    day4::parts_1_and_2,
    day5::parts_1_and_2,
    day6::parts_1_and_2,
    day7::parts_1_and_2,
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
        DAY_FUNCS.len()
    };

    let data = std::fs::read_to_string(format!("data/day{}.{}", day, data_ext)).unwrap();
    let data_lines: Vec<&str> = data.trim().lines().map(|x| x.trim()).collect();

    let mut out_str = String::with_capacity(256);

    println!();
    println!("Day {} - Parts One and Two", day);

    let reg = stats_alloc::Region::new(GLOBAL);
    let start_time = clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap();
    (DAY_FUNCS[day - 1])(&data_lines, &mut out_str);
    let end_time = clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap();
    let stats = reg.change();
    let delta_time = end_time - start_time;
    let micros = delta_time.tv_nsec() / 1000 + delta_time.tv_sec() * 1_000_000;

    println!("Solution:      {}", out_str);
    println!("Time (μs):     {}", micros);
    println!(
        "Heap (bytes):  {}",
        stats.bytes_allocated + stats.bytes_reallocated.max(0) as usize
    );
    println!("Allocations:   {}", stats.allocations + stats.reallocations);
    println!();
}
