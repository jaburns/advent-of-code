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

fn solve_cave(cave: &[Vec<usize>]) -> usize {
    astar(
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
    .1
}

fn grow_cave(mut cave: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let og_height = cave.len();
    let og_width = cave[0].len();

    for col in cave.iter_mut() {
        col.resize(5 * og_width, 0_usize);
    }
    let row = cave[0].clone();
    cave.resize_with(5 * og_height, || row.clone());

    for r in 0..cave.len() {
        for c in 0..cave[r].len() {
            if r == 0 && c == 0 {
                continue;
            }
            let r1 = r % og_height;
            let c1 = c % og_width;
            let inc = r / og_height + c / og_width;
            cave[r][c] = (cave[r1][c1] + inc - 1) % 9 + 1;
        }
    }

    cave
}

pub fn part1(lines: &[&str], out: &mut String) {
    let result = solve_cave(&parse_cave(lines));
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = solve_cave(&grow_cave(parse_cave(lines)));
    write!(out, "{}", result).unwrap();
}
