use glam::{ivec2, IVec2};
use smallvec::SmallVec;
use std::fmt::Write;

type HashMap<K, V> =
    std::collections::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut sum = 0_u64;
    let mut gears = HashMap::<IVec2, SmallVec<[u32; 3]>>::default();

    let empty = ".".repeat(lines[0].len());

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

    let mut gear_sum = 0_u64;
    for vals in gears.into_values() {
        if vals.len() == 2 {
            gear_sum += (vals[0] * vals[1]) as u64;
        }
    }

    write!(out, "{}  {}", sum, gear_sum).unwrap();
}

#[allow(clippy::char_lit_as_u8)]
const DOT: u8 = '.' as u8;
#[allow(clippy::char_lit_as_u8)]
const GEAR: u8 = '*' as u8;

fn sum_part_numbers(
    line_idx: i32,
    line: &str,
    prev: &str,
    next: &str,
    gear_cache: &mut HashMap<IVec2, SmallVec<[u32; 3]>>,
) -> u64 {
    let mut sum = 0;

    let bytes = line.as_bytes();
    let bytes_p = prev.as_bytes();
    let bytes_n = next.as_bytes();

    let mut digit_buf: [u8; 16] = Default::default();
    let mut digit_buf_idx: usize = 0;
    let mut valid = false;
    let mut write_gears = SmallVec::<[IVec2; 8]>::default();

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
                    let list = gear_cache.entry(gear).or_default();
                    list.push(parsed as u32);
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
            let list = gear_cache.entry(gear).or_default();
            list.push(parsed as u32);
        }
    }

    sum
}
