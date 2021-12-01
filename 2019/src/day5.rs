use crate::intcode::vm::IntCodeMachine;

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day5.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let results0 = IntCodeMachine::run_all(&tape, &[1]);
    let results1 = IntCodeMachine::run_all(&tape, &[5]);

    let result0 = results0[results0.len() - 1];
    let result1 = results1[results1.len() - 1];

    println!("{} {}", result0, result1);
}
