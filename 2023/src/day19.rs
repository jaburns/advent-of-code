use arrayvec::ArrayVec;
use std::{collections::VecDeque, fmt::Write, ops::Range};

const MAX_FUNCTIONS: usize = 543;
const MAX_DATA: usize = 200;
const MAX_EXPRESSIONS: usize = 4;
const REGISTER_COUNT: usize = 4;
const STATE_STACK_INITIAL_CAP: usize = 256;

type Function = ArrayVec<Expression, MAX_EXPRESSIONS>;

#[derive(Debug)]
struct Expression {
    condition: Option<Condition>,
    target: Target,
}

#[derive(Debug)]
enum Condition {
    LessThan(u8, u16),
    GreaterThan(u8, u16),
}

fn reg_idx_for_letter(s: &str) -> u8 {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!(),
    }
}

impl Condition {
    fn parse(s: &str) -> Self {
        if s.contains('<') {
            let (reg, val) = s.split_once('<').unwrap();
            let reg_idx = reg_idx_for_letter(reg);
            Condition::LessThan(reg_idx, val.parse().unwrap())
        } else {
            let (reg, val) = s.split_once('>').unwrap();
            let reg_idx = reg_idx_for_letter(reg);
            Condition::GreaterThan(reg_idx, val.parse().unwrap())
        }
    }
}

#[derive(Debug)]
enum Target {
    Accept,
    Reject,
    Jump(u16),
}

impl Target {
    fn parse(s: &str, index: &HashMap<&str, u16>) -> Self {
        if s == "A" {
            Target::Accept
        } else if s == "R" {
            Target::Reject
        } else {
            Target::Jump(index[s])
        }
    }
}

type HashMap<K, V> =
    std::collections::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut function_index_by_name = HashMap::with_capacity_and_hasher(
        MAX_FUNCTIONS,
        core::hash::BuildHasherDefault::<ahash::AHasher>::default(),
    );
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            break;
        }
        let fn_name = line.split('{').next().unwrap();
        function_index_by_name.insert(fn_name, idx as u16);
    }

    let mut parsing_functions = true;
    let mut functions = ArrayVec::<Function, MAX_FUNCTIONS>::new();
    let mut part_1_data = ArrayVec::<[u16; REGISTER_COUNT], MAX_DATA>::new();

    for line in lines.iter() {
        if line.is_empty() {
            parsing_functions = false;
            continue;
        }
        if parsing_functions {
            let x = line.split('{').nth(1).unwrap();
            let y = x.split('}').next().unwrap();
            let exprs = y.split(',');
            let mut func = Function::new();
            for expr in exprs {
                if expr.contains(':') {
                    let mut parts = expr.split(':');
                    let cond = parts.next().unwrap();
                    let targ = parts.next().unwrap();
                    func.push(Expression {
                        condition: Some(Condition::parse(cond)),
                        target: Target::parse(targ, &function_index_by_name),
                    });
                } else {
                    func.push(Expression {
                        condition: None,
                        target: Target::parse(expr, &function_index_by_name),
                    });
                }
            }
            functions.push(func);
        } else {
            let (_, line) = line.split_once("{x=").unwrap();
            let (x, line) = line.split_once(",m=").unwrap();
            let (m, line) = line.split_once(",a=").unwrap();
            let (a, line) = line.split_once(",s=").unwrap();
            let (s, _) = line.split_once('}').unwrap();

            part_1_data.push([
                x.parse::<u16>().unwrap(),
                m.parse::<u16>().unwrap(),
                a.parse::<u16>().unwrap(),
                s.parse::<u16>().unwrap(),
            ]);
        }
    }

    let start_idx = function_index_by_name["in"];

    let mut sum = 0_u64;
    for dat in part_1_data.iter() {
        if evaluate_single(&functions, start_idx, *dat) {
            sum += dat[0] as u64;
            sum += dat[1] as u64;
            sum += dat[2] as u64;
            sum += dat[3] as u64;
        }
    }
    let result_0 = sum;

    let result_1 = evaluate_ranges(&functions, start_idx);

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn evaluate_single(functions: &[Function], start_idx: u16, data: [u16; REGISTER_COUNT]) -> bool {
    let mut f_ptr = start_idx as usize;
    let mut e_ptr = 0_usize;

    loop {
        let expr = &functions[f_ptr][e_ptr];
        e_ptr += 1;
        if let Some(cond) = &expr.condition {
            match cond {
                Condition::LessThan(reg, val) => {
                    if data[*reg as usize] >= *val {
                        continue;
                    }
                }
                Condition::GreaterThan(reg, val) => {
                    if data[*reg as usize] <= *val {
                        continue;
                    }
                }
            }
        }
        match expr.target {
            Target::Accept => return true,
            Target::Reject => return false,
            Target::Jump(x) => {
                f_ptr = x as usize;
                e_ptr = 0;
            }
        }
    }
}

#[derive(Clone)]
struct RangeEvalState {
    registers: [Range<u16>; REGISTER_COUNT],
    f_ptr: usize,
    e_ptr: usize,
}

fn evaluate_ranges(functions: &[Function], start_idx: u16) -> u64 {
    let mut states = VecDeque::with_capacity(STATE_STACK_INITIAL_CAP);
    let mut accepted_state_count = 0_u64;
    states.push_back(RangeEvalState {
        registers: [1..4001, 1..4001, 1..4001, 1..4001],
        f_ptr: start_idx as usize,
        e_ptr: 0,
    });

    while let Some(mut state) = states.pop_front() {
        let expr = &functions[state.f_ptr][state.e_ptr];
        state.e_ptr += 1;
        if let Some(cond) = &expr.condition {
            match cond {
                Condition::LessThan(reg, val) => {
                    let range = &state.registers[*reg as usize];
                    let pivot = *val;
                    if range.end <= pivot {
                        // whole range passes
                    } else if range.start >= pivot {
                        states.push_back(state);
                        continue;
                    } else {
                        let mut failed = state.clone();
                        failed.registers[*reg as usize].start = pivot;
                        state.registers[*reg as usize].end = pivot;
                        states.push_back(failed);
                    }
                }
                Condition::GreaterThan(reg, val) => {
                    let range = &state.registers[*reg as usize];
                    let pivot = *val + 1;
                    if range.start >= pivot {
                        // whole range passes
                    } else if range.end <= pivot {
                        states.push_back(state);
                        continue;
                    } else {
                        let mut failed = state.clone();
                        failed.registers[*reg as usize].end = pivot;
                        state.registers[*reg as usize].start = pivot;
                        states.push_back(failed);
                    }
                }
            }
        }
        match expr.target {
            Target::Accept => {
                accepted_state_count += state
                    .registers
                    .iter()
                    .map(|x| (x.end - x.start) as u64)
                    .product::<u64>();
            }
            Target::Reject => {}
            Target::Jump(x) => {
                state.f_ptr = x as usize;
                state.e_ptr = 0;
                states.push_back(state);
            }
        }
    }

    accepted_state_count
}
