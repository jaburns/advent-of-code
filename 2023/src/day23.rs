use std::{fmt::Write, mem::zeroed};

enum CellKind {
    Empty,
    Wall,
    HillLeft,
    HillRight,
    HillDown,
}

const GRID_SIZE: usize = 141;

struct Grid(u8, [[CellKind; GRID_SIZE]; GRID_SIZE]);

type HashSet<K> = std::collections::HashSet<K, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };
    grid.0 = lines.len() as u8;

    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, col) in row.chars().enumerate() {
            grid.1[row_idx][col_idx] = match col {
                '.' => CellKind::Empty,
                '#' => CellKind::Wall,
                '<' => CellKind::HillLeft,
                '>' => CellKind::HillRight,
                'v' => CellKind::HillDown,
                // Part 2:
                // '<' => CellKind::Empty,
                // '>' => CellKind::Empty,
                // 'v' => CellKind::Empty,
                _ => panic!(),
            };
        }
    }
    // Lazy part 2 brute force result:
    // Day 23 - Parts One and Two
    // Solution:      6426
    // Time (μs):     390957743
    // Heap (bytes):  1712414734988
    // Allocations:   77171049

    let mut branch = Branch::default();
    let mut results = vec![];
    branch.history_set.insert((0, 1));
    traverse(&mut grid, 1, 1, branch, &mut results);

    let result_0 = results.iter().max().unwrap() - 1;

    write!(out, "{}", result_0).unwrap();
}

#[derive(Default, Clone)]
struct Branch {
    history_set: HashSet<(u8, u8)>,
}

fn traverse(grid: &mut Grid, mut row: u8, mut col: u8, mut branch: Branch, results: &mut Vec<u32>) {
    loop {
        branch.history_set.insert((row, col));

        if row == grid.0 - 1 {
            results.push(branch.history_set.len() as u32);
            return;
        }

        let (r, c) = (row as usize, col as usize);

        let can_left = matches!(grid.1[r][c - 1], CellKind::Empty | CellKind::HillLeft)
            && !branch.history_set.contains(&(row, col - 1));
        let can_right = matches!(grid.1[r][c + 1], CellKind::Empty | CellKind::HillRight)
            && !branch.history_set.contains(&(row, col + 1));
        let can_up = matches!(grid.1[r - 1][c], CellKind::Empty)
            && !branch.history_set.contains(&(row - 1, col));
        let can_down = matches!(grid.1[r + 1][c], CellKind::Empty | CellKind::HillDown)
            && !branch.history_set.contains(&(row + 1, col));

        let can_sum = can_left as u8 + can_right as u8 + can_up as u8 + can_down as u8;

        if can_sum == 0 {
            return;
        } else if can_sum > 1 {
            if can_left {
                traverse(grid, row, col - 1, branch.clone(), results);
            }
            if can_right {
                traverse(grid, row, col + 1, branch.clone(), results);
            }
            if can_up {
                traverse(grid, row - 1, col, branch.clone(), results);
            }
            if can_down {
                traverse(grid, row + 1, col, branch.clone(), results);
            }
            return;
        }

        if can_left {
            col -= 1;
        } else if can_right {
            col += 1;
        } else if can_up {
            row -= 1;
        } else if can_down {
            row += 1;
        }
    }
}
