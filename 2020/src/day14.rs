use std::collections::HashMap;

const MAX_BITS: u64 = 36;

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

#[derive(Debug, Clone, Copy)]
enum MachineMode {
    ValueMask,
    MemoryMask,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            mask_set: 0,
            mask_values: 0,
            memory: HashMap::new(),
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction, mode: MachineMode) {
        match instruction {
            Instruction::SetMask(set, values) => {
                self.mask_set = set;
                self.mask_values = values;
            }
            Instruction::SetMem(addr, val) => match mode {
                MachineMode::ValueMask => self.update_with_value_mask(addr, val),
                MachineMode::MemoryMask => self.update_with_memory_mask(addr, val),
            },
        };
    }

    fn update_with_value_mask(&mut self, addr: u64, val: u64) {
        self.memory
            .insert(addr, val & !self.mask_set | self.mask_values);
    }

    fn update_with_memory_mask(&mut self, mut addr: u64, val: u64) {
        for i in 0..MAX_BITS {
            let bit = 1 << i;
            if self.mask_set & bit == 0 {
                addr &= !bit;
            } else if self.mask_values & bit != 0 {
                addr |= bit;
            }
        }

        let mut write_addrs = Vec::<u64>::new();

        for i in 0..MAX_BITS {
            let bit = 1 << i;
            if self.mask_set & bit == 0 {
                if write_addrs.len() == 0 {
                    write_addrs.push(addr);
                    addr |= bit;
                    write_addrs.push(addr);
                } else {
                    write_addrs.extend(write_addrs.iter().map(|x| x | bit).collect::<Vec<_>>());
                }
            }
        }

        for wa in write_addrs {
            self.memory.insert(wa, val);
        }
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn run_instructions_and_sum_memory(instructions: &[Instruction], mode: MachineMode) -> u64 {
    let mut machine = Machine::new();
    for &ins in instructions {
        machine.run_instruction(ins, mode);
    }
    machine.memory_sum()
}

pub fn main() {
    let instructions = std::fs::read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let part1 = run_instructions_and_sum_memory(&instructions, MachineMode::ValueMask);
    let part2 = run_instructions_and_sum_memory(&instructions, MachineMode::MemoryMask);

    println!("{} {}", part1, part2);
}
