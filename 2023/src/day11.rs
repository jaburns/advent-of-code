use arrayvec::ArrayVec;
use glam::IVec2;
use std::{fmt::Write, mem::zeroed};

const GRID_SIZE: usize = 140;
const MAX_GALAXIES: usize = 512;

type Grid = [[bool; GRID_SIZE]; GRID_SIZE];

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let grid_size = lines[0].len();

    let mut grid: Grid = unsafe { zeroed() };

    for (row, line) in lines.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            grid[row][col] = chr == '#';
        }
    }

    let mut empty_rows = ArrayVec::<u8, GRID_SIZE>::new();
    let mut empty_cols = ArrayVec::<u8, GRID_SIZE>::new();

    for (row_i, row) in grid.iter().take(grid_size).enumerate() {
        let has_galaxy = row.iter().take(grid_size).any(|x| *x);
        if !has_galaxy {
            empty_rows.push(row_i as u8);
        }
    }
    for col_i in 0..grid_size {
        let mut has_galaxy = false;
        for row in grid.iter().take(grid_size) {
            if row[col_i] {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            empty_cols.push(col_i as u8);
        }
    }

    let mut galaxies = ArrayVec::<(IVec2, IVec2), MAX_GALAXIES>::new();
    {
        let mut empty_row_idx = 0;

        let mut coord_0 = IVec2::ZERO;
        let mut coord_1 = IVec2::ZERO;

        for (row_i, row) in grid.iter().take(grid_size).enumerate() {
            if empty_row_idx < empty_rows.len() && empty_rows[empty_row_idx] == row_i as u8 {
                coord_0.y += 1;
                coord_1.y += 999_999;
                empty_row_idx += 1;
            }

            let mut empty_col_idx = 0;
            coord_0.x = 0;
            coord_1.x = 0;

            for (col_i, cell) in row.iter().take(grid_size).enumerate() {
                if empty_col_idx < empty_cols.len() && empty_cols[empty_col_idx] == col_i as u8 {
                    coord_0.x += 1;
                    coord_1.x += 999_999;
                    empty_col_idx += 1;
                }

                if *cell {
                    galaxies.push((coord_0, coord_1));
                }

                coord_0.x += 1;
                coord_1.x += 1;
            }

            coord_0.y += 1;
            coord_1.y += 1;
        }
    }

    let mut result_0 = 0_u64;
    let mut result_1 = 0_u64;

    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let (a, b) = unsafe { (galaxies.get_unchecked(i), galaxies.get_unchecked(j)) };

            let delta_0 = b.0 - a.0;
            result_0 += (delta_0.x.abs() + delta_0.y.abs()) as u64;

            let delta_1 = b.1 - a.1;
            result_1 += (delta_1.x.abs() + delta_1.y.abs()) as u64;
        }
    }

    write!(out, "{}  {}", result_0, result_1).unwrap();
}
