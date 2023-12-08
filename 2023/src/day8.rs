#![allow(clippy::char_lit_as_u8)]

use arrayvec::ArrayVec;
use std::fmt::Write;

const IDENT_TABLE_SIZE: usize = 26 * 26 * 26;
const INSTRUCTIONS_SIZE: usize = 512;
const LINES_SIZE: usize = 1024;
const PARALLEL_SIZE: usize = 8;

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

    let result_2 = {
        0
        // let mut par_pos = par_starts.clone();
        // let mut ip = 0;

        // loop {
        //     let right = instructions[ip % instructions.len()];
        //     ip += 1;

        //     for pos in par_pos.iter_mut() {
        //         *pos = data[*pos as usize][right as usize];
        //     }

        //     let mut done = true;
        //     for pos in par_pos.iter() {
        //         if !par_ends.contains(pos) {
        //             done = false;
        //             break;
        //         }
        //     }

        //     if done {
        //         break ip;
        //     }

        //     if ip % 10_000_000 == 0 {
        //         println!("{ip}");
        //     }
        // }
    };

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn ident_to_idx(id: &str) -> usize {
    let bytes = id.as_bytes();
    (bytes[0] as usize - 65) + 26 * (bytes[1] as usize - 65) + 26 * 26 * (bytes[2] as usize - 65)
}
