use crate::intcode::vm::{IntCodeMachine, RunResult};

fn try_inventory_combo(tape: &[i64], combo: u16) -> Option<String> {
    let mut machine = IntCodeMachine::new(&tape);

    let mut collection_index = 0usize;

    let mut run = vec![
        "north",
        "take tambourine",
        "east",
        "take astrolabe",
        "south",
        "take shell",
        "north",
        "east",
        "north",
        "take klein bottle",
        "north",
        "take easter egg",
        "south",
        "south",
        "west",
        "west",
        "south",
        "south",
        "south",
        "take hypercube",
        "north",
        "north",
        "west",
        "take dark matter",
        "west",
        "north",
        "west",
        "inv",
        "take coin",
        "south",
    ]
    .iter()
    .map(|x| String::from(*x))
    .collect::<Vec<_>>();

    let items = vec![
        "coin",
        "dark matter",
        "hypercube",
        "tambourine",
        "shell",
        "astrolabe",
        "klein bottle",
        "easter egg",
    ];

    let mut mask = 1u16;
    for i in 0..8 {
        if mask & combo != 0 {
            run.push(format!("drop {}", items[i]));
        }
        mask <<= 1;
    }

    run.push(String::from("south"));

    let mut result = String::new();

    loop {
        match machine.run() {
            RunResult::Halted => break,
            RunResult::ProvidingOutput(x) => result.push(x as u8 as char),
            RunResult::RequiresInput => {
                if collection_index >= run.len() {
                    break;
                }

                let line = format!("{}\n", run[collection_index]);
                collection_index += 1;
                let in_line = line
                    .chars()
                    .map(|x| x as i64)
                    .filter(|x| *x != 13)
                    .collect::<Vec<_>>();

                machine.provide_input(in_line[0]);
                for i in 1..in_line.len() {
                    machine.run_and_provide_input(in_line[i]).unwrap();
                }
            }
        }
    }

    if result.contains("Alert!") {
        None
    } else {
        Some(result)
    }
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day25.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    for i in 0..256 {
        let result = try_inventory_combo(&tape, i);
        if result.is_some() {
            println!("{}", result.unwrap());
            break;
        }
    }
}
