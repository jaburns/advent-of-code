mod day1;
mod day2;

fn run_from_arg(arg: i32) {
    match arg {
        1 => day1::main(),
        2 => day2::main(),
        _ => {}
    }
}

fn run_default() {
    day1::main()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run_from_arg(args[1].parse::<i32>().unwrap())
    } else {
        run_default()
    }
}
