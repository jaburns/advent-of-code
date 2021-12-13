use std::fmt::Write;

#[derive(Debug)]
enum Fold {
    AlongX(u16),
    AlongY(u16),
}

#[derive(Debug)]
struct Inputs {
    grid: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

fn parse_inputs(lines: &[&str]) -> Inputs {
    let mut w = 0_usize;
    let mut h = 0_usize;
    let mut coords = vec![];
    let mut folds = vec![];

    for line in lines.iter() {
        if line.contains(',') {
            let mut iter = line.split(',');

            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();

            w = w.max(x);
            h = h.max(y);

            coords.push((x, y));
        } else if line.contains("fold") {
            let coord = line[13..].parse::<u16>().unwrap();

            if line.contains("x=") {
                folds.push(Fold::AlongX(coord));
            } else {
                folds.push(Fold::AlongY(coord));
            }
        }
    }

    let mut grid = vec![vec![false; h + 1]; w + 1];
    for coord in coords {
        grid[coord.0][coord.1] = true;
    }

    Inputs { grid, folds }
}

fn apply_fold_to_grid(grid: &mut Vec<Vec<bool>>, fold: &Fold) {
    match &fold {
        Fold::AlongX(col) => {
            for x in 0..(*col as usize) {
                for y in 0..grid[0].len() {
                    grid[x][y] = grid[x][y] || grid[grid.len() - 1 - x][y]
                }
            }
            grid.resize(*col as usize, vec![]);
        }
        Fold::AlongY(row) => {
            for x in 0..grid.len() {
                for y in 0..(*row as usize) {
                    grid[x][y] = grid[x][y] || grid[x][grid[0].len() - 1 - y]
                }
                grid[x].resize(*row as usize, false);
            }
        }
    }
}

fn count_dots(grid: &[Vec<bool>]) -> u32 {
    let mut count = 0_u32;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            count += grid[x][y] as u32;
        }
    }

    count
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut inputs = parse_inputs(lines);
    apply_fold_to_grid(&mut inputs.grid, &inputs.folds[0]);
    let result = count_dots(&inputs.grid);
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
