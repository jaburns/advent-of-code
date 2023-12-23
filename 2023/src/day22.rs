use arrayvec::ArrayVec;
use std::{fmt::Write, mem::zeroed};

const GRID_SIZE_X: usize = 10;
const GRID_SIZE_Y: usize = 10;
const GRID_SIZE_Z: usize = 310;

const MAX_BLOCKS: usize = 1273;
const MAX_SUPPORTS: usize = 10;

type Grid = [[[u16; GRID_SIZE_Z]; GRID_SIZE_Y]; GRID_SIZE_X];

#[derive(Debug, Default)]
struct Block {
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
    min_z: u16,
    max_z: u16,
    supported_by: ArrayVec<u16, MAX_SUPPORTS>,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut grid: Grid = unsafe { zeroed() };
    let mut blocks = ArrayVec::<Block, MAX_BLOCKS>::new();

    for line in lines.iter() {
        let mut parts = line.split('~');

        let mut a = parts.next().unwrap().split(',');
        let ax = a.next().unwrap().parse::<usize>().unwrap();
        let ay = a.next().unwrap().parse::<usize>().unwrap();
        let az = a.next().unwrap().parse::<usize>().unwrap();

        let mut b = parts.next().unwrap().split(',');
        let bx = b.next().unwrap().parse::<usize>().unwrap();
        let by = b.next().unwrap().parse::<usize>().unwrap();
        let bz = b.next().unwrap().parse::<usize>().unwrap();

        let block = Block {
            min_x: ax.min(bx) as u8,
            max_x: ax.max(bx) as u8,
            min_y: ay.min(by) as u8,
            max_y: ay.max(by) as u8,
            min_z: az.min(bz) as u16,
            max_z: az.max(bz) as u16,
            supported_by: ArrayVec::default(),
        };

        blocks.push(block);
    }

    blocks.sort_unstable_by_key(|x| x.min_z);

    for (i, block) in blocks.iter().enumerate() {
        let block_id = i as u16 + 1;

        for x in block.min_x..=block.max_x {
            for y in block.min_y..=block.max_y {
                for z in block.min_z..=block.max_z {
                    grid[x as usize][y as usize][z as usize] = block_id;
                }
            }
        }
    }

    loop {
        let mut dropped_one = false;

        for (i, block) in blocks.iter_mut().enumerate() {
            let block_id = i as u16 + 1;

            if block.min_z <= 1 {
                continue;
            }

            let mut can_fall = true;
            'outer: for x in block.min_x..=block.max_x {
                for y in block.min_y..=block.max_y {
                    let block_id_below = grid[x as usize][y as usize][block.min_z as usize - 1];
                    if block_id_below != 0 {
                        can_fall = false;
                        break 'outer;
                    }
                }
            }

            if can_fall {
                for x in block.min_x..=block.max_x {
                    for y in block.min_y..=block.max_y {
                        // assert_eq!(grid[x as usize][y as usize][block.min_z as usize], block_id);
                        grid[x as usize][y as usize][block.min_z as usize - 1] = block_id;
                        grid[x as usize][y as usize][block.max_z as usize] = 0;
                    }
                }
                block.min_z -= 1;
                block.max_z -= 1;
                dropped_one = true;
            }
        }

        if !dropped_one {
            break;
        }
    }

    for block in blocks.iter_mut() {
        for x in block.min_x..=block.max_x {
            for y in block.min_y..=block.max_y {
                let block_id_below = grid[x as usize][y as usize][block.min_z as usize - 1];
                if block_id_below != 0 && !block.supported_by.contains(&block_id_below) {
                    block.supported_by.push(block_id_below);
                }
            }
        }
    }

    let mut result_0 = 0;
    let mut result_1 = 0;
    let mut fallen = [false; MAX_BLOCKS];

    for block in blocks.iter() {
        if block.supported_by.len() != 1 {
            continue;
        }

        let j = block.supported_by[0] as usize - 1;

        if fallen[j] {
            continue;
        }

        let mut cascade = [false; MAX_BLOCKS];
        let mut fell_one = true;
        cascade[j] = true;

        while fell_one {
            fell_one = false;
            for (k, block) in blocks.iter().enumerate() {
                if cascade[k] {
                    continue;
                }
                let mut should_fall = !block.supported_by.is_empty();
                for supported_by_id in block.supported_by.iter() {
                    if !cascade[*supported_by_id as usize - 1] {
                        should_fall = false;
                        break;
                    }
                }
                if should_fall {
                    fell_one = true;
                    cascade[k] = true;
                    result_1 += 1;
                }
            }
        }

        fallen[j] = true;
        result_0 += 1;
    }

    let result_0 = blocks.len() - result_0;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}
