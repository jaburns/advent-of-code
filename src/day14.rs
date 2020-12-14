use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SetMask(u64, u64),
    SetMem(u64, u64),
}

impl Instruction {
    pub fn parse(s: &str) -> Instruction {
        if s.starts_with("mask") {
            let bits = s.replace("mask = ", "").chars().collect::<Vec<_>>();
            let len = bits.len();
            let mut set = 0u64;
            let mut values = 0u64;

            for i in 0..len {
                let bit = bits[len - 1 - i];
                if bit != 'X' {
                    set |= 1 << i;
                }
                if bit == '1' {
                    values |= 1 << i;
                }
            }

            Instruction::SetMask(set, values)
        } else {
            let args = s
                .replace("mem[", "")
                .split("] = ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            Instruction::SetMem(args[0], args[1])
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    mask_set: u64,
    mask_values: u64,
    memory: HashMap<u64, u64>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            mask_set: 0,
            mask_values: 0,
            memory: HashMap::new(),
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(set, values) => {
                self.mask_set = set;
                self.mask_values = values;
            }
            Instruction::SetMem(addr, val) => {
                self.memory
                    .insert(addr, val & !self.mask_set | self.mask_values);
            }
        };
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn main() {
    let instructions = std::fs::read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let part1 = {
        let mut machine = Machine::new();
        for ins in instructions {
            machine.run_instruction(ins);
        }
        machine.memory_sum()
    };

    println!("{}", part1);
}
