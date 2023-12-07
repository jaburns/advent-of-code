use arrayvec::ArrayVec;
use std::{fmt::Write, mem::swap, ops::Range};

const SEEDS_COUNT: usize = 32;
const SEED_RANGES_COUNT: usize = 128;
const MAPS_COUNT: usize = 8;
const MAP_SIZE: usize = 64;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let seeds = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<ArrayVec<i64, SEEDS_COUNT>>();

    let seed_ranges_1 = seeds
        .iter()
        .map(|&x| x..(x + 1))
        .collect::<ArrayVec<Range<i64>, SEED_RANGES_COUNT>>();

    let seed_ranges_2 = seeds
        .array_chunks::<2>()
        .map(|&[a, b]| a..(a + b))
        .collect::<ArrayVec<Range<i64>, SEED_RANGES_COUNT>>();

    let maps = get_maps(&lines[3..]);
    let result_1 = run_ranges_through_maps(&maps, seed_ranges_1);
    let result_2 = run_ranges_through_maps(&maps, seed_ranges_2);

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn get_maps(lines: &[&str]) -> ArrayVec<ArrayVec<(Range<i64>, i64), MAP_SIZE>, MAPS_COUNT> {
    let mut maps = ArrayVec::new();
    let mut skip = 0;
    let mut cur_map = ArrayVec::new();

    for line in lines.iter() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if line.is_empty() {
            skip = 1;
            maps.push(cur_map);
            cur_map = ArrayVec::new();
            continue;
        }
        let mut chunks = line.split_whitespace();
        let a = chunks.next().unwrap().parse::<i64>().unwrap();
        let b = chunks.next().unwrap().parse::<i64>().unwrap();
        let c = chunks.next().unwrap().parse::<i64>().unwrap();
        cur_map.push((b..(b + c), a - b));
    }

    maps.push(cur_map);

    maps
}

fn run_ranges_through_maps(
    maps: &[ArrayVec<(Range<i64>, i64), MAP_SIZE>],
    mut seed_ranges: ArrayVec<Range<i64>, SEED_RANGES_COUNT>,
) -> i64 {
    let mut min = i64::MAX;

    let mut buffer = ArrayVec::<Range<i64>, SEED_RANGES_COUNT>::new();
    let mut mapped = ArrayVec::<Range<i64>, SEED_RANGES_COUNT>::new();

    let mut unmapped = &mut seed_ranges;
    let mut unmapped_new = &mut buffer;

    for map in maps.iter() {
        loop {
            let mut mapped_some = false;

            for (chopper, delta) in map.iter() {
                for targ in unmapped.iter() {
                    let chopped = chop_range(chopper.clone(), targ.clone());
                    if let Some(mut inner) = chopped.inside {
                        inner.start += *delta;
                        inner.end += *delta;
                        mapped.push(inner);
                        mapped_some = true;
                    }
                    for y in chopped.outside {
                        unmapped_new.push(y);
                    }
                }
                swap(&mut unmapped, &mut unmapped_new);
                unmapped_new.clear();
            }

            if !mapped_some {
                break;
            }
        }

        unmapped.extend(mapped.drain(0..));
    }

    for range in unmapped {
        min = min.min(range.start);
    }

    min
}

struct RangeChopResult {
    inside: Option<Range<i64>>,
    outside: ArrayVec<Range<i64>, 2>,
}

fn chop_range(chopper: Range<i64>, targ: Range<i64>) -> RangeChopResult {
    let mut outside = ArrayVec::new();

    if targ.start >= chopper.end || targ.end <= chopper.start {
        outside.push(targ);
        RangeChopResult {
            inside: None,
            outside,
        }
    } else if targ.start >= chopper.start {
        if targ.end <= chopper.end {
            RangeChopResult {
                inside: Some(targ),
                outside,
            }
        } else {
            outside.push(chopper.end..targ.end);
            RangeChopResult {
                inside: Some(targ.start..chopper.end),
                outside,
            }
        }
    } else if targ.end <= chopper.end {
        outside.push(targ.start..chopper.start);
        RangeChopResult {
            inside: Some(chopper.start..targ.end),
            outside,
        }
    } else {
        outside.push(targ.start..chopper.start);
        outside.push(chopper.end..targ.end);
        RangeChopResult {
            inside: Some(chopper),
            outside,
        }
    }
}
