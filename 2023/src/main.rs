use std::time::Instant;

mod day1;
mod day2;
mod day3;

#[global_allocator]
pub static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

type PartFn = fn(&[&str], &mut String);

enum DayFn {
    Separate(PartFn, PartFn),
    #[allow(unused)]
    Combined(PartFn),
}

static DAY_FUNCS: &[DayFn] = &[
    DayFn::Separate(day1::part1, day1::part2),
    DayFn::Separate(day2::part1, day2::part2),
    DayFn::Separate(day3::part1, day3::part2),
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
