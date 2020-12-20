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
        file.write('#[rustfmt::skip]\n')
        file.write('pub fn eval_data_part1() -> Val1 {\n')
        file.write('\n'.join(out_lines_1))
        file.write('\n}\n\n')
        file.write('#[rustfmt::skip]\n')
        file.write('pub fn eval_data_part2() -> Val2 {\n')
        file.write('\n'.join(out_lines_2))
        file.write('\n}\n')

def translate_day19_rule(mode, rule):
    parts = rule.split(': ')
    out = 'pub fn rule_' + mode + '_' + parts[0] + "<'a>() -> Parser<'a, char, ()> { "

    def translate_inner(part):
        return ' * '.join([ 'rule_' + mode + '_' + x + '()' for x in part.split(' ') ])

    if '"' in parts[1]:
        out = out + "sym('" + parts[1].replace('"', '') + "').discard()"
    elif '|' in parts[1]:
        subs = parts[1].split(' | ')
        out = out + translate_inner(subs[0]) + ' | ' + translate_inner(subs[1])
    else:
        out = out + translate_inner(parts[1])

    return out + ' }'

def day19():
    with open('data/day19.txt', 'r') as file:
        in_lines = [ l.strip() for l in file.readlines() ]

    out_lines = []
    out_lines = []
    for line in in_lines:
        if line == '':
            break
        out_lines.append(translate_day19_rule('1', line))
        if line.startswith('8: '):
            line = '8: 42 | 42 8'
        elif line.startswith('11: '):
            line = '11: 42 31 | 42 11 31'
        out_lines.append(translate_day19_rule('2', line))

    with open('src/day19_gen.rs', 'w') as file:
        file.write('use pom::parser::*;\n')
        file.write('\n'.join(out_lines))

def main():
    day18()
    day19()

if __name__ == "__main__":
    main()