#![allow(clippy::char_lit_as_u8)]

use arrayvec::ArrayVec;
use std::fmt::Write;

type HashMap<K, V> =
    std::collections::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

const IDENT_TABLE_SIZE: usize = 26 * 26 * 26;
const INSTRUCTIONS_SIZE: usize = 512;
const LINES_SIZE: usize = 1024;
const PARALLEL_SIZE: usize = 8;

const RUN_VALIDATION: bool = false;

type Part2Fn = fn(u16, &[[u16; 2]], &[bool], &[u16]) -> u64;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let instructions = lines[0]
        .chars()
        .map(|x| x == 'R')
        .collect::<ArrayVec<bool, INSTRUCTIONS_SIZE>>();

    let mut lookup = [0_u16; IDENT_TABLE_SIZE];
    let mut par_starts = ArrayVec::<u16, PARALLEL_SIZE>::new();
    let mut par_ends = ArrayVec::<u16, PARALLEL_SIZE>::new();
    let mut data = [[0_u16, 0_u16]; LINES_SIZE];

    for (i, line) in lines[2..].iter().enumerate() {
        let i = i as u16;

        let idx = ident_to_idx(line);
        lookup[idx] = i;

        if line.as_bytes()[2] == 'A' as u8 {
            par_starts.push(i);
        } else if line.as_bytes()[2] == 'Z' as u8 {
            par_ends.push(i);
        }
    }
    for (line, entry) in lines[2..].iter().zip(data.iter_mut()) {
        let left = lookup[ident_to_idx(&line[7..10])];
        let right = lookup[ident_to_idx(&line[12..15])];
        *entry = [left, right];
    }

    let result_1 = {
        let finish = lookup[ident_to_idx("ZZZ")];
        let mut pos = lookup[ident_to_idx("AAA")];
        let mut ip = 0;

        loop {
            let right = instructions[ip % instructions.len()];
            ip += 1;

            pos = data[pos as usize][right as usize];
            if pos == finish {
                break ip;
            }
        }
    };

    const PART_2_FN: Part2Fn = if RUN_VALIDATION {
        validate_loop_and_find_length
    } else {
        assume_valid_loop_find_length
    };

    let result_2 = par_starts
        .iter()
        .map(|start| PART_2_FN(*start, &data, &instructions, &par_ends))
        .reduce(lcm)
        .unwrap();

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn validate_loop_and_find_length(
    start: u16,
    data: &[[u16; 2]],
    instructions: &[bool],
    par_ends: &[u16],
) -> u64 {
    let mut pos = start;
    let mut ip = 0;
    let mut visited = HashMap::default();
    let mut z = 0;

    loop {
        let ipm = ip % instructions.len();

        if par_ends.contains(&pos) {
            z = ip;
        }

        if let Some(loop_start) = visited.get(&(ipm, pos)) {
            // The data is setup such that this is true, which makes the problem solvable by computing LCM.
            assert_eq!(ip - z, *loop_start);
            return z as u64;
        }
        visited.insert((ipm, pos), ip);

        let right = instructions[ipm];
        ip += 1;
        pos = data[pos as usize][right as usize];
    }
}

fn assume_valid_loop_find_length(
    start: u16,
    data: &[[u16; 2]],
    instructions: &[bool],
    par_ends: &[u16],
) -> u64 {
    let mut pos = start;
    let mut ip = 0;

    loop {
        let right = instructions[ip % instructions.len()];
        ip += 1;

        pos = data[pos as usize][right as usize];
        if par_ends.contains(&pos) {
            break ip as u64;
        }
    }
}

fn ident_to_idx(id: &str) -> usize {
    let bytes = id.as_bytes();
    (bytes[0] as usize - 65) + 26 * (bytes[1] as usize - 65) + 26 * 26 * (bytes[2] as usize - 65)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
