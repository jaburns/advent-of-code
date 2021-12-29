use std::fmt::Write;

const BORDER: usize = 100;

fn sample_grid(grid: &[Vec<bool>], mut row: isize, mut col: isize) -> bool {
    if row < 0 {
        row = 0;
    }
    if col < 0 {
        col = 0;
    }
    if row >= grid.len() as isize {
        row = grid.len() as isize - 1;
    }
    if col >= grid[0].len() as isize {
        col = grid[0].len() as isize - 1;
    }
    grid[row as usize][col as usize]
}

fn run_rule_on_grid(rule: &[bool], grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();

    for r in 0..grid.len() as isize {
        for c in 0..grid[0].len() as isize {
            let bits = [
                sample_grid(grid, r - 1, c - 1),
                sample_grid(grid, r - 1, c),
                sample_grid(grid, r - 1, c + 1),
                sample_grid(grid, r, c - 1),
                sample_grid(grid, r, c),
                sample_grid(grid, r, c + 1),
                sample_grid(grid, r + 1, c - 1),
                sample_grid(grid, r + 1, c),
                sample_grid(grid, r + 1, c + 1),
            ];

            let val = (bits[0] as usize * 0b100000000)
                | (bits[1] as usize * 0b010000000)
                | (bits[2] as usize * 0b001000000)
                | (bits[3] as usize * 0b000100000)
                | (bits[4] as usize * 0b000010000)
                | (bits[5] as usize * 0b000001000)
                | (bits[6] as usize * 0b000000100)
                | (bits[7] as usize * 0b000000010)
                | bits[8] as usize;

            new_grid[r as usize][c as usize] = rule[val];
        }
    }

    grid.clone_from(&new_grid);
}

fn solve(lines: &[&str], out: &mut String, iter: u32) {
    let mut grid = vec![];
    let size = lines[2].len();

    for _ in 0..BORDER {
        grid.push(vec![false; size + 2 * BORDER]);
    }
    for line in lines.iter().skip(2) {
        let mut cur_row = vec![false; BORDER];
        for ch in line.chars() {
            cur_row.push(ch == '#');
        }
        cur_row.resize(size + 2 * BORDER, false);
        grid.push(cur_row);
    }
    for _ in 0..BORDER {
        grid.push(vec![false; size + 2 * BORDER]);
    }

    let rule = lines[0].chars().map(|x| x == '#').collect::<Vec<_>>();

    for _ in 0..iter {
        run_rule_on_grid(&rule, &mut grid);
    }

    let count = grid
        .iter()
        .map(|x| x.iter().map(|x| *x as u32).sum::<u32>())
        .sum::<u32>();

    write!(out, "{}", count).unwrap();
}

pub fn part1(lines: &[&str], out: &mut String) {
    solve(lines, out, 2);
}

pub fn part2(lines: &[&str], out: &mut String) {
    solve(lines, out, 50);
}
