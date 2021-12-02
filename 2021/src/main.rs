mod day1;
mod day2;

#[global_allocator]
static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

const NUM_DAYS: usize = 2;

#[allow(clippy::type_complexity)]
static DAY_FUNCS: [fn(&str) -> (i64, i64); NUM_DAYS] = [day1::main, day2::main];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        NUM_DAYS
    };

    let data = std::fs::read_to_string(&format!("data/day{}.txt", day)).unwrap();

    let reg = stats_alloc::Region::new(GLOBAL);

    let (a, b) = DAY_FUNCS[day - 1](&data);

    println!("\n  {}  {}\n\n{:#?}\n", a, b, reg.change());
}
