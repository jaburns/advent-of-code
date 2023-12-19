use arrayvec::ArrayVec;
use glam::DVec2;
use std::fmt::Write;

const MAX_COORDS: usize = 1024;

type CoordsVec = ArrayVec<DVec2, MAX_COORDS>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let result_0 = parse_and_solve_part1(lines);
    let result_1 = parse_and_solve_part2(lines);
    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn parse_and_solve_part1(lines: &[&str]) -> i64 {
    let mut winding = 0_i32;
    let mut prev = &lines[lines.len() - 1][0..1];
    let mut coords = CoordsVec::new();
    let mut bounary_cells = 0;
    let mut coord = DVec2::ZERO;

    for line in lines {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap().parse::<i64>().unwrap();

        bounary_cells += dist;
        winding += get_wind(dir, prev);
        prev = dir;

        match dir {
            "R" => coord.x += dist as f64,
            "L" => coord.x -= dist as f64,
            "D" => coord.y += dist as f64,
            "U" => coord.y -= dist as f64,
            _ => panic!(),
        }

        coords.push(coord);
    }

    let boundary_area = bounary_cells / 2 + 1;
    let inner_area = clip_and_measure_area(&mut coords, winding > 0);

    inner_area as i64 + boundary_area
}

fn parse_and_solve_part2(lines: &[&str]) -> i64 {
    let mut winding = 0_i32;
    let (mut prev, _) =
        translate_part2_coords(lines[lines.len() - 1].split_whitespace().nth(2).unwrap());
    let mut coords = CoordsVec::new();
    let mut bounary_cells = 0;
    let mut coord = DVec2::ZERO;

    for line in lines {
        let (dir, dist) = translate_part2_coords(line.split_whitespace().nth(2).unwrap());

        bounary_cells += dist;
        winding += get_wind(dir, prev);
        prev = dir;

        match dir {
            "R" => coord.x += dist as f64,
            "L" => coord.x -= dist as f64,
            "D" => coord.y += dist as f64,
            "U" => coord.y -= dist as f64,
            _ => panic!(),
        }

        coords.push(coord);
    }

    let boundary_area = bounary_cells / 2 + 1;
    let inner_area = clip_and_measure_area(&mut coords, winding > 0);

    inner_area as i64 + boundary_area
}

fn translate_part2_coords(color: &str) -> (&'static str, i64) {
    let dist = i64::from_str_radix(&color[2..7], 16).unwrap();
    let dir = match &color[7..8] {
        "0" => "R",
        "1" => "D",
        "2" => "L",
        "3" => "U",
        _ => panic!(),
    };

    (dir, dist)
}

fn get_wind(prev: &str, cur: &str) -> i32 {
    match cur {
        "R" => {
            if prev == "U" {
                1
            } else if prev == "D" {
                -1
            } else {
                panic!()
            }
        }
        "L" => {
            if prev == "U" {
                -1
            } else if prev == "D" {
                1
            } else {
                panic!()
            }
        }
        "D" => {
            if prev == "L" {
                -1
            } else if prev == "R" {
                1
            } else {
                panic!()
            }
        }
        "U" => {
            if prev == "L" {
                1
            } else if prev == "R" {
                -1
            } else {
                panic!()
            }
        }
        _ => panic!(),
    }
}

fn clip_and_measure_area(coords: &mut CoordsVec, winding_positive: bool) -> f64 {
    let mut total_area = 0_f64;

    let mut i = 0_usize;
    while coords.len() > 3 {
        i %= coords.len();
        let j = (i + 1) % coords.len();
        let k = (i + 2) % coords.len();

        let a = coords[i];
        let b = coords[j];
        let c = coords[k];

        let cross = (c - a).perp_dot(b - a);
        let is_convex = (cross > 0.0) == winding_positive;

        let mut clipped = false;
        if is_convex {
            let mut ear = true;
            for (n, &p) in coords.iter().enumerate() {
                if n != i && n != j && n != k && triangle_has_point(p, a, b, c) {
                    ear = false;
                    break;
                }
            }
            if ear {
                clipped = true;
                total_area += triangle_area(a, b, c);
                coords.remove(j);
            }
        }
        if !clipped {
            i += 1;
        }
    }

    total_area + triangle_area(coords[0], coords[1], coords[2])
}

fn triangle_area(a: DVec2, b: DVec2, c: DVec2) -> f64 {
    0.5 * ((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))).abs()
}

fn triangle_has_point(p: DVec2, a: DVec2, b: DVec2, c: DVec2) -> bool {
    let sign = |p1: DVec2, p2: DVec2, p3: DVec2| {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    };
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}
