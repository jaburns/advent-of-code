use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    arg: i32,
}

impl Instruction {
    pub fn parse(asm: &str) -> Self {
        let parts = asm.split(" ").collect::<Vec<_>>();

        Self {
            kind: match parts[0] {
                "nop" => InstructionKind::Nop,
                "acc" => InstructionKind::Acc,
                "jmp" => InstructionKind::Jmp,
                _ => panic!(),
            },
            arg: parts[1].parse::<i32>().unwrap(),
        }
    }

    pub fn swap_kind(&mut self, a: InstructionKind, b: InstructionKind) {
        if self.kind == a {
            self.kind = b;
        } else if self.kind == b {
            self.kind = a;
        }
    }
}

#[derive(Debug)]
struct VmState {
    visited_instructions: HashSet<i32>,
    instruction_ptr: i32,
    accumulator: i32,
}

impl VmState {
    pub fn new() -> Self {
        Self {
            visited_instructions: HashSet::new(),
            instruction_ptr: 0,
            accumulator: 0,
        }
    }

    pub fn step_with_program(&mut self, program: &[Instruction]) {
        self.visited_instructions.insert(self.instruction_ptr);

        let instruction = &program[self.instruction_ptr as usize];
        match instruction.kind {
            InstructionKind::Nop => {
                self.instruction_ptr += 1;
            }
            InstructionKind::Acc => {
                self.accumulator += instruction.arg;
                self.instruction_ptr += 1;
            }
            InstructionKind::Jmp => {
                self.instruction_ptr += instruction.arg;
            }
        };
    }

    pub fn has_visited(&self, instruction_addr: i32) -> bool {
        self.visited_instructions.contains(&instruction_addr)
    }
}

fn find_loop_or_termination_and_get_vm(program: &[Instruction]) -> (bool, VmState) {
    let mut vm = VmState::new();

    loop {
        if vm.instruction_ptr >= program.len() as i32 {
            return (true, vm);
        }

        vm.step_with_program(&program);

        if vm.has_visited(vm.instruction_ptr) {
            return (false, vm);
        }
    }
}

fn find_termination_patch_and_get_vm(
    mut program: Vec<Instruction>,
    candidate_swaps: &[i32],
) -> VmState {
    for &swap_addr in candidate_swaps {
        if program[swap_addr as usize].kind == InstructionKind::Acc {
            continue;
        }

        program[swap_addr as usize].swap_kind(InstructionKind::Jmp, InstructionKind::Nop);
        let (terminates, vm) = find_loop_or_termination_and_get_vm(&program);
        program[swap_addr as usize].swap_kind(InstructionKind::Jmp, InstructionKind::Nop);

        if terminates {
            return vm;
        }
    }

    panic!()
}

pub fn main() {
    let program: Vec<_> = std::fs::read_to_string("data/day8.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .collect();

    let (_, part1_vm) = find_loop_or_termination_and_get_vm(&program);
    let part1 = part1_vm.accumulator;
    let candidate_swaps = part1_vm
        .visited_instructions
        .into_iter()
        .collect::<Vec<i32>>();

    let part2_vm = find_termination_patch_and_get_vm(program, candidate_swaps.as_slice());
    let part2 = part2_vm.accumulator;

    println!("{} {}", part1, part2);
}
