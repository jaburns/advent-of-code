use crate::intcode::assembler::assemble;
use crate::intcode::vm::IntCodeMachine;

pub fn main() {
    let mut digits: Vec<i64> = std::fs::read_to_string("data/day8.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();

    digits.push(99);

    let tape = assemble("intcode/day8.asm", false);
    let results = IntCodeMachine::run_all(&tape, &digits);

    let mut out_image = String::new();
    for y in 0..6 {
        for x in 0..25 {
            out_image.push_str(if results[1 + y * 25 + x] == 1 {
                "X"
            } else {
                " "
            })
        }
        out_image.push_str("\n");
    }

    println!("{}\n\n{}", results[0], out_image);
}
