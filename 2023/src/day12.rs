use arrayvec::ArrayVec;
use std::{fmt::Write, mem::zeroed};

const ENABLE_PARALLELISM: bool = true;
const PRINT_PROGRESS: bool = false;

const MAX_SLOTS: usize = 128;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Slot {
    Empty,
    Full,
    Unknown,
}

#[derive(Debug, Clone)]
struct Entry {
    slots: ArrayVec<Slot, MAX_SLOTS>,
    groups: ArrayVec<u8, MAX_SLOTS>,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let result = if ENABLE_PARALLELISM {
        use rayon::prelude::*;
        lines
            .par_iter()
            .enumerate()
            .map(|(i, line)| solve_line(i, lines.len(), line))
            .reduce(|| (0, 0), |(x0, y0), (x1, y1)| (x0 + x1, y0 + y1))
    } else {
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| solve_line(i, lines.len(), line))
            .reduce(|(x0, y0), (x1, y1)| (x0 + x1, y0 + y1))
            .unwrap()
    };
    write!(out, "{}  {}", result.0, result.1).unwrap();
}

pub fn solve_line(i: usize, len: usize, line: &str) -> (usize, usize) {
    let mut halves = line.split_whitespace();

    let mut entry: Entry = unsafe { zeroed() };

    for ch in halves.next().unwrap().chars() {
        if ch == '.' {
            entry.slots.push(Slot::Empty);
        } else if ch == '#' {
            entry.slots.push(Slot::Full);
        } else {
            entry.slots.push(Slot::Unknown);
        }
    }

    entry.groups = halves
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    if PRINT_PROGRESS {
        println!("Working on ({} / {}): '{}'", i, len, line);
    }

    let result_0 = count_arrangements(entry.clone());

    let mut entry_1: Entry = unsafe { zeroed() };
    for i in 0..5 {
        for slot in entry.slots.iter() {
            entry_1.slots.push(*slot);
        }
        if i < 4 {
            entry_1.slots.push(Slot::Unknown);
        }
        for g in entry.groups.iter() {
            entry_1.groups.push(*g);
        }
    }

    let result_1 = count_arrangements(entry_1);

    (result_0, result_1)
}

#[derive(Debug, Default, Clone)]
struct State {
    slot_idx: usize,
    group_idx: usize,
    product: usize,
}

fn count_arrangements(entry: Entry) -> usize {
    let mut count = 0;
    let state = State {
        product: 1,
        ..Default::default()
    };
    go(&mut count, &entry, state);
    count
}

fn step_one_empty(count: &mut usize, entry: &Entry, mut state: State) {
    state.slot_idx += 1;
    go(count, entry, state);
}

fn step_group(count: &mut usize, entry: &Entry, mut state: State) {
    for _ in 0..entry.groups[state.group_idx] {
        if state.slot_idx >= entry.slots.len() || entry.slots[state.slot_idx] == Slot::Empty {
            return;
        }
        state.slot_idx += 1;
    }
    if state.slot_idx < entry.slots.len() && entry.slots[state.slot_idx] == Slot::Full {
        return;
    }
    state.slot_idx += 1;
    state.group_idx += 1;
    go(count, entry, state);
}

fn go(count: &mut usize, entry: &Entry, mut state: State) {
    if state.group_idx >= entry.groups.len() {
        if state.slot_idx < entry.slots.len() {
            for i in state.slot_idx..entry.slots.len() {
                if entry.slots[i] == Slot::Full {
                    return;
                }
            }
        }
        *count += state.product;
        return;
    }

    if state.slot_idx >= entry.slots.len() {
        return;
    }

    let this_slot = entry.slots[state.slot_idx];

    match this_slot {
        Slot::Empty => step_one_empty(count, entry, state),
        Slot::Full => step_group(count, entry, state),
        Slot::Unknown => {
            let mut unknown_size = 1;
            let mut end_kind = Slot::Empty;
            loop {
                if state.slot_idx + unknown_size >= entry.slots.len() {
                    break;
                }
                if entry.slots[state.slot_idx + unknown_size] == Slot::Empty {
                    break;
                }
                if entry.slots[state.slot_idx + unknown_size] == Slot::Full {
                    end_kind = Slot::Full;
                    break;
                }
                unknown_size += 1;
            }

            let group = entry.groups[state.group_idx];
            let check_offset = if end_kind == Slot::Full { 1 } else { 0 };

            // Try skipping the whole block placing no groups.
            {
                let mut state = state.clone();
                state.slot_idx += unknown_size;
                go(count, entry, state);
            }

            // Calculate the number of ways to lay down the amount of groups that fit
            // if the block ends in an empty space AND there's enough room to put at least
            // the next available group in here. If the block doesn't end in an empty space,
            // assume the last '?' is a '.'.
            if unknown_size - check_offset >= group as usize {
                let mut state = state.clone();
                let mut unknown_size_left = unknown_size - group as usize - check_offset;
                let mut buckets = 2;
                loop {
                    state.group_idx += 1;

                    let mut next_state = state.clone();
                    next_state.product *= ways_to_fit(buckets, unknown_size_left);
                    next_state.slot_idx += unknown_size;
                    go(count, entry, next_state);

                    if state.group_idx >= entry.groups.len() {
                        break;
                    }

                    let next_group_space = 1 + entry.groups[state.group_idx] as usize; // (+1) to account for space between groups
                    if unknown_size_left < next_group_space {
                        break;
                    }

                    unknown_size_left -= next_group_space;
                    buckets += 1;
                }
            }

            // Check placements of groups that extend off the end of the current block if
            // it ends with a full slot.
            if end_kind != Slot::Full {
                return;
            }
            let mut group_count = 1;
            while state.group_idx < entry.groups.len() {
                let end_group = entry.groups[state.group_idx] as usize;
                if end_group > 1 {
                    let mut extra_group_sizes = 0;
                    if group_count > 1 {
                        for i in 1..group_count {
                            extra_group_sizes += 1 + entry.groups[state.group_idx - i] as usize;
                        }
                    }

                    for from_end in 1..=(end_group.min(unknown_size)) {
                        let mut ways = 1;

                        if group_count > 1 {
                            let unknown_size_left = unknown_size - from_end;
                            if extra_group_sizes > unknown_size_left {
                                if from_end == 1 {
                                    return;
                                } else {
                                    continue;
                                }
                            }
                            ways = ways_to_fit(group_count, unknown_size_left - extra_group_sizes);
                        }

                        let mut state = state.clone();
                        state.product *= ways;
                        state.slot_idx += unknown_size - from_end;

                        step_group(count, entry, state);
                    }
                }

                group_count += 1;
                state.group_idx += 1;
            }
        }
    }
}

fn ways_to_fit(buckets: usize, slots: usize) -> usize {
    choose(buckets + slots - 1, slots)
}
fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        1
    } else {
        n * choose(n - 1, k - 1) / k
    }
}
