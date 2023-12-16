use std::{fmt::Write, mem::zeroed};

const GRID_SIZE: usize = 110;

const KIND_BACKSLASH: u8 = '\\' as u8;
const KIND_SLASH: u8 = '/' as u8;
const KIND_DASH: u8 = '-' as u8;
const KIND_PIPE: u8 = '|' as u8;

const FLAG_ENERGIZED_LEFT: u8 = 1;
const FLAG_ENERGIZED_RIGHT: u8 = 2;
const FLAG_ENERGIZED_UP: u8 = 4;
const FLAG_ENERGIZED_DOWN: u8 = 8;

struct Cell {
    kind: u8,
    energized_from: u8,
}

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

macro_rules! get {
    [$grid: expr; $row: expr, $col: expr] => {
        unsafe { $grid.get_unchecked_mut($row).get_unchecked_mut($col) }
    }
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };
    let grid_size = lines.len() as isize;

    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            grid[row_idx][col_idx].kind = ch as u8;
        }
    }

    energize(&mut grid, grid_size, 0, 0, FLAG_ENERGIZED_LEFT);
    let result_0 = count_energy_and_reset(&mut grid, grid_size);

    let mut result_1 = 0;
    for i in 0..grid_size {
        energize(&mut grid, grid_size, i, 0, FLAG_ENERGIZED_LEFT);
        result_1 = result_1.max(count_energy_and_reset(&mut grid, grid_size));
        energize(&mut grid, grid_size, i, grid_size - 1, FLAG_ENERGIZED_RIGHT);
        result_1 = result_1.max(count_energy_and_reset(&mut grid, grid_size));
        energize(&mut grid, grid_size, 0, i, FLAG_ENERGIZED_UP);
        result_1 = result_1.max(count_energy_and_reset(&mut grid, grid_size));
        energize(&mut grid, grid_size, grid_size - 1, i, FLAG_ENERGIZED_DOWN);
        result_1 = result_1.max(count_energy_and_reset(&mut grid, grid_size));
    }

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn count_energy_and_reset(grid: &mut Grid, grid_size: isize) -> usize {
    let mut count = 0;
    for row in 0..grid_size as usize {
        for col in 0..grid_size as usize {
            let cell = get!(grid; row, col);
            count += (cell.energized_from != 0) as usize;
            cell.energized_from = 0;
        }
    }
    count
}

fn energize(grid: &mut Grid, grid_size: isize, mut row: isize, mut col: isize, mut from: u8) {
    loop {
        if row < 0 || col < 0 || row >= grid_size || col >= grid_size {
            return;
        }
        let cell = get!(grid; row as usize, col as usize);
        if (cell.energized_from & from) != 0 {
            return;
        }
        cell.energized_from |= from;

        match cell.kind {
            KIND_BACKSLASH => match from {
                FLAG_ENERGIZED_LEFT => {
                    row += 1;
                    from = FLAG_ENERGIZED_UP;
                }
                FLAG_ENERGIZED_RIGHT => {
                    row -= 1;
                    from = FLAG_ENERGIZED_DOWN;
                }
                FLAG_ENERGIZED_UP => {
                    col += 1;
                    from = FLAG_ENERGIZED_LEFT;
                }
                _down => {
                    col -= 1;
                    from = FLAG_ENERGIZED_RIGHT;
                }
            },
            KIND_SLASH => match from {
                FLAG_ENERGIZED_LEFT => {
                    row -= 1;
                    from = FLAG_ENERGIZED_DOWN;
                }
                FLAG_ENERGIZED_RIGHT => {
                    row += 1;
                    from = FLAG_ENERGIZED_UP;
                }
                FLAG_ENERGIZED_UP => {
                    col -= 1;
                    from = FLAG_ENERGIZED_RIGHT;
                }
                _down => {
                    col += 1;
                    from = FLAG_ENERGIZED_LEFT;
                }
            },
            KIND_DASH => match from {
                FLAG_ENERGIZED_LEFT => {
                    col += 1;
                }
                FLAG_ENERGIZED_RIGHT => {
                    col -= 1;
                }
                _up_or_down => {
                    energize(grid, grid_size, row, col + 1, FLAG_ENERGIZED_LEFT);
                    energize(grid, grid_size, row, col - 1, FLAG_ENERGIZED_RIGHT);
                    return;
                }
            },
            KIND_PIPE => match from {
                FLAG_ENERGIZED_UP => {
                    row += 1;
                }
                FLAG_ENERGIZED_DOWN => {
                    row -= 1;
                }
                _left_or_right => {
                    energize(grid, grid_size, row + 1, col, FLAG_ENERGIZED_UP);
                    energize(grid, grid_size, row - 1, col, FLAG_ENERGIZED_DOWN);
                    return;
                }
            },
            _empty => match from {
                FLAG_ENERGIZED_LEFT => {
                    col += 1;
                }
                FLAG_ENERGIZED_RIGHT => {
                    col -= 1;
                }
                FLAG_ENERGIZED_UP => {
                    row += 1;
                }
                _down => {
                    row -= 1;
                }
            },
        }
    }
}
