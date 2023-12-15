use ahash::AHasher;
use arrayvec::ArrayVec;
use std::fmt::Write;
use std::hash::{Hash, Hasher};

type HashMap<K, V> =
    std::collections::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

const GRID_SIZE: usize = 100;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Square,
    Round,
}

type GridRow = ArrayVec<Cell, GRID_SIZE>;
type Grid = ArrayVec<GridRow, GRID_SIZE>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid = Grid::new();

    for line in lines {
        let row: GridRow = line
            .chars()
            .map(|c| match c {
                'O' => Cell::Round,
                '#' => Cell::Square,
                _ => Cell::Empty,
            })
            .collect();
        grid.push(row);
    }

    let result_0 = {
        let mut grid = grid.clone();
        while grid_step(&mut grid, 0) {}
        grid_measure_total_load(&grid)
    };

    let result_1 = {
        let mut iterations = 0_u64;
        let mut seen = HashMap::<(u32, u64), u64>::default();
        let mut jumped = false;

        for mode in (0..4).cycle() {
            if !jumped {
                let hash = grid_hash(&grid);
                if let Some(seen_at) = seen.get(&(mode, hash)) {
                    let cycle_length = iterations - *seen_at;
                    let iterations_left = 4_000_000_000 - iterations;
                    iterations += (iterations_left / cycle_length) * cycle_length;
                    jumped = true;
                } else {
                    seen.insert((mode, hash), iterations);
                }
            }

            iterations += 1;
            while grid_step(&mut grid, mode) {}
            if iterations == 4_000_000_000 {
                break;
            }
        }

        grid_measure_total_load(&grid)
    };

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn grid_step(grid: &mut Grid, mode: u32) -> bool {
    macro_rules! get {
        [$grid: expr; $row: expr, $col: expr] => {
            unsafe { grid.get_unchecked_mut($row).get_unchecked_mut($col) }
        }
    }

    let row_count = grid.len();
    let col_count = grid[0].len();
    let mut changed = false;

    match mode {
        0 => {
            for row_idx in 0..(row_count - 1) {
                for col_idx in 0..col_count {
                    if *get![grid; row_idx, col_idx] == Cell::Empty
                        && *get![grid; row_idx + 1, col_idx] == Cell::Round
                    {
                        *get![grid; row_idx, col_idx] = Cell::Round;
                        *get![grid; row_idx + 1, col_idx] = Cell::Empty;
                        changed = true;
                    }
                }
            }
        }
        1 => {
            for col_idx in 0..(col_count - 1) {
                for row_idx in 0..row_count {
                    if *get![grid; row_idx, col_idx] == Cell::Empty
                        && *get![grid; row_idx, col_idx + 1] == Cell::Round
                    {
                        *get![grid; row_idx, col_idx] = Cell::Round;
                        *get![grid; row_idx, col_idx + 1] = Cell::Empty;
                        changed = true;
                    }
                }
            }
        }
        2 => {
            for row_idx in (1..row_count).rev() {
                for col_idx in 0..col_count {
                    if *get![grid; row_idx, col_idx] == Cell::Empty
                        && *get![grid; row_idx - 1, col_idx] == Cell::Round
                    {
                        *get![grid; row_idx, col_idx] = Cell::Round;
                        *get![grid; row_idx - 1, col_idx] = Cell::Empty;
                        changed = true;
                    }
                }
            }
        }
        3 => {
            for col_idx in (1..col_count).rev() {
                for row_idx in 0..row_count {
                    if *get![grid; row_idx, col_idx] == Cell::Empty
                        && *get![grid; row_idx, col_idx - 1] == Cell::Round
                    {
                        *get![grid; row_idx, col_idx] = Cell::Round;
                        *get![grid; row_idx, col_idx - 1] = Cell::Empty;
                        changed = true;
                    }
                }
            }
        }
        _ => panic!(),
    }

    changed
}

fn grid_hash(grid: &Grid) -> u64 {
    let mut hasher = AHasher::default();
    grid.hash(&mut hasher);
    hasher.finish()
}

#[allow(unused)]
fn grid_print(grid: &Grid) {
    for row in grid.iter() {
        for ch in row.iter() {
            print!(
                "{}",
                match ch {
                    Cell::Empty => ".",
                    Cell::Square => "#",
                    Cell::Round => "O",
                }
            );
        }
        println!();
    }
}

fn grid_measure_total_load(grid: &Grid) -> u64 {
    let mut total = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        let load = (grid.len() - row_idx) as u64;
        total += row.iter().filter(|x| **x == Cell::Round).count() as u64 * load;
    }

    total
}
