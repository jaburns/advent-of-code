use pathfinding::prelude::astar;
use std::fmt::Write;

fn parse_cave(lines: &[&str]) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn part1(lines: &[&str], out: &mut String) {
    let cave = parse_cave(lines);

    let result = astar(
        &(0_usize, 0_usize),
        |&(r, c)| {
            vec![(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .into_iter()
                .filter_map(|(r, c)| {
                    if r < cave.len() && c < cave[0].len() {
                        Some(((r, c), cave[r][c]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |&(r, c)| (cave.len() - 1 - r) + (cave[0].len() - 1 - c),
        |&(r, c)| r == cave.len() - 1 && c == cave[0].len() - 1,
    )
    .unwrap()
    .1;

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
