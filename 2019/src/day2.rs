const IMI_HALT: usize = 99;
const IMI_ADD: usize = 1;
const IMI_MUL: usize = 2;

fn execute_tape(tape: &[usize], noun: usize, verb: usize) -> usize {
    let mut p = Vec::from(tape);
    let mut i = 0usize;

    p[1] = noun;
    p[2] = verb;

    loop {
        match p[i] {
            IMI_ADD => {
                let addr_out = p[i + 3];
                p[addr_out] = p[p[i + 1]] + p[p[i + 2]]
            }

            IMI_MUL => {
                let addr_out = p[i + 3];
                p[addr_out] = p[p[i + 1]] * p[p[i + 2]];
            }

            IMI_HALT => break,

            _ => panic!(
                "ABORTED: Encountered unknown opcode {} at location {}",
                p[i], i
            ),
        }
        i += 4;
    }

    p[0]
}

fn search_for_result(tape: &[usize], result: usize) -> (usize, usize) {
    let mut noun = 0usize;
    loop {
        for verb in 0..noun {
            let test = execute_tape(&tape, noun, verb);
            if test == result {
                return (noun, verb);
            }
        }
        noun += 1;
    }
}

pub fn main() {
    let tape: Vec<usize> = std::fs::read_to_string("data/day2.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = execute_tape(&tape, 12, 2);
    let (result1_noun, result1_verb) = search_for_result(&tape, 19690720);
    let result1 = 100 * result1_noun + result1_verb;

    println!("{} {}", result0, result1);
}
