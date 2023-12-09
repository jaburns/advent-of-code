use arrayvec::ArrayVec;
use glam::{ivec2, IVec2};
use std::{fmt::Write, mem::swap};

const HISTORY_BUFFER_CAP: usize = 32;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let results = lines
        .iter()
        .map(|x| parse_and_extrapolate::<false>(x))
        .sum::<IVec2>();
    write!(out, "{}  {}", results.x, results.y).unwrap();
}

fn parse_and_extrapolate<const REVERSE: bool>(line: &str) -> IVec2 {
    let mut buffer_0 = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<ArrayVec<i32, HISTORY_BUFFER_CAP>>();
    let mut buffer_1 = ArrayVec::<i32, HISTORY_BUFFER_CAP>::new();

    let mut acc = ArrayVec::<IVec2, HISTORY_BUFFER_CAP>::new();
    acc.push(ivec2(*buffer_0.last().unwrap(), buffer_0[0]));

    let mut cur = &mut buffer_0;
    let mut next = &mut buffer_1;

    loop {
        let mut all_zero = true;
        let mut diff = 0;
        for &[a, b] in cur.array_windows::<2>() {
            diff = b - a;
            all_zero &= diff == 0;
            next.push(diff);
        }
        if all_zero {
            break;
        }
        acc.push(ivec2(diff, next[0]));
        swap(&mut cur, &mut next);
        next.clear();
    }

    let mut ret = IVec2::ZERO;
    for v in acc.iter().rev() {
        ret.x += v.x;
        ret.y = v.y - ret.y;
    }

    ret
}
