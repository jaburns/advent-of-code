#![feature(array_chunks, array_windows, iter_collect_into)]
#![allow(clippy::char_lit_as_u8)]

use nix::time::{clock_gettime, ClockId};

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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    day8::parts_1_and_2,
    day9::parts_1_and_2,
    day10::parts_1_and_2,
    day11::parts_1_and_2,
    day12::parts_1_and_2,
    day13::parts_1_and_2,
    day14::parts_1_and_2,
    day15::parts_1_and_2,
    day16::parts_1_and_2,
    day17::parts_1_and_2,
    day18::parts_1_and_2,
    day19::parts_1_and_2,
    day20::parts_1_and_2,
    day21::parts_1_and_2,
    day22::parts_1_and_2,
    // day23::parts_1_and_2,
    // day24::parts_1_and_2,
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
