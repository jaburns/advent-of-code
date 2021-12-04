#!/usr/bin/env python3
import sys
import requests
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

def main():
    try:
        day = int(sys.argv[-1])
    except:
        day = 0

    if day >= 1 and day <= 25:
        build_day_txt(day)
        build_day_rs_if_not_exist(day)
    else:
        print("Usage:")
        print("  ./get.py [day]")

if __name__ == "__main__":
    main()
