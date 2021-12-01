use crate::intcode::vm::IntCodeMachine;

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day9.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = IntCodeMachine::run_all(&tape, &[1]).pop().unwrap();
    let result1 = IntCodeMachine::run_all(&tape, &[2]).pop().unwrap();

    println!("{} {}", result0, result1);
}
