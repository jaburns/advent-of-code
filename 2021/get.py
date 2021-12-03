#!/usr/bin/env python3
import sys
import requests
import glob
from os import path

RUST_DAY_TEMPLATE = """use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
"""

RUST_MAIN_TEMPLATE = """use std::time::Instant;

$MODS

const NUM_DAYS: usize = $NUM_DAYS;

#[global_allocator]
pub static GLOBAL: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

type PartFn = fn(&[&str], &mut String);

static DAY_FUNCS: [(PartFn, PartFn); NUM_DAYS] = [
$FUNCS
];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        NUM_DAYS
    };

    let data = std::fs::read_to_string(&format!("data/day{}.txt", day)).unwrap();
    let data_lines: Vec<&str> = data.lines().collect();

    let mut out_str = String::with_capacity(256);

    println!();
    println!("Day {} - Part One", day);

    let reg = stats_alloc::Region::new(GLOBAL);
    let start_time = Instant::now();
    DAY_FUNCS[day - 1].0(&data_lines, &mut out_str);
    let delta_time = Instant::now() - start_time;
    let stats = reg.change();

    println!("Solution: {}", out_str);
    println!("Time (us): {}", delta_time.as_micros());
    println!("{:#?}", stats);

    out_str.clear();

    println!();
    println!("Day {} - Part Two", day);

    let reg = stats_alloc::Region::new(GLOBAL);
    let start_time = Instant::now();
    DAY_FUNCS[day - 1].1(&data_lines, &mut out_str);
    let delta_time = Instant::now() - start_time;
    let stats = reg.change();

    println!("Solution: {}", out_str);
    println!("Time (us): {}", delta_time.as_micros());
    println!("{:#?}", stats);
}
"""

def build_day_txt(day):
    with open('session.txt', 'r') as file:
        cookies = {'session': file.read().strip()}
    data = requests.get('https://adventofcode.com/2021/day/'+str(day)+'/input', cookies=cookies)
    with open('data/day'+str(day)+'.txt', 'w') as file:
        file.write(data.text.strip())

def build_day_rs_if_not_exist(day):
    src_path = 'src/day'+str(day)+'.rs'
    if path.exists(src_path):
        return
    with open(src_path, 'w') as file:
        file.write(RUST_DAY_TEMPLATE)

def generate_main_rs():
    mods = []
    for file in glob.glob("src/day*.rs"):
        mods.append(file.replace('src/day','').replace('.rs',''))
    mods.sort()

    code = RUST_MAIN_TEMPLATE
    code = code.replace('$MODS', '\n'.join(['mod day'+i+';' for i in mods]))
    code = code.replace('$NUM_DAYS', str(len(mods)))
    code = code.replace('$FUNCS', '\n'.join(['    (day'+i+'::part1, day'+i+'::part2),' for i in mods if 'gen' not in i]))

    with open('src/main.rs', 'w') as file:
        file.write(code)

def main():
    try:
        day = int(sys.argv[-1])
    except:
        day = 0

    if day >= 1 and day <= 25:
        build_day_txt(day)
        build_day_rs_if_not_exist(day)
        generate_main_rs()
    else:
        print("Usage:")
        print("  ./get.py [day]")

if __name__ == "__main__":
    main()
