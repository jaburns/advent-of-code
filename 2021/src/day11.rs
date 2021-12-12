#![allow(clippy::needless_range_loop)]
use std::fmt::Write;

type Grid = [[(u8, bool); 10]; 10];

pub fn part1(lines: &[&str], out: &mut String) {
    let mut grid = parse_grid(lines);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step_grid(&mut grid);
    }

    write!(out, "{}", flashes).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut grid = parse_grid(lines);
    let mut counter = 1;

    while step_grid(&mut grid) < 100 {
        counter += 1;
    }

    write!(out, "{}", counter).unwrap();
}

fn parse_grid(lines: &[&str]) -> Grid {
    let mut ret = Grid::default();

    for (y, row) in lines.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            ret[x][y] = (ch.to_digit(10).unwrap() as u8, false);
        }
    }

    ret
}

fn step_grid(grid: &mut Grid) -> u32 {
    let mut total_flashes = 0;

    for x in 0..10 {
        for y in 0..10 {
            grid[x][y].0 += 1;
            grid[x][y].1 = false;
        }
    }

    loop {
        let mut flashes = 0;

        for x in 0..10 {
            for y in 0..10 {
                if grid[x][y].0 < 10 {
                    continue;
                }

                grid[x][y].0 = 0;
                grid[x][y].1 = true;
                flashes += 1;

                let min_x = if x == 0 { 0 } else { x - 1 };
                let max_x = (x + 1).min(9);
                let min_y = if y == 0 { 0 } else { y - 1 };
                let max_y = (y + 1).min(9);

                for nx in min_x..=max_x {
                    for ny in min_y..=max_y {
                        if !grid[nx][ny].1 {
                            grid[nx][ny].0 += 1;
                        }
                    }
                }
            }
        }

        total_flashes += flashes;

        if flashes == 0 {
            break;
        }
    }

    total_flashes
}
