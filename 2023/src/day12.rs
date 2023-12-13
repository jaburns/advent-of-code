use arrayvec::ArrayVec;
use std::fmt::Write;

const MAX_GROUPS: usize = 64;

#[derive(Debug)]
struct Entry {}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut result_0 = 0;
    let mut result_1 = 0;

    for line in lines {
        let mut halves = line.split_whitespace();

        let mut operational_mask_0: u128 = 0;
        let mut operational_mask_1: u128 = 0;
        let mut broken_mask_0: u128 = 0;
        let mut broken_mask_1: u128 = 0;
        let mut mask: u128 = 1;

        let pattern = halves.next().unwrap();
        let size = pattern.len();

        for ch in pattern.chars() {
            if ch == '.' {
                operational_mask_0 |= mask;
                operational_mask_1 |= mask;
            } else if ch == '#' {
                broken_mask_0 |= mask;
                broken_mask_1 |= mask;
            }
            mask <<= 1;
        }
        for _ in 0..4 {
            for ch in pattern.chars() {
                if ch == '.' {
                    operational_mask_1 |= mask;
                } else if ch == '#' {
                    broken_mask_1 |= mask;
                }
                mask <<= 1;
            }
        }

        let digits = halves.next().unwrap();

        let groups_0 = digits
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<ArrayVec<usize, MAX_GROUPS>>();

        result_0 += count_arrangements(operational_mask_0, broken_mask_0, &groups_0, size);

        let mut groups_1 = ArrayVec::<usize, MAX_GROUPS>::new();
        for _ in 0..5 {
            groups_1.extend(groups_0.iter().cloned());
        }

        println!(
            "{:b} {:b} {:?}",
            operational_mask_1, broken_mask_1, groups_1
        );

        result_1 += count_arrangements(operational_mask_1, broken_mask_1, &groups_1, size);
    }

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn count_arrangements(
    operational_mask: u128,
    broken_mask: u128,
    groups: &[usize],
    size: usize,
) -> usize {
    let gaps = size + 1 - groups.iter().map(|x| *x + 1).sum::<usize>();
    let mut count = 0;

    iter_gap_distributions(gaps, groups.len() + 1, &mut ArrayVec::new(), &mut |dist| {
        let mut offset = 0_u128;
        let mut mask = 0_u128;
        for (&g, &gap) in groups.iter().zip(dist) {
            let group_bits = ((1_u128 << g) - 1) << (offset + gap as u128);
            offset += g as u128 + 1 + gap as u128;
            mask |= group_bits;
        }
        if broken_mask & mask == broken_mask && operational_mask & !mask == operational_mask {
            count += 1;
        }
    });

    count
}

fn iter_gap_distributions(
    gaps: usize,
    gap_count: usize,
    scratch: &mut ArrayVec<usize, MAX_GROUPS>,
    f: &mut impl FnMut(&[usize]),
) {
    if gap_count == 1 {
        scratch.push(gaps);
        f(scratch);
        scratch.pop();
    } else {
        for i in 0..=gaps {
            scratch.push(i);
            iter_gap_distributions(gaps - i, gap_count - 1, scratch, f);
            scratch.pop();
        }
    }
}
