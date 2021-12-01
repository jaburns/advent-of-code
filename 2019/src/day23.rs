use crate::intcode::vm::{IntCodeMachine, RunResult};
use std::collections::VecDeque;

pub fn run_network(tape: &[i64], mode2: bool) -> i64 {
    let mut machines = Vec::new();
    let mut buffers = Vec::<VecDeque<(i64, i64)>>::new();
    let mut nat_packet = (0, 0);
    let mut reading_from_empty = Vec::new();
    let mut run_once = false;
    let mut last_nat_y = 0;

    for i in 0..50 {
        let mut machine = IntCodeMachine::new(&tape);
        machine.run_and_provide_input(i).unwrap();
        machines.push(machine);

        buffers.push(VecDeque::new());
        reading_from_empty.push(false);
    }

    loop {
        for i in 0..50 {
            let machine = machines.get_mut(i).unwrap();
            reading_from_empty[i] = false;

            match machine.run() {
                RunResult::Halted => panic!("Machine halted!"),
                RunResult::RequiresInput => {
                    if let Some((x, y)) = buffers[i].pop_front() {
                        machine.provide_input(x);
                        machine.run_and_provide_input(y).unwrap();
                    } else {
                        machine.provide_input(-1);
                        reading_from_empty[i] = true;
                    }
                }
                RunResult::ProvidingOutput(addr) => {
                    let x = machine.run_and_get_output().unwrap();
                    let y = machine.run_and_get_output().unwrap();

                    if addr == 255 {
                        if mode2 {
                            nat_packet = (x, y);
                            continue;
                        } else {
                            return y;
                        }
                    }

                    buffers[addr as usize].push_back((x, y));
                }
            }
        }

        if run_once && mode2 && reading_from_empty.iter().fold(true, |a, x| a && *x) {
            if nat_packet.1 == last_nat_y {
                return last_nat_y;
            }
            last_nat_y = nat_packet.1;
            buffers[0].push_back(nat_packet);
        }

        run_once = true;
    }
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day23.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = run_network(&tape, false);
    let result1 = run_network(&tape, true);

    println!("{} {}", result0, result1);
}
