mod day1;
mod day2;
mod day3;

use day3 as default_day;

fn run_from_arg(arg: i32) {
    match arg {
        2 => day2::main(),
        3 => day3::main(),
        1 => day1::main(),
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run_from_arg(args[1].parse::<i32>().unwrap())
    } else {
        default_day::main();
    }
}
