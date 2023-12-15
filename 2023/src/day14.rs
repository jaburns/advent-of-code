use arrayvec::ArrayVec;
use std::fmt::Write;

const GRID_SIZE: usize = 100;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Square,
    Round,
}

type GridRow = ArrayVec<Cell, GRID_SIZE>;
type Grid = ArrayVec<GridRow, GRID_SIZE>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid_0 = Grid::new();

    for line in lines {
        let row: GridRow = line
            .chars()
            .map(|c| match c {
                'O' => Cell::Round,
                '#' => Cell::Square,
                _ => Cell::Empty,
            })
            .collect();
        grid_0.push(row);
    }

    while grid_step(&mut grid_0) {}

    let result_0 = grid_measure_total_load(&grid_0);
    let result_1 = 0;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn grid_step(grid: &mut Grid) -> bool {
    macro_rules! get {
        [$grid: expr; $row: expr, $col: expr] => {
            unsafe { grid.get_unchecked_mut($row).get_unchecked_mut($col) }
        }
    }

    let col_count = grid[0].len();

    let mut changed = false;
    for row_idx in 0..(grid.len() - 1) {
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
    changed
}

fn grid_measure_total_load(grid: &Grid) -> u64 {
    let mut total = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        let load = (grid.len() - row_idx) as u64;
        total += row.iter().filter(|x| **x == Cell::Round).count() as u64 * load;
    }

    total
}
