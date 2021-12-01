pub mod defs {
    pub const I_HALT: i64 = 99;
    pub const I_ADD: i64 = 01;
    pub const I_MUL: i64 = 02;
    pub const I_IN: i64 = 03;
    pub const I_OUT: i64 = 04;
    pub const I_JNZ: i64 = 05;
    pub const I_JZ: i64 = 06;
    pub const I_LESS: i64 = 07;
    pub const I_CMP: i64 = 08;
    pub const I_RBA: i64 = 09;
}

pub mod vm {
    use crate::intcode::defs::*;

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum RunResult {
        RequiresInput,
        ProvidingOutput(i64),
        Halted,
    }

    #[derive(Debug, Clone)]
    pub struct IntCodeMachine {
        tape: Vec<i64>,
        ip: usize,
        relative_base: i64,
        last_result: Option<RunResult>,
        input_address: usize,
    }

    impl IntCodeMachine {
        pub fn new(init_tape: &[i64]) -> IntCodeMachine {
            IntCodeMachine {
                tape: Vec::from(init_tape),
                ip: 0,
                relative_base: 0,
                last_result: None,
                input_address: 0,
            }
        }

        pub fn provide_input(&mut self, input: i64) {
            self.write_to_tape(self.input_address, input);
        }

        pub fn poke(&mut self, addr: usize, val: i64) {
            self.tape[addr] = val;
        }

        pub fn run(&mut self) -> RunResult {
            if self.last_result == Some(RunResult::Halted) {
                panic!("Cannot continue from halted state");
            }

            loop {
                match self.tape[self.ip] % 100 {
                    I_HALT => {
                        self.last_result = Some(RunResult::Halted);
                        break;
                    }

                    I_ADD => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.get_out_arg(2);
                        self.write_to_tape(arg2 as usize, arg0 + arg1);
                        self.ip += 4
                    }

                    I_MUL => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.get_out_arg(2);
                        self.write_to_tape(arg2 as usize, arg0 * arg1);
                        self.ip += 4
                    }

                    I_IN => {
                        self.input_address = self.get_out_arg(0);
                        self.last_result = Some(RunResult::RequiresInput);
                        self.ip += 2;
                        break;
                    }

                    I_OUT => {
                        let arg0 = self.get_arg(0);
                        self.last_result = Some(RunResult::ProvidingOutput(arg0));
                        self.ip += 2;
                        break;
                    }

                    I_JNZ => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        if arg0 != 0 {
                            self.ip = arg1 as usize
                        } else {
                            self.ip += 3
                        }
                    }

                    I_JZ => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        if arg0 == 0 {
                            self.ip = arg1 as usize
                        } else {
                            self.ip += 3
                        }
                    }

