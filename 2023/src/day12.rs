use arrayvec::ArrayVec;
use std::fmt::Write;

const MAX_GROUPS: usize = 8;

#[derive(Debug)]
struct Entry {
    operational_mask: u32,
    broken_mask: u32,
    groups: ArrayVec<u8, MAX_GROUPS>,
    size: u8,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut result_0 = 0;
    let result_1 = 0;

    for line in lines {
        let mut halves = line.split_whitespace();

        let mut operational_mask: u32 = 0;
        let mut broken_mask: u32 = 0;
        let mut mask: u32 = 1;
        let mut size: u8 = 0;

        for ch in halves.next().unwrap().chars() {
            if ch == '.' {
                operational_mask |= mask;
            } else if ch == '#' {
                broken_mask |= mask;
            }
            mask <<= 1;
            size += 1;
        }

        let groups = halves
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<ArrayVec<u8, MAX_GROUPS>>();

        result_0 += count_arrangements(Entry {
            operational_mask,
            broken_mask,
            groups,
            size,
        });
    }

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn count_arrangements(entry: Entry) -> u32 {
    let gaps = entry.size + 1 - entry.groups.iter().map(|x| *x + 1).sum::<u8>();
    let mut count = 0;

    iter_gap_distributions(
        gaps,
        entry.groups.len() + 1,
        &mut ArrayVec::new(),
        &mut |dist| {
            let mut offset = 0_u32;
            let mut mask = 0_u32;
            for (&g, &gap) in entry.groups.iter().zip(dist) {
                let group_bits = ((1_u32 << g) - 1) << (offset + gap as u32);
                offset += g as u32 + 1 + gap as u32;
                mask |= group_bits;
            }

            if entry.broken_mask & mask == entry.broken_mask
                && entry.operational_mask & !mask == entry.operational_mask
            {
                count += 1;
            }
        },
    );

    count
}

fn iter_gap_distributions(
    gaps: u8,
    gap_count: usize,
    scratch: &mut ArrayVec<u8, MAX_GROUPS>,
    f: &mut impl FnMut(&[u8]),
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
