use arrayvec::ArrayVec;
use glam::DVec2;
use std::fmt::Write;

const MAX_COORDS: usize = 1024;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut bounary_cells_0 = 0;
    let mut bounary_cells_1 = 0;
    let mut coords_0 = ArrayVec::<DVec2, MAX_COORDS>::new();
    let mut coords_1 = ArrayVec::<DVec2, MAX_COORDS>::new();
    let mut coord_0 = DVec2::ZERO;
    let mut coord_1 = DVec2::ZERO;

    for line in lines {
        let mut parts = line.split_whitespace();
        let dir_0 = parts.next().unwrap();
        let dist_0 = parts.next().unwrap().parse::<i64>().unwrap();

        let color = parts.next().unwrap();
        let dir_1 = &color[7..8];
        let dist_1 = i64::from_str_radix(&color[2..7], 16).unwrap();

        bounary_cells_0 += dist_0;
        bounary_cells_1 += dist_1;

        match dir_0 {
            "R" => coord_0.x += dist_0 as f64,
            "D" => coord_0.y += dist_0 as f64,
            "L" => coord_0.x -= dist_0 as f64,
            _u => coord_0.y -= dist_0 as f64,
        }
        match dir_1 {
            "0" => coord_1.x += dist_1 as f64,
            "1" => coord_1.y += dist_1 as f64,
            "2" => coord_1.x -= dist_1 as f64,
            _three => coord_1.y -= dist_1 as f64,
        }

        coords_0.push(coord_0);
        coords_1.push(coord_1);
    }

    let result_0 = bounary_cells_0 / 2 + 1 + measure_area(&coords_0).abs() as i64;
    let result_1 = bounary_cells_1 / 2 + 1 + measure_area(&coords_1).abs() as i64;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn measure_area(coords: &[DVec2]) -> f64 {
    0.5 * coords
        .iter()
        .zip(coords.iter().cycle().skip(1))
        .map(|(a, b)| (a.x + b.x) * (a.y - b.y))
        .sum::<f64>()
}
