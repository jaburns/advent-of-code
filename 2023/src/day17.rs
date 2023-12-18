use arrayvec::ArrayVec;
use std::{cmp::Ordering, collections::BinaryHeap, fmt::Write};

const ARRIVED_FROM_KINDS: usize = 40;

type NeighborGetter = fn(Coord, u8) -> ArrayVec<Coord, 3>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let grid_size = lines.len();
    let mut grid = Grid::new(grid_size);

    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            for arr in 0..ARRIVED_FROM_KINDS {
                let cell = grid.get_mut(row_idx, col_idx, arr);
                cell.cost = ch.to_digit(10).unwrap() as u8;
            }
        }
    }

    let result_0 = search(&mut grid, grid_size, get_neighbors::<0, 3>);
    grid.clear();
    let result_1 = search(&mut grid, grid_size, get_neighbors::<4, 10>);

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord(u8, u8, ArrivedFrom);

#[derive(Debug, Clone, Default)]
struct Cell {
    cost: u8,
    cost_so_far: u32,
    reached: bool,
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ArrivedFrom {
    Left(u8),
    Top(u8),
    Right(u8),
    Bottom(u8),
}

impl ArrivedFrom {
    fn to_idx(self) -> usize {
        match self {
            ArrivedFrom::Left(x) => x as usize - 1,
            ArrivedFrom::Top(x) => x as usize + 9,
            ArrivedFrom::Right(x) => x as usize + 19,
            ArrivedFrom::Bottom(x) => x as usize + 29,
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
    size: usize,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            cells: vec![Cell::default(); ARRIVED_FROM_KINDS * size * size],
            size,
        }
    }
    fn get(&self, row: usize, col: usize, arrived_from: usize) -> &Cell {
        let idx = arrived_from * self.size * self.size + col * self.size + row;
        self.cells.get(idx).unwrap()
    }
    fn get_mut(&mut self, row: usize, col: usize, arrived_from: usize) -> &mut Cell {
        let idx = arrived_from * self.size * self.size + col * self.size + row;
        self.cells.get_mut(idx).unwrap()
    }
    fn clear(&mut self) {
        for item in self.cells.iter_mut() {
            item.cost_so_far = 0;
            item.reached = false;
        }
    }
}

struct FrontierItem {
    coord: Coord,
    cost: u32,
}
impl Eq for FrontierItem {}
impl PartialEq for FrontierItem {
    fn eq(&self, other: &Self) -> bool {
        other.cost == self.cost
    }
}
impl Ord for FrontierItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for FrontierItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn search(grid: &mut Grid, grid_size: usize, get_neighbors: NeighborGetter) -> u32 {
    let max_coord = grid_size as u8 - 1;

    let mut frontier = BinaryHeap::<FrontierItem>::new();
    frontier.push(FrontierItem {
        coord: Coord(0, 1, ArrivedFrom::Left(1)),
        cost: grid.get(0, 1, ArrivedFrom::Left(1).to_idx()).cost as u32,
    });
    frontier.push(FrontierItem {
        coord: Coord(1, 0, ArrivedFrom::Top(1)),
        cost: grid.get(1, 0, ArrivedFrom::Top(1).to_idx()).cost as u32,
    });

    grid.get_mut(0, 1, ArrivedFrom::Left(1).to_idx())
        .cost_so_far = grid.get(0, 1, ArrivedFrom::Left(1).to_idx()).cost as u32;
    grid.get_mut(1, 0, ArrivedFrom::Top(1).to_idx()).cost_so_far =
        grid.get(1, 0, ArrivedFrom::Top(1).to_idx()).cost as u32;

    while let Some(cur) = frontier.pop() {
        if cur.coord.0 == max_coord && cur.coord.1 == max_coord {
            return cur.cost;
        }
        let cur_cell = grid
            .get(
                cur.coord.0 as usize,
                cur.coord.1 as usize,
                cur.coord.2.to_idx(),
            )
            .clone();

        for neighbor in get_neighbors(cur.coord, max_coord) {
            let next_cell = grid.get_mut(
                neighbor.0 as usize,
                neighbor.1 as usize,
                neighbor.2.to_idx(),
            );
            let new_cost = cur_cell.cost_so_far + next_cell.cost as u32;

            if !next_cell.reached || new_cost < next_cell.cost_so_far {
                next_cell.cost_so_far = new_cost;
                next_cell.reached = true;
                frontier.push(FrontierItem {
                    coord: neighbor,
                    cost: new_cost,
                });
            }
        }
    }

    panic!()
}

fn get_neighbors<const MIN_STEPS: u8, const MAX_STEPS: u8>(
    coord: Coord,
    max_coord: u8,
) -> ArrayVec<Coord, 3> {
    let mut ret = ArrayVec::new();
    match coord.2 {
        ArrivedFrom::Left(x) => {
            if x < MIN_STEPS {
                if coord.1 < max_coord {
                    ret.push(Coord(coord.0, coord.1 + 1, ArrivedFrom::Left(x + 1)));
                }
                return ret;
            }
            if coord.1 < max_coord && x < MAX_STEPS {
                ret.push(Coord(coord.0, coord.1 + 1, ArrivedFrom::Left(x + 1)));
            }
            if coord.0 > 0 {
                ret.push(Coord(coord.0 - 1, coord.1, ArrivedFrom::Bottom(1)));
            }
            if coord.0 < max_coord {
                ret.push(Coord(coord.0 + 1, coord.1, ArrivedFrom::Top(1)));
            }
        }
        ArrivedFrom::Right(x) => {
            if x < MIN_STEPS {
                if coord.1 > 0 {
                    ret.push(Coord(coord.0, coord.1 - 1, ArrivedFrom::Right(x + 1)));
                }
                return ret;
            }
            if coord.1 > 0 && x < MAX_STEPS {
                ret.push(Coord(coord.0, coord.1 - 1, ArrivedFrom::Right(x + 1)));
            }
            if coord.0 > 0 {
                ret.push(Coord(coord.0 - 1, coord.1, ArrivedFrom::Bottom(1)));
            }
            if coord.0 < max_coord {
                ret.push(Coord(coord.0 + 1, coord.1, ArrivedFrom::Top(1)));
            }
        }
        ArrivedFrom::Top(x) => {
            if x < MIN_STEPS {
                if coord.0 < max_coord {
                    ret.push(Coord(coord.0 + 1, coord.1, ArrivedFrom::Top(x + 1)));
                }
                return ret;
            }
            if coord.0 < max_coord && x < MAX_STEPS {
                ret.push(Coord(coord.0 + 1, coord.1, ArrivedFrom::Top(x + 1)));
            }
            if coord.1 > 0 {
                ret.push(Coord(coord.0, coord.1 - 1, ArrivedFrom::Right(1)));
            }
            if coord.1 < max_coord {
                ret.push(Coord(coord.0, coord.1 + 1, ArrivedFrom::Left(1)));
            }
        }
        ArrivedFrom::Bottom(x) => {
            if x < MIN_STEPS {
                if coord.0 > 0 {
                    ret.push(Coord(coord.0 - 1, coord.1, ArrivedFrom::Bottom(x + 1)));
                }
                return ret;
            }
            if coord.0 > 0 && x < MAX_STEPS {
                ret.push(Coord(coord.0 - 1, coord.1, ArrivedFrom::Bottom(x + 1)));
            }
            if coord.1 > 0 {
                ret.push(Coord(coord.0, coord.1 - 1, ArrivedFrom::Right(1)));
            }
            if coord.1 < max_coord {
                ret.push(Coord(coord.0, coord.1 + 1, ArrivedFrom::Left(1)));
            }
        }
    }
    ret
}
