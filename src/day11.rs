#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Floor,
    EmptySeat,
    FilledSeat,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        match ch {
            '.' => Cell::Floor,
            'L' => Cell::EmptySeat,
            '#' => Cell::FilledSeat,
            _ => panic!(),
        }
    }
}

type CountFn = fn(&Vec<Vec<Cell>>, i32, i32) -> u32;

fn sample_grid(grid: &Vec<Vec<Cell>>, y: i32, x: i32) -> Option<Cell> {
    if y < 0 {
        return None;
    };
    if x < 0 {
        return None;
    };
    if y as usize >= grid.len() {
        return None;
    }
    if x as usize >= grid[y as usize].len() {
        return None;
    }
    Some(grid[y as usize][x as usize])
}

fn count_neighbors_simple(grid: &Vec<Vec<Cell>>, y: i32, x: i32) -> u32 {
    let mut occupied = 0u32;

    for dy in -1i32..=1i32 {
        for dx in -1i32..=1i32 {
            if dy == 0 && dx == 0 {
                continue;
            }
            if sample_grid(grid, y as i32 + dy, x as i32 + dx) == Some(Cell::FilledSeat) {
                occupied += 1;
            }
        }
    }

    occupied
}

fn count_neighbors_visible(grid: &Vec<Vec<Cell>>, y: i32, x: i32) -> u32 {
    let mut occupied = 0u32;

    for dy in -1i32..=1i32 {
        for dx in -1i32..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let mut sy = y;
            let mut sx = x;
            loop {
                sy += dy;
                sx += dx;

                match sample_grid(grid, sy, sx) {
                    Some(Cell::FilledSeat) => {
                        occupied += 1;
                        break;
                    }
                    Some(Cell::Floor) => continue,
                    _ => break,
                }
            }
        }
    }

    occupied
}

fn step_grid(
    prev_grid: &Vec<Vec<Cell>>,
    grid: &mut Vec<Vec<Cell>>,
    counter: CountFn,
    max_neighbors: u32,
) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = counter(prev_grid, y as i32, x as i32);

            if grid[y][x] == Cell::FilledSeat && neighbors >= max_neighbors {
                grid[y][x] = Cell::EmptySeat;
            }

            if grid[y][x] == Cell::EmptySeat && neighbors < 1 {
                grid[y][x] = Cell::FilledSeat;
            }
        }
    }
}

fn count_seats_eventually_full(grid: &Vec<Vec<Cell>>, counter: CountFn, max_neighbors: u32) -> u32 {
    let mut grid = grid.clone();

    loop {
        let prev_grid = grid.clone();
        step_grid(&prev_grid, &mut grid, counter, max_neighbors);
        if prev_grid == grid {
            break;
        }
    }

    grid.iter()
        .map(|x| {
            x.iter()
                .map(|x| (*x == Cell::FilledSeat) as u32)
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn main() {
    let grid = std::fs::read_to_string("data/day11.smol.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(Cell::new).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = count_seats_eventually_full(&grid, count_neighbors_simple, 4);
    let part2 = count_seats_eventually_full(&grid, count_neighbors_visible, 5);

    println!("{} {}", part1, part2);
}
