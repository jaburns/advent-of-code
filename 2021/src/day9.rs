use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let grid = parse_grid(lines);
    let result: u32 = find_low_points(&grid).iter().map(|(_, _, risk)| risk).sum();
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut grid = parse_grid(lines);
    let low_points = find_low_points(&grid);

    let mut basins: Vec<u32> = low_points
        .iter()
        .map(|(x, y, _)| fill_and_measure_basin(&mut grid, *x, *y))
        .collect();

    basins.sort_unstable();

    let result = basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap();

    write!(out, "{}", result).unwrap();
}

fn parse_grid(lines: &[&str]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn find_low_points(grid: &[Vec<u8>]) -> Vec<(usize, usize, u32)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut result = vec![];

    for iy in 0..height {
        for ix in 0..width {
            let left = if ix > 0 { grid[iy][ix - 1] } else { 255 };
            let right = if ix < width - 1 {
                grid[iy][ix + 1]
            } else {
                255
            };
            let top = if iy > 0 { grid[iy - 1][ix] } else { 255 };
            let bottom = if iy < height - 1 {
                grid[iy + 1][ix]
            } else {
                255
            };
            let this = grid[iy][ix];

            if this < left && this < right && this < top && this < bottom {
                result.push((ix, iy, 1 + this as u32));
            }
        }
    }

    result
}

fn fill_and_measure_basin(grid: &mut [Vec<u8>], x: usize, y: usize) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let this = grid[y][x];
    if this >= 9 {
        return 0;
    }

    grid[y][x] = 9;

    let mut result = 1;
    if x > 0 {
        result += fill_and_measure_basin(grid, x - 1, y);
    }
    if x < width - 1 {
        result += fill_and_measure_basin(grid, x + 1, y);
    }
    if y > 0 {
        result += fill_and_measure_basin(grid, x, y - 1);
    }
    if y < height - 1 {
        result += fill_and_measure_basin(grid, x, y + 1);
    }
    result
}
