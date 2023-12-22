use std::{fmt::Write, mem::zeroed};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Cell {
    Empty = 0,
    Wall = 1,
    Visited = 2,
}

const GRID_SIZE: usize = 131;

#[derive(Clone)]
struct Grid([[Cell; GRID_SIZE]; GRID_SIZE]);

#[derive(Clone)]
struct GridSim {
    grid_0: Grid,
    grid_1: Grid,
    idx: bool,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let cell = match ch {
                'S' => Cell::Visited,
                '#' => Cell::Wall,
                _dot => Cell::Empty,
            };
            grid.0[row][col] = cell;
        }
    }

    let mut sim = GridSim::new(grid);

    sim.step(64);

    let result_0 = sim.count(mask_all);

    sim.step(130 - 64);

    let small_full = sim.count(mask_all);
    let small_0 = sim.count(mask_small_top_left);
    let small_1 = sim.count(mask_small_top_right);
    let small_2 = sim.count(mask_small_bottom_left);
    let small_3 = sim.count(mask_small_bottom_right);

    sim.step(1);

    let big_full = sim.count(mask_all);
    let big_0 = sim.count(mask_big_top_left);
    let big_1 = sim.count(mask_big_top_right);
    let big_2 = sim.count(mask_big_bottom_left);
    let big_3 = sim.count(mask_big_bottom_right);

    let wedge_0 = sim.count(mask_wedge_top);
    let wedge_1 = sim.count(mask_wedge_left);
    let wedge_2 = sim.count(mask_wedge_right);
    let wedge_3 = sim.count(mask_wedge_bottom);

    let iterations = 26501365_u64;
    let n = (iterations - 65) / 131;

    #[rustfmt::skip]
    let partial_board_cells =
                n * (small_0 + small_1 + small_2 + small_3)
        + (n - 1) * (  big_0 +   big_1 +   big_2 +   big_3)
        +            wedge_0 + wedge_1 + wedge_2 + wedge_3;

    let small_n = n / 2;
    let small_last_term = 4 + 8 * (small_n - 1);
    let small_full_boards = small_n * (4 + small_last_term) / 2;

    let big_n = (n - 1) / 2;
    let big_last_term = 8 + 8 * (big_n - 1);
    let big_full_boards = 1 + big_n * (8 + big_last_term) / 2;

    let full_board_cells = big_full_boards * big_full + small_full_boards * small_full;

    let result_1 = partial_board_cells + full_board_cells;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn mask_all(_: i32, _: i32, _: i32) -> bool {
    true
}
fn mask_small_top_left(dim: i32, r: i32, c: i32) -> bool {
    -c - dim >= r
}
fn mask_big_top_left(dim: i32, r: i32, c: i32) -> bool {
    -c - dim <= r
}
fn mask_small_top_right(dim: i32, r: i32, c: i32) -> bool {
    c - dim >= r
}
fn mask_big_top_right(dim: i32, r: i32, c: i32) -> bool {
    c - dim <= r
}
fn mask_small_bottom_right(dim: i32, r: i32, c: i32) -> bool {
    -c + dim <= r
}
fn mask_big_bottom_right(dim: i32, r: i32, c: i32) -> bool {
    -c + dim >= r
}
fn mask_small_bottom_left(dim: i32, r: i32, c: i32) -> bool {
    c + dim <= r
}
fn mask_big_bottom_left(dim: i32, r: i32, c: i32) -> bool {
    c + dim >= r
}
fn mask_wedge_top(dim: i32, r: i32, c: i32) -> bool {
    mask_big_top_left(dim, r, c) && mask_big_top_right(dim, r, c)
}
fn mask_wedge_left(dim: i32, r: i32, c: i32) -> bool {
    mask_big_top_left(dim, r, c) && mask_big_bottom_left(dim, r, c)
}
fn mask_wedge_right(dim: i32, r: i32, c: i32) -> bool {
    mask_big_top_right(dim, r, c) && mask_big_bottom_right(dim, r, c)
}
fn mask_wedge_bottom(dim: i32, r: i32, c: i32) -> bool {
    mask_big_bottom_left(dim, r, c) && mask_big_bottom_right(dim, r, c)
}

impl GridSim {
    fn new(grid: Grid) -> Self {
        Self {
            grid_0: grid.clone(),
            grid_1: grid,
            idx: false,
        }
    }

    fn grid(&self) -> &Grid {
        if self.idx {
            &self.grid_1
        } else {
            &self.grid_0
        }
    }

    fn step(&mut self, times: usize) {
        for _ in 0..times {
            let (grid_read, grid_write) = if self.idx {
                (&mut self.grid_1.0, &mut self.grid_0.0)
            } else {
                (&mut self.grid_0.0, &mut self.grid_1.0)
            };

            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    if grid_read[row][col] == Cell::Wall {
                        continue;
                    }

                    let cond_a = row > 0 && grid_read[row - 1][col] == Cell::Visited;
                    let cond_b = col > 0 && grid_read[row][col - 1] == Cell::Visited;
                    let cond_c = row < GRID_SIZE - 1 && grid_read[row + 1][col] == Cell::Visited;
                    let cond_d = col < GRID_SIZE - 1 && grid_read[row][col + 1] == Cell::Visited;

                    grid_write[row][col] = if cond_a || cond_b || cond_c || cond_d {
                        Cell::Visited
                    } else {
                        Cell::Empty
                    };
                }
            }
            self.idx = !self.idx;
        }
    }

    fn count(&self, mask: fn(i32, i32, i32) -> bool) -> u64 {
        let mut count = 0;
        let edge = GRID_SIZE as i32 / 2;
        let grid = &self.grid().0;
        for r in -edge..=edge {
            for c in -edge..=edge {
                if !mask(edge, r, c) {
                    continue;
                }
                let ri = (edge + r) as usize;
                let ci = (edge + c) as usize;
                count += (grid[ri][ci] == Cell::Visited) as u64;
            }
        }
        count
    }

    #[allow(unused)]
    fn print_masked(&self, mask: fn(i32, i32, i32) -> bool) {
        let mut this = self.clone();
        let edge = GRID_SIZE as i32 / 2;
        for r in -edge..=edge {
            for c in -edge..=edge {
                if mask(edge, r, c) {
                    continue;
                }
                let ri = (edge + r) as usize;
                let ci = (edge + c) as usize;
                this.grid_0.0[ri][ci] = Cell::Empty;
                this.grid_1.0[ri][ci] = Cell::Empty;
            }
        }
        this.print();
    }
    #[allow(unused)]
    fn print(&self) {
        for (i, row) in self.grid().0.iter().enumerate() {
            print!("{:03} ", i);
            for ch in row.iter() {
                print!(
                    "{}",
                    match ch {
                        Cell::Empty => ".",
                        Cell::Wall => "#",
                        Cell::Visited => "O",
                    }
                );
            }
            println!();
        }
    }
}
