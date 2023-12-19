use std::{fmt::Write, mem::zeroed};

const GRID_SIZE: usize = 1000;

type Grid = [[u32; GRID_SIZE]; GRID_SIZE];

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };

    let mut row = GRID_SIZE / 2;
    let mut col = GRID_SIZE / 2;
    let mut winding = 0_i32;
    let mut prev = &lines[lines.len() - 1][0..1];
    let mut outside_count = 0;

    grid[row][col] = 0xffffffff;

    for line in lines {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap().parse::<usize>().unwrap();
        let _color = parts.next().unwrap();
        let color = 0xffffff00 | 0xff;
        #[allow(clippy::needless_range_loop)]
        match dir {
            "R" => {
                if prev == "U" {
                    winding += 1;
                } else if prev == "D" {
                    winding -= 1;
                }
                for c in (col + 1)..=(col + dist) {
                    grid[row][c] = color;
                }
                outside_count += dist;
                col += dist;
            }
            "L" => {
                if prev == "U" {
                    winding -= 1;
                } else if prev == "D" {
                    winding += 1;
                }
                for c in ((col - dist)..=(col - 1)).rev() {
                    grid[row][c] = color;
                }
                outside_count += dist;
                col -= dist;
            }
            "D" => {
                if prev == "L" {
                    winding -= 1;
                } else if prev == "R" {
                    winding += 1;
                }
                for r in (row + 1)..=(row + dist) {
                    grid[r][col] = color;
                }
                outside_count += dist;
                row += dist;
            }
            "U" => {
                if prev == "L" {
                    winding += 1;
                } else if prev == "R" {
                    winding -= 1;
                }
                for r in ((row - dist)..=(row - 1)).rev() {
                    grid[r][col] = color;
                }
                outside_count += dist;
                row -= dist;
            }
            _ => panic!(),
        }
        prev = dir;
    }

    let first_dir = &lines[0][0..1];
    let (in_row, in_col) = match (first_dir, winding.signum() > 0) {
        ("R", true) => (GRID_SIZE / 2 + 1, GRID_SIZE / 2 + 1),
        ("R", false) => (GRID_SIZE / 2 - 1, GRID_SIZE / 2 + 1),
        ("L", true) => (GRID_SIZE / 2 - 1, GRID_SIZE / 2 - 1),
        ("L", false) => (GRID_SIZE / 2 + 1, GRID_SIZE / 2 - 1),
        ("D", true) => (GRID_SIZE / 2 + 1, GRID_SIZE / 2 - 1),
        ("D", false) => (GRID_SIZE / 2 + 1, GRID_SIZE / 2 + 1),
        ("U", true) => (GRID_SIZE / 2 - 1, GRID_SIZE / 2 + 1),
        ("U", false) => (GRID_SIZE / 2 - 1, GRID_SIZE / 2 - 1),
        _ => panic!(),
    };

    let inside_count = flood(&mut grid, in_row, in_col);

    let result_0 = outside_count + inside_count;
    let result_1 = 0;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn flood(grid: &mut Grid, row: usize, col: usize) -> usize {
    if grid[row][col] != 0 {
        return 0;
    }
    let mut ret = 1;
    grid[row][col] = 0xff;
    if row > 0 {
        ret += flood(grid, row - 1, col);
    }
    if row < GRID_SIZE - 1 {
        ret += flood(grid, row + 1, col);
    }
    if col > 0 {
        ret += flood(grid, row, col - 1);
    }
    if col < GRID_SIZE - 1 {
        ret += flood(grid, row, col + 1);
    }
    ret
}
