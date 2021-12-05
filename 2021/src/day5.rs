use std::fmt::Write;

#[derive(Debug)]
struct VentLine {
    a: (i32, i32),
    b: (i32, i32),
}

impl VentLine {
    fn parse(line: &str) -> Self {
        let mut parts = line.split("->").map(|x| x.trim());

        let mut parts_a = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());

        let mut parts_b = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());

        Self {
            a: (parts_a.next().unwrap(), parts_a.next().unwrap()),
            b: (parts_b.next().unwrap(), parts_b.next().unwrap()),
        }
    }
}

const GRID_SIZE: usize = 1000;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE / 4];

fn draw_line_to_grid(grid: &mut Grid, line: &VentLine, diagonals: bool) -> u32 {
    if !diagonals && line.a.0 != line.b.0 && line.a.1 != line.b.1 {
        return 0;
    }

    let dx = line.b.0 - line.a.0;
    let dy = line.b.1 - line.a.1;
    let dx_step = dx.signum();
    let dy_step = dy.signum();

    let mut ix = line.a.0;
    let mut iy = line.a.1;
    let mut count = 0;

    for _ in 0..=dx.abs().max(dy.abs()) {
        let mut wide_val = grid[(ix >> 2) as usize][iy as usize];

        let mask = 0b11 << ((ix & 3) << 1);
        let mut val = wide_val & mask;
        val >>= (ix & 3) << 1;
        val += (val < 3) as u8;
        count += (val == 2) as u32;
        val <<= (ix & 3) << 1;

        wide_val &= !mask;
        wide_val |= val;

        grid[(ix >> 2) as usize][iy as usize] = wide_val;

        ix += dx_step;
        iy += dy_step;
    }

    count
}

fn run(lines: &[&str], out: &mut String, diagonals: bool) {
    let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE / 4];
    let mut count = 0u32;

    for line in lines {
        count += draw_line_to_grid(&mut grid, &VentLine::parse(line), diagonals);
    }

    write!(out, "{}", count).unwrap();
}

pub fn part1(lines: &[&str], out: &mut String) {
    run(lines, out, false);
}

pub fn part2(lines: &[&str], out: &mut String) {
    run(lines, out, true);
}
