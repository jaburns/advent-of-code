use arrayvec::ArrayVec;
use std::{fmt::Write, mem::zeroed};

const HISTORY_BUFFER_CAP: usize = 32;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let (result_1, result_2) = lines
        .iter()
        .map(|x| parse_and_extrapolate::<false>(x))
        .reduce(|(x0, y0), (x1, y1)| (x0 + x1, y0 + y1))
        .unwrap();
    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn parse_and_extrapolate<const REVERSE: bool>(line: &str) -> (i32, i32) {
    let mut diffs: [ArrayVec<i32, HISTORY_BUFFER_CAP>; HISTORY_BUFFER_CAP] = unsafe { zeroed() };

    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_into(&mut diffs[0]);

    let mut depth = 0_i32;
    loop {
        depth += 1;
        let (upper, lower) = diffs.split_at_mut(depth as usize);
        let prev = upper.last().unwrap();
        let cur = lower.first_mut().unwrap();

        let mut all_zero = true;
        for &[a, b] in prev.array_windows::<2>() {
            let diff = b - a;
            all_zero &= diff == 0;
            cur.push(diff);
        }

        if all_zero {
            break;
        }
    }

    depth -= 1;

    let mut future = 0_i32;
    let mut past = 0_i32;

    while depth >= 0 {
        future += *diffs[depth as usize].last().unwrap();
        past = *diffs[depth as usize].first().unwrap() - past;
        depth -= 1;
    }

    (future, past)
}
