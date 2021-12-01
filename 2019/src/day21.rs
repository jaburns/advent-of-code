use crate::intcode::vm::IntCodeMachine;

fn build_program(lines: &[&str]) -> Vec<i64> {
    if lines.len() > 15 {
        panic!("Springscript program too long!");
    }

    let mut result_str = String::new();

    for line in lines {
        result_str.push_str(line);
        result_str.push_str("\n");
    }

    result_str.chars().map(|x| x as i64).collect()
}

fn run_program_or_print_failure(tape: &[i64], program_lines: &[&str]) -> i64 {
    let program = build_program(program_lines);
    let result = IntCodeMachine::run_all(tape, &program);

    if *result.last().unwrap() > 0xFF {
        return *result.last().unwrap();
    }

    let debug_frame: String = result.iter().map(|&x| x as u8 as char).collect();
    println!("{}", debug_frame);

    0
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day21.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    #[rustfmt::skip]
    let result0 = run_program_or_print_failure(&tape, &[
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK",
    ]);

    #[rustfmt::skip]
    let result1 = run_program_or_print_failure(&tape, &[
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "NOT E T",
        "NOT T T",
        "OR H T",
        "AND T J",
        "RUN",
    ]);

    println!("{} {}", result0, result1);
}
