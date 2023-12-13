use glam::{ivec2, IVec2};
use std::{fmt::Write, mem::zeroed};

const GRID_SIZE: usize = 140;

const FLAG_LEFT: u8 = 1;
const FLAG_RIGHT: u8 = 2;
const FLAG_UP: u8 = 4;
const FLAG_DOWN: u8 = 8;
const FLAG_ON_LOOP: u8 = 16;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };

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
    let result_1 = part_2(&grid);

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

#[rustfmt::skip]
fn part_2(grid: &Grid) -> u32 {
    let mut inside = false;
    let mut edge_from_left = false;
    let mut edge_from_right = false;
    let mut count = 0;

    for grid_x in grid.iter() {
        for cell in grid_x.iter() {
            if cell & (FLAG_DOWN | FLAG_RIGHT | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_RIGHT | FLAG_ON_LOOP {
                edge_from_right = true;
            } else if cell & (FLAG_DOWN | FLAG_UP | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_UP | FLAG_ON_LOOP {
                // noop
            } else if cell & (FLAG_DOWN | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_DOWN | FLAG_LEFT | FLAG_ON_LOOP {
                edge_from_left = true;
            } else if cell & (FLAG_UP | FLAG_RIGHT | FLAG_ON_LOOP) == FLAG_UP | FLAG_RIGHT | FLAG_ON_LOOP {
                inside ^= edge_from_left;
                edge_from_left = false;
                edge_from_right = false;
            } else if cell & (FLAG_UP | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_UP | FLAG_LEFT | FLAG_ON_LOOP {
                inside ^= edge_from_right;
                edge_from_left = false;
                edge_from_right = false;
            } else if cell & (FLAG_RIGHT | FLAG_LEFT | FLAG_ON_LOOP) == FLAG_RIGHT | FLAG_LEFT | FLAG_ON_LOOP {
                inside = !inside;
            } else if inside {
                count += 1;
            }
        }
    }

    count
}
