#!/usr/bin/env python3

def translate_day19_rule(rule):
    parts = rule.split(': ')
    out = 'pub fn rule_' + parts[0] + "<'a>() -> Parser<'a, char, ()> { "

    def translate_inner(part):
        return ' * '.join([ 'rule_' + x + '()' for x in part.split(' ') ])

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

    out_lines = ['use pom::parser::*;\n']
    for line in in_lines:
        if line == '':
            break
        out_lines.append(translate_day19_rule(line))

    with open('src/day19_gen.rs', 'w') as file:
        file.write('\n'.join(out_lines))

def main():
    day19()

if __name__ == "__main__":
    main()