                    I_LESS => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.get_out_arg(2);
                        self.write_to_tape(arg2 as usize, if arg0 < arg1 { 1 } else { 0 });
                        self.ip += 4
                    }

                    I_CMP => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.get_out_arg(2);
                        self.write_to_tape(arg2 as usize, if arg0 == arg1 { 1 } else { 0 });
                        self.ip += 4
                    }

                    I_RBA => {
                        let arg0 = self.get_arg(0);
                        self.relative_base += arg0;
                        self.ip += 2
                    }

                    _ => panic!(
                        "ABORTED: Encountered unknown opcode {} at location {}",
                        self.tape[self.ip], self.ip
                    ),
                }
            }

            self.last_result.unwrap()
        }

        fn write_to_tape(&mut self, address: usize, value: i64) {
            while self.tape.len() < address + 1 {
                self.tape.push(0);
            }
            self.tape[address] = value;
        }

        fn read_from_tape(&self, address: usize) -> i64 {
            if address < self.tape.len() {
                self.tape[address]
            } else {
                0
            }
        }

        fn get_arg_digit(&self, arg: usize) -> i64 {
            let mut arg_digit = self.tape[self.ip] / 100;
            for _ in 0..arg {
                arg_digit /= 10;
            }
            arg_digit % 10
        }

        fn get_arg(&self, arg: usize) -> i64 {
            let arg_digit = self.get_arg_digit(arg);

            if arg_digit == 1 {
                self.read_from_tape(self.ip + arg + 1)
            } else if arg_digit == 2 {
                self.read_from_tape((self.tape[self.ip + arg + 1] + self.relative_base) as usize)
            } else {
                self.read_from_tape(self.tape[self.ip + arg + 1] as usize)
            }
        }

        fn get_out_arg(&self, arg: usize) -> usize {
            let arg_digit = self.get_arg_digit(arg);

            if arg_digit == 2 {
                (self.read_from_tape(self.ip + arg + 1) + self.relative_base) as usize
            } else {
                self.read_from_tape(self.ip + arg + 1) as usize
            }
        }

        pub fn run_all(tape: &[i64], inputs: &[i64]) -> Vec<i64> {
            let mut vm = IntCodeMachine::new(tape);
            let mut input_ptr = 0usize;
            let mut outputs = Vec::<i64>::new();

            loop {
                match vm.run() {
                    RunResult::Halted => break,
                    RunResult::ProvidingOutput(x) => outputs.push(x),
                    RunResult::RequiresInput => {
                        vm.provide_input(inputs[input_ptr]);
                        input_ptr += 1
                    }
                }
            }

            outputs
        }

        pub fn run_and_provide_input(&mut self, input: i64) -> Result<(), ()> {
            match self.run() {
                RunResult::Halted => Err(()),
                RunResult::ProvidingOutput(_) => panic!("Expected input state, encountered output"),
                RunResult::RequiresInput => {
                    self.provide_input(input);
                    Ok(())
                }
            }
        }

        pub fn run_and_get_output(&mut self) -> Result<i64, ()> {
            match self.run() {
                RunResult::Halted => Err(()),
                RunResult::RequiresInput => panic!("Expected output state, encountered input"),
                RunResult::ProvidingOutput(x) => Ok(x),
            }
        }
    }
}

pub mod assembler {
    use crate::intcode::defs::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct InstructionDef {
        pub name: &'static str,
        pub opcode: i64,
        pub inargs: u64,
        pub outargs: u64,
    }

    #[derive(Debug)]
    struct ParsedInstruction {
        pub size: u64,
        pub def: &'static InstructionDef,
        pub words: Vec<String>,
        pub internal_labels: Vec<(String, u64)>,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    enum AddressMode {
        Pointer,
        Immediate,
        Relative,
    }

    const INSTRUCTIONS: [InstructionDef; 12] = [
        InstructionDef {
            name: "halt",
            opcode: I_HALT,
            inargs: 0,
            outargs: 0,
        },
        InstructionDef {
            name: "add",
            opcode: I_ADD,
            inargs: 2,
            outargs: 1,
        },
        InstructionDef {
            name: "mul",
            opcode: I_MUL,
            inargs: 2,
            outargs: 1,
        },
        InstructionDef {
            name: "in",
            opcode: I_IN,
            inargs: 0,
            outargs: 1,
        },
        InstructionDef {
            name: "out",
            opcode: I_OUT,
            inargs: 1,
            outargs: 0,
        },
        InstructionDef {
            name: "jnz",
            opcode: I_JNZ,
            inargs: 2,
            outargs: 0,
        },
        InstructionDef {
            name: "jz",
            opcode: I_JZ,
            inargs: 2,
            outargs: 0,
        },
        InstructionDef {
            name: "less",
            opcode: I_LESS,
            inargs: 2,
            outargs: 1,
        },
        InstructionDef {
            name: "cmp",
            opcode: I_CMP,
            inargs: 2,
            outargs: 1,
        },
        InstructionDef {
            name: "rba",
            opcode: I_RBA,
            inargs: 1,
            outargs: 0,
        },
        InstructionDef {
            name: "dd",
            opcode: -1,
            inargs: 0,
            outargs: 0,
        },
        InstructionDef {
            name: "fill",
            opcode: -1,
            inargs: 0,
            outargs: 0,
        },
    ];

    fn parse_label(labels: &HashMap<String, i64>, arg: &str) -> (i64, AddressMode) {
        if arg.starts_with("$") {
            (0, AddressMode::Pointer)
        } else if arg.starts_with("&") {
            (labels[&arg[1..]], AddressMode::Immediate)
        } else if arg.starts_with("^") {
            (arg[1..].parse::<i64>().unwrap(), AddressMode::Relative)
        } else {
            (labels[arg], AddressMode::Pointer)
        }
    }

