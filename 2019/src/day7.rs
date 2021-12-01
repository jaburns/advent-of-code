use crate::intcode::vm::IntCodeMachine;
use permutohedron::Heap;

fn run_forward_amplifier_circuit(tape: &Vec<i64>, phase_seq: &[i64; 5]) -> i64 {
    let a = IntCodeMachine::run_all(&tape, &[phase_seq[0], 0])
        .pop()
        .unwrap();
    let b = IntCodeMachine::run_all(&tape, &[phase_seq[1], a])
        .pop()
        .unwrap();
    let c = IntCodeMachine::run_all(&tape, &[phase_seq[2], b])
        .pop()
        .unwrap();
    let d = IntCodeMachine::run_all(&tape, &[phase_seq[3], c])
        .pop()
        .unwrap();
    let e = IntCodeMachine::run_all(&tape, &[phase_seq[4], d])
        .pop()
        .unwrap();
    e
}

fn run_feedback_amplifier_circuit(tape: &Vec<i64>, phase_seq: &[i64; 5]) -> i64 {
    let mut signal = 0i64;
    let mut machine_index = 0usize;
    let mut machines = [
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
    ];

    for i in 0..machines.len() {
        machines[i].run_and_provide_input(phase_seq[i]).unwrap();
    }

    while let Ok(_) = (|| -> Result<(), ()> {
        machines[machine_index].run_and_provide_input(signal)?;
        signal = machines[machine_index].run_and_get_output()?;
        machine_index = (machine_index + 1) % 5;
        Ok(())
    })() {}

    signal
}

fn find_best_thruster_signal(
    tape: &Vec<i64>,
    phases: &[i64; 5],
    run: fn(&Vec<i64>, &[i64; 5]) -> i64,
) -> i64 {
    let mut mut_phases = [0i64; 5];
    mut_phases.copy_from_slice(phases);

    let mut heap = Heap::new(&mut mut_phases);
    let mut best_signal = 0i64;
    while let Some(perm) = heap.next_permutation() {
        let new_signal = run(tape, perm);
        if new_signal > best_signal {
            best_signal = new_signal
        };
    }
    best_signal
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day7.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = find_best_thruster_signal(&tape, &[0, 1, 2, 3, 4], run_forward_amplifier_circuit);
    let result1 =
        find_best_thruster_signal(&tape, &[5, 6, 7, 8, 9], run_feedback_amplifier_circuit);

    println!("{} {}", result0, result1);
}
