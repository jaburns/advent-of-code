use arrayvec::{ArrayString, ArrayVec};
use glam::{ivec2, IVec2};
use std::fmt::Write;

const LINE_LEN: usize = 140;
const LINE_COUNT: usize = 140;

struct Gears {
    set: [(u16, u16); LINE_LEN * LINE_COUNT],
    sum: u64,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut sum = 0_u64;
    let mut gears: Gears = unsafe { std::mem::zeroed() };

    let mut empty = ArrayString::<LINE_LEN>::new();
    for _ in 0..lines[0].len() {
        empty.push('.');
    }

    sum += sum_part_numbers(0, lines[0], &empty, lines[1], &mut gears);
    sum += sum_part_numbers(
        lines.len() as i32 - 1,
        lines[lines.len() - 1],
        lines[lines.len() - 2],
        &empty,
        &mut gears,
    );
    for i in 1..(lines.len() - 1) {
        sum += sum_part_numbers(i as i32, lines[i], lines[i - 1], lines[i + 1], &mut gears);
    }

    write!(out, "{}  {}", sum, gears.sum).unwrap();
}

#[allow(clippy::char_lit_as_u8)]
const DOT: u8 = '.' as u8;
#[allow(clippy::char_lit_as_u8)]
const GEAR: u8 = '*' as u8;

fn sum_part_numbers(line_idx: i32, line: &str, prev: &str, next: &str, gears: &mut Gears) -> u64 {
    let mut sum = 0;

    let bytes = line.as_bytes();
    let bytes_p = prev.as_bytes();
    let bytes_n = next.as_bytes();

    let mut digit_buf: [u8; 16] = Default::default();
    let mut digit_buf_idx: usize = 0;
    let mut valid = false;
    let mut write_gears = ArrayVec::<IVec2, 8>::default();

    for (i, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            if digit_buf_idx == 0
                && i > 0
                && (bytes[i - 1] != DOT || bytes_p[i - 1] != DOT || bytes_n[i - 1] != DOT)
            {
                if bytes[i - 1] == GEAR {
                    write_gears.push(ivec2(i as i32 - 1, line_idx));
                }
                if bytes_p[i - 1] == GEAR {
                    write_gears.push(ivec2(i as i32 - 1, line_idx - 1));
                }
                if bytes_n[i - 1] == GEAR {
                    write_gears.push(ivec2(i as i32 - 1, line_idx + 1));
                }
                valid = true;
            }
            if bytes_p[i] != DOT || bytes_n[i] != DOT {
                if bytes_p[i] == GEAR {
                    write_gears.push(ivec2(i as i32, line_idx - 1));
                }
                if bytes_n[i] == GEAR {
                    write_gears.push(ivec2(i as i32, line_idx + 1));
                }
                valid = true;
            }
            digit_buf[digit_buf_idx] = ch as u8;
            digit_buf_idx += 1;
        } else if digit_buf_idx > 0 {
            if bytes[i] != DOT || bytes_p[i] != DOT || bytes_n[i] != DOT {
                if bytes[i] == GEAR {
                    write_gears.push(ivec2(i as i32, line_idx));
                }
                if bytes_p[i] == GEAR {
                    write_gears.push(ivec2(i as i32, line_idx - 1));
                }
                if bytes_n[i] == GEAR {
                    write_gears.push(ivec2(i as i32, line_idx + 1));
                }
                valid = true;
            }
            if valid {
                let parsed = std::str::from_utf8(&digit_buf[0..digit_buf_idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                sum += parsed;
                for gear in write_gears.drain(0..) {
                    let entry = &mut gears.set[gear.x as usize + LINE_LEN * gear.y as usize];
                    entry.0 += 1;
                    if entry.0 == 2 {
                        gears.sum += entry.1 as u64 * parsed;
                    } else {
                        entry.1 = parsed as u16;
                    }
                }
            }
            digit_buf_idx = 0;
            valid = false;
        }
    }
    if digit_buf_idx > 0 && valid {
        let parsed = std::str::from_utf8(&digit_buf[0..digit_buf_idx])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        sum += parsed;
        for gear in write_gears.drain(0..) {
            let entry = &mut gears.set[gear.x as usize + LINE_LEN * gear.y as usize];
            entry.0 += 1;
            if entry.0 == 2 {
                gears.sum += entry.1 as u64 * parsed;
            } else {
                entry.1 = parsed as u16;
            }
        }
    }

    sum
}