    fn get_address_mode_flag(word_i: usize, mode: AddressMode) -> i64 {
        match mode {
            AddressMode::Pointer => 0,
            AddressMode::Immediate => 10i64.pow(1 + word_i as u32),
            AddressMode::Relative => 2 * 10i64.pow(1 + word_i as u32),
        }
    }

    fn assemble_parsed_instruction(
        labels: &HashMap<String, i64>,
        parsed: &ParsedInstruction,
    ) -> Vec<i64> {
        if parsed.def.opcode < 0 {
            let arg = &parsed.words[1];
            let arg_val = match arg.parse::<i64>() {
                Ok(x) => x,
                Err(_) => {
                    let (arg_val, _) = parse_label(labels, arg);
                    arg_val
                }
            };

            return match parsed.def.name {
                "dd" => vec![arg_val],
                "fill" => vec![arg_val; parsed.size as usize],
                _ => panic!(),
            };
        }

        let mut result = Vec::<i64>::new();
        let mut word_i = 1usize;
        let mut op_flags = 0i64;

        for _ in 0..parsed.def.inargs {
            let arg = &parsed.words[word_i];

            result.push(match arg.parse::<i64>() {
                Ok(x) => {
                    op_flags += get_address_mode_flag(word_i, AddressMode::Immediate);
                    x
                }
                Err(_) => {
                    let (arg_val, mode) = parse_label(labels, arg);
                    op_flags += get_address_mode_flag(word_i, mode);
                    arg_val
                }
            });

            word_i += 1
        }

        for _ in 0..parsed.def.outargs {
            let (arg_val, mode) = parse_label(labels, &parsed.words[word_i]);
            if mode == AddressMode::Immediate {
                panic!("Cannot have immediate-mode out arg");
            }
            op_flags += get_address_mode_flag(word_i, mode);
            result.push(arg_val);
            word_i += 1
        }

        result.insert(0, op_flags + parsed.def.opcode);

        result
    }

    fn parse_instruction_text(text: &str) -> ParsedInstruction {
        let no_commas = String::from(text).replace(",", " ");
        let words: Vec<String> = no_commas
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();

        let ins: &InstructionDef = INSTRUCTIONS.iter().find(|x| x.name == words[0]).unwrap();

        let size = match words[0].as_str() {
            "dd" => 1,
            "fill" => words[2].parse::<u64>().unwrap(),
            _ => 1 + ins.inargs + ins.outargs,
        };

        let mut internal_labels = Vec::<(String, u64)>::new();
        for i in 1..words.len() {
            if words[i].starts_with("$") {
                internal_labels.push((String::from(&words[i][1..]), i as u64));
            }
        }

        ParsedInstruction {
            size: size,
            def: ins,
            words: words,
            internal_labels: internal_labels,
        }
    }

    pub fn assemble(path: &str, debug: bool) -> Vec<i64> {
        let source: Vec<String> = std::fs::read_to_string(path)
            .unwrap()
            .replace(":", ":\n")
            .lines()
            .map(|x| String::from(x.trim()))
            .filter(|x| x.len() > 0 && !x.starts_with(";"))
            .collect();

        let mut address_labels = HashMap::<String, i64>::new();
        let mut cur_address = 0u64;
        let mut instructions = Vec::<ParsedInstruction>::new();

        for line in source {
            if line.ends_with(":") {
                address_labels.insert(String::from(line.replace(":", "")), cur_address as i64);
            } else {
                let parsed = parse_instruction_text(&line);

                for label in &parsed.internal_labels {
                    address_labels.insert(label.0.clone(), (cur_address + label.1) as i64);
                }

                cur_address += parsed.size;
                instructions.push(parsed);
            }
        }

        let mut output = Vec::<i64>::new();

        for ins in instructions {
            let addr = output.len();
            let asm = assemble_parsed_instruction(&address_labels, &ins);
            if debug {
                println!("{} : {:?} : {:?}", addr, ins.words, asm)
            };
            output.extend(asm);
        }

        if debug {
            println!("");
            println!("{:?}", output);
            println!("");
        }

        output
    }
}
