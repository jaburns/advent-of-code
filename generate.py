#!/usr/bin/env python3
import re

def day18():
    with open('data/day18.txt', 'r') as file:
        in_lines = [ l.strip() for l in file.readlines() ]

    out_lines_1 = []
    out_lines_2 = []
    re_digit = re.compile(r'([0-9])')
    
    for line in in_lines:
        line1 = re_digit.sub(r'Val1(\1)', line).replace('*', '-')
        line2 = re_digit.sub(r'Val2(\1)', line).replace('*', '-').replace('+', '*').replace('-', '+')
        out_lines_1.append('( ' + line1 + ' ) +')
        out_lines_2.append('( ' + line2 + ' ) *')

    out_lines_1.append('Val1(0)')
    out_lines_2.append('Val2(0)')

    with open('src/day18_gen.rs', 'w') as file:
        file.write('use crate::day18::{Val1,Val2};\n\n')
        file.write('pub fn eval_data_part1() -> Val1 {\n')
        file.write('\n'.join(out_lines_1))
        file.write('\n}\n\n')
        file.write('pub fn eval_data_part2() -> Val2 {\n')
        file.write('\n'.join(out_lines_2))
        file.write('\n}\n')

def main():
    day18()

if __name__ == "__main__":
    main()