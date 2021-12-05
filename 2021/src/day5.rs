#![allow(clippy::needless_range_loop)]
use std::fmt::Write;

#[derive(Debug)]
struct VentLine {
    a: (usize, usize),
    b: (usize, usize),
}

impl VentLine {
    fn parse(line: &str) -> Self {
        let mut parts = line.split("->").map(|x| x.trim());

        let mut parts_a = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap());

        let mut parts_b = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap());

        Self {
            a: (parts_a.next().unwrap(), parts_a.next().unwrap()),
            b: (parts_b.next().unwrap(), parts_b.next().unwrap()),
        }
    }
}

const GRID_SIZE: usize = 1000;

type Grid = [[u16; GRID_SIZE]; GRID_SIZE];

fn draw_line_to_grid(grid: &mut Grid, line: &VentLine, diagonals: bool) -> u32 {
    let mut count = 0;

    if line.a.0 == line.b.0 {
        let min = line.a.1.min(line.b.1);
        let max = line.a.1.max(line.b.1);
        for i in min..=max {
            grid[line.a.0][i] += 1;
            count += (grid[line.a.0][i] == 2) as u32;
        }
    } else if line.a.1 == line.b.1 {
        let min = line.a.0.min(line.b.0);
        let max = line.a.0.max(line.b.0);
        for i in min..=max {
            grid[i][line.a.1] += 1;
            count += (grid[i][line.a.1] == 2) as u32;
        }
    } else if diagonals {
        let dx = ((line.b.0 as i32) - (line.a.0 as i32)).signum();
        let dy = ((line.b.1 as i32) - (line.a.1 as i32)).signum();
        let (mut ix, mut iy) = (line.a.0 as i32, line.a.1 as i32);

        loop {
            grid[ix as usize][iy as usize] += 1;
            count += (grid[ix as usize][iy as usize] == 2) as u32;

            ix += dx;
            iy += dy;

            if dx < 0 && ix < line.b.0 as i32 || dx > 0 && ix > line.b.0 as i32 {
                break;
            }
        }
    }

    count
}

fn run(lines: &[&str], out: &mut String, diagonals: bool) {
    let mut grid: Grid = [[0u16; GRID_SIZE]; GRID_SIZE];
    let mut count = 0u32;

    for line in lines {
        count += draw_line_to_grid(&mut grid, &VentLine::parse(line), diagonals);
    }

    write!(out, "{}", count).unwrap();
}

pub fn part1(lines: &[&str], out: &mut String) {
    run(lines, out, false);
}

pub fn part2(lines: &[&str], out: &mut String) {
    run(lines, out, true);
}
