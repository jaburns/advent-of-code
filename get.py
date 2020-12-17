#!/usr/bin/env python3
import sys
import requests
import glob
import os.path
from os import path

RUST_DAY_TEMPLATE = """
pub fn main() {
    let lines = std::fs::read_to_string("data/day$DAY.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    println!("{:?}", lines);
}
"""

RUST_MAIN_TEMPLATE = """$MODS
mod utils;

use day$DEFAULT as default_day;

fn run_from_arg(arg: i32) {
    match arg {
$RUNS
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run_from_arg(args[1].parse::<i32>().unwrap())
    } else {
        default_day::main();
    }
}
"""

def build_day_txt(day):
    with open('session.txt', 'r') as file:
        cookies = {'session': file.read().strip()}
    data = requests.get('https://adventofcode.com/2020/day/'+str(day)+'/input', cookies=cookies)
    with open('data/day'+str(day)+'.txt', 'w') as file:
        file.write(data.text.strip())

def build_day_rs_if_not_exist(day):
    src_path = 'src/day'+str(day)+'.rs'
    if path.exists(src_path):
        return
    with open(src_path, 'w') as file:
        file.write(RUST_DAY_TEMPLATE.replace('$DAY', str(day)))

def generate_main_rs(default_day):
    mods = []
    for file in glob.glob("src/day*.rs"):
        mods.append(int(file.replace('src/day','').replace('.rs','')))
    mods.sort()

    code = RUST_MAIN_TEMPLATE
    code = code.replace('$MODS', '\n'.join(['mod day'+str(i)+';' for i in mods]))
    code = code.replace('$RUNS', '\n'.join(['        '+str(i)+' => day'+str(i)+'::main(),' for i in mods]))
    code = code.replace('$DEFAULT', str(default_day))

    with open('src/main.rs', 'w') as file:
        file.write(code)

def main():
    try:
        day = int(sys.argv[-1])
    except:
        day = 0

    if day >= 1 and day <= 24:
        build_day_txt(day)
        build_day_rs_if_not_exist(day)
        generate_main_rs(day)
    else:
        print("Usage:")
        print("  ./scaffold.py [day]")

if __name__ == "__main__":
    main()