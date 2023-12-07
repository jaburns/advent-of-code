use std::{fmt::Write, mem::swap, ops::Range};

use smallvec::{smallvec, SmallVec};

type Maps = Vec<Vec<(Range<i64>, i64)>>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let seeds: Vec<i64> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let seed_ranges_2: Vec<Range<i64>> = seeds
        .array_chunks::<2>()
        .map(|&[a, b]| a..(a + b))
        .collect();

    let seed_ranges_1: Vec<Range<i64>> = seeds.into_iter().map(|x| x..(x + 1)).collect();

    let maps = get_maps(&lines[3..]);
    let result_1 = run_ranges_through_maps(&maps, seed_ranges_1);
    let result_2 = run_ranges_through_maps(&maps, seed_ranges_2);

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

fn get_maps(lines: &[&str]) -> Maps {
    const MAPS_COUNT: usize = 8;
    const MAP_SIZE: usize = 64;

    let mut maps = Vec::with_capacity(MAPS_COUNT);
    let mut skip = 0;
    let mut cur_map = Vec::with_capacity(MAP_SIZE);

    for line in lines.iter() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if line.is_empty() {
            skip = 1;
            maps.push(cur_map);
            cur_map = Vec::with_capacity(MAPS_COUNT);
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

fn run_ranges_through_maps(maps: &Maps, seed_ranges: Vec<Range<i64>>) -> i64 {
    let mut min = i64::MAX;

    let mut unmapped = seed_ranges;
    let mut unmapped_new = vec![];
    let mut mapped = vec![];

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

        unmapped.append(&mut mapped);
    }

    for range in unmapped {
        min = min.min(range.start);
    }

    min
}

struct RangeChopResult {
    inside: Option<Range<i64>>,
    outside: SmallVec<[Range<i64>; 2]>,
}

fn chop_range(chopper: Range<i64>, targ: Range<i64>) -> RangeChopResult {
    if targ.start >= chopper.end || targ.end <= chopper.start {
        RangeChopResult {
            inside: None,
            outside: smallvec![targ],
        }
    } else if targ.start >= chopper.start {
        if targ.end <= chopper.end {
            RangeChopResult {
                inside: Some(targ),
                outside: smallvec![],
            }
        } else {
            RangeChopResult {
                inside: Some(targ.start..chopper.end),
                outside: smallvec![chopper.end..targ.end],
            }
        }
    } else if targ.end <= chopper.end {
        RangeChopResult {
            inside: Some(chopper.start..targ.end),
            outside: smallvec![targ.start..chopper.start],
        }
    } else {
        RangeChopResult {
            inside: Some(chopper.clone()),
            outside: smallvec![targ.start..chopper.start, chopper.end..targ.end],
        }
    }
}
