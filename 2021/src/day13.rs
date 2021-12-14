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

            coords.push((x, y));
        } else if line.contains("fold") {
            let coord = line[13..].parse::<u16>().unwrap();

            if line.contains("x=") {
                if w == 0 {
                    w = 2 * coord as usize + 1;
                }
                folds.push(Fold::AlongX(coord));
            } else {
                if h == 0 {
                    h = 2 * coord as usize + 1;
                }
                folds.push(Fold::AlongY(coord));
            }
        }
    }

    let mut grid = vec![vec![false; h]; w];
    for coord in coords {
        grid[coord.0][coord.1] = true;
    }

    Inputs { grid, folds }
}

fn apply_fold_to_grid(grid: &mut Vec<Vec<bool>>, fold: &Fold) {
    let mut new_grid = grid.clone();

    match &fold {
        Fold::AlongX(col) => {
            for x in 0..(*col as usize) {
                for y in 0..grid[0].len() {
                    new_grid[x][y] = grid[x][y] || grid[grid.len() - 1 - x][y]
                }
            }
            new_grid.resize(*col as usize, vec![]);
        }
        Fold::AlongY(row) => {
            for x in 0..grid.len() {
                for y in 0..(*row as usize) {
                    new_grid[x][y] = grid[x][y] || grid[x][grid[0].len() - 1 - y]
                }
                new_grid[x].resize(*row as usize, false);
            }
        }
    }

    grid.clone_from(&new_grid);
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut inputs = parse_inputs(lines);

    apply_fold_to_grid(&mut inputs.grid, &inputs.folds[0]);

    let mut result = 0_u32;

    for x in 0..inputs.grid.len() {
        for y in 0..inputs.grid[0].len() {
            result += inputs.grid[x][y] as u32;
        }
    }

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut inputs = parse_inputs(lines);

    #[allow(clippy::never_loop)]
    for fold in &inputs.folds {
        apply_fold_to_grid(&mut inputs.grid, fold);
    }

    let mut print = String::from("\n");

    for y in 0..inputs.grid[0].len() {
        for x in 0..inputs.grid.len() {
            print += if inputs.grid[x][y] { "#" } else { " " };
        }
        print += "\n";
    }

    write!(out, "{}", print).unwrap();
}
