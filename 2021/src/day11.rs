#![allow(clippy::needless_range_loop)]
use std::fmt::Write;

type Grid = [[(u8, bool); 10]; 10];

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

                if x > 0 {
                    if y > 0 && !grid[x - 1][y - 1].1 {
                        grid[x - 1][y - 1].0 += 1;
                    }
                    if !grid[x - 1][y].1 {
                        grid[x - 1][y].0 += 1;
                    }
                    if y < 9 && !grid[x - 1][y + 1].1 {
                        grid[x - 1][y + 1].0 += 1;
                    }
                }

                if y > 0 && !grid[x][y - 1].1 {
                    grid[x][y - 1].0 += 1;
                }
                if !grid[x][y].1 {
                    grid[x][y].0 += 1;
                }
                if y < 9 && !grid[x][y + 1].1 {
                    grid[x][y + 1].0 += 1;
                }

                if x < 9 {
                    if y > 0 && !grid[x + 1][y - 1].1 {
                        grid[x + 1][y - 1].0 += 1;
                    }
                    if !grid[x + 1][y].1 {
                        grid[x + 1][y].0 += 1;
                    }
                    if y < 9 && !grid[x + 1][y + 1].1 {
                        grid[x + 1][y + 1].0 += 1;
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

fn print_grid(grid: &Grid) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", grid[x][y].0);
        }
        println!();
    }
    println!();
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut grid = parse_grid(lines);
    let mut flashes = 0;

    for _ in 0..100 {
        //print_grid(&grid);
        flashes += step_grid(&mut grid);
    }

    write!(out, "{}", flashes).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
