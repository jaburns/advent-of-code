use glam::{ivec2, IVec2};
use std::{fmt::Write, mem::zeroed};

const GRID_SIZE: usize = 140;

const FLAG_LEFT: u8 = 1;
const FLAG_RIGHT: u8 = 2;
const FLAG_UP: u8 = 4;
const FLAG_DOWN: u8 = 8;
const FLAG_ON_LOOP: u8 = 16;

const P2_FLAG_TL: u8 = 1;
const P2_FLAG_TR: u8 = 2;
const P2_FLAG_BL: u8 = 4;
const P2_FLAG_BR: u8 = 8;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };
    let true_size = lines[0].len();

    let mut animal_pos = ivec2(0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            grid[x][y] = match chr {
                'S' => {
                    animal_pos = ivec2(x as i32, y as i32);
                    FLAG_ON_LOOP
                }
                'F' => FLAG_DOWN | FLAG_RIGHT,
                '|' => FLAG_DOWN | FLAG_UP,
                '7' => FLAG_DOWN | FLAG_LEFT,
                'L' => FLAG_UP | FLAG_RIGHT,
                'J' => FLAG_UP | FLAG_LEFT,
                '-' => FLAG_LEFT | FLAG_RIGHT,
                '.' => 0,
                _ => panic!(),
            };
        }
    }

    let result_0 = part_1(&mut grid, animal_pos);
    convert_grid(&mut grid);
    // debug_print_new_grid(&grid, true_size);
    let result_1 = part2(&mut grid, true_size);

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn part_1(grid: &mut Grid, animal_pos: IVec2) -> u32 {
    let mut counter = 0;
    let mut pos = IVec2::ZERO;
    let mut from = 0;

    let checks = [
        (FLAG_LEFT, FLAG_RIGHT, animal_pos + ivec2(1, 0)),
        (FLAG_UP, FLAG_DOWN, animal_pos + ivec2(0, 1)),
        (FLAG_RIGHT, FLAG_LEFT, animal_pos - ivec2(1, 0)),
        (FLAG_DOWN, FLAG_UP, animal_pos - ivec2(0, 1)),
    ];
    for (init_from, animal_flag, p) in checks {
        if p.x >= GRID_SIZE as i32 || p.y >= GRID_SIZE as i32 || p.x < 0 || p.y < 0 {
            continue;
        }
        if grid[p.x as usize][p.y as usize] & init_from != 0 {
            grid[animal_pos.x as usize][animal_pos.y as usize] |= animal_flag;
            pos = p;
            from = init_from;
            counter += 1;
            break;
        }
    }

    loop {
        if pos == animal_pos {
            grid[animal_pos.x as usize][animal_pos.y as usize] |= from;
            break;
        }
        let target_flags = &mut grid[pos.x as usize][pos.y as usize];
        let out_flag = *target_flags & !from;
        if out_flag == FLAG_LEFT {
            pos.x -= 1;
            from = FLAG_RIGHT;
        } else if out_flag == FLAG_RIGHT {
            pos.x += 1;
            from = FLAG_LEFT;
        } else if out_flag == FLAG_UP {
            pos.y -= 1;
            from = FLAG_DOWN;
        } else if out_flag == FLAG_DOWN {
            pos.y += 1;
            from = FLAG_UP;
        }
        *target_flags |= FLAG_ON_LOOP;
        counter += 1;
    }

    counter / 2
}

// New format for each byte cell: each nibble is a 2x2 grid given by P2_FLAG_*, lower nibble is walls, upper nibble is fill
#[rustfmt::skip]
#[allow(clippy::needless_range_loop, clippy::if_same_then_else)]
fn convert_grid(grid: &mut Grid) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let cell = grid[x][y];
            if cell & (FLAG_DOWN | FLAG_RIGHT | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_RIGHT | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL | P2_FLAG_TR | P2_FLAG_BL;
            } else if cell & (FLAG_DOWN | FLAG_UP | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_UP | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL | P2_FLAG_BL;
            } else if cell & (FLAG_DOWN | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_LEFT | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL | P2_FLAG_BL;
            } else if cell & (FLAG_UP | FLAG_RIGHT | FLAG_ON_LOOP) == FLAG_UP | FLAG_RIGHT | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL | P2_FLAG_TR;
            } else if cell & (FLAG_UP | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_UP | FLAG_LEFT | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL;
            } else if cell & (FLAG_RIGHT | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_RIGHT | FLAG_LEFT | FLAG_ON_LOOP {
                grid[x][y] = P2_FLAG_TL | P2_FLAG_TR;
            } else {
                grid[x][y] = 0;
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Cursor {
    pos: IVec2,
    inner: u8,
}
impl Cursor {
    fn left(mut self) -> Self {
        if self.inner & (P2_FLAG_TL | P2_FLAG_BL) != 0 {
            self.inner <<= 1;
            self.pos.x -= 1;
        } else {
            self.inner >>= 1;
        }
        self
    }
    fn right(mut self) -> Self {
        if self.inner & (P2_FLAG_TR | P2_FLAG_BR) != 0 {
            self.inner >>= 1;
            self.pos.x += 1;
        } else {
            self.inner <<= 1;
        }
        self
    }
    fn up(mut self) -> Self {
        if self.inner & (P2_FLAG_TL | P2_FLAG_TR) != 0 {
            self.inner <<= 2;
            self.pos.y -= 1;
        } else {
            self.inner >>= 2;
        }
        self
    }
    fn down(mut self) -> Self {
        if self.inner & (P2_FLAG_BL | P2_FLAG_BR) != 0 {
            self.inner >>= 2;
            self.pos.y += 1;
        } else {
            self.inner <<= 2;
        }
        self
    }
}

fn part2(grid: &mut Grid, true_size: usize) -> u32 {
    fn flood(grid: &mut Grid, cursor: Cursor) {
        let cell = &mut grid[cursor.pos.x as usize][cursor.pos.y as usize];

        if (*cell & (cursor.inner | (cursor.inner << 4))) != 0 {
            return;
        }

        *cell |= cursor.inner << 4;
        flood(grid, cursor.left());
        flood(grid, cursor.right());
        flood(grid, cursor.up());
        flood(grid, cursor.down());
    }

    flood(
        grid,
        Cursor {
            pos: IVec2::splat(true_size as i32 / 2),
            inner: P2_FLAG_BR,
        },
    );

    let mut count = 0;
    for y in 0..true_size {
        for grid_x in grid.iter().take(true_size) {
            let cell = grid_x[y];
            count += (cell & 0xf0 == 0xf0) as u32;
        }
    }

    count
}

#[allow(unused)]
fn debug_print_new_grid(grid: &Grid, true_size: usize) {
    for y in 0..true_size {
        let mut line0 = String::new();
        let mut line1 = String::new();
        for grid_x in grid.iter().take(true_size) {
            let cell = grid_x[y];
            line0 += if cell & P2_FLAG_TL != 0 { "X" } else { " " };
            line0 += if cell & P2_FLAG_TR != 0 { "X" } else { " " };
            line1 += if cell & P2_FLAG_BL != 0 { "X" } else { " " };
            line1 += if cell & P2_FLAG_BR != 0 { "X" } else { " " };
        }
        println!("{line0}");
        println!("{line1}");
    }
}
