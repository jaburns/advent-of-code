use arrayvec::ArrayVec;
use glam::{dvec2, dvec3, DVec2, DVec3};
use nalgebra::{DMatrix, DVector};
use std::fmt::Write;

const MAX_LINES: usize = 300;

#[derive(Default, Debug, Copy, Clone)]
struct Line2 {
    pos: DVec2,
    vel: DVec2,
}

#[derive(Default, Debug, Copy, Clone)]
struct Line3 {
    pos: DVec3,
    vel: DVec3,
}

pub fn parts_1_and_2(inputs: &[&str], out: &mut String) {
    let mut lines2 = ArrayVec::<Line2, MAX_LINES>::new();
    let mut lines3 = ArrayVec::<Line3, MAX_LINES>::new();

    for input in inputs {
        let mut sides = input.split(" @ ");
        let mut lefts = sides
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim_start().parse::<f64>().unwrap());
        let mut rights = sides
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim_start().parse::<f64>().unwrap());
        let pt = (
            lefts.next().unwrap(),
            lefts.next().unwrap(),
            lefts.next().unwrap(),
        );
        let dir = (
            rights.next().unwrap(),
            rights.next().unwrap(),
            rights.next().unwrap(),
        );
        lines2.push(Line2 {
            pos: dvec2(pt.0, pt.1),
            vel: dvec2(dir.0, dir.1),
        });
        lines3.push(Line3 {
            pos: dvec3(pt.0, pt.1, pt.2),
            vel: dvec3(dir.0, dir.1, dir.2),
        });
    }

    let (test_min, test_max) = if inputs.len() == MAX_LINES {
        (200000000000000_f64, 400000000000000_f64)
    } else {
        (7_f64, 27_f64)
    };

    let mut result_0 = 0;
    for i in 0..(lines2.len() - 1) {
        for j in (i + 1)..lines2.len() {
            let a = lines2[i];
            let b = lines2[j];
            if let Some(pt) = find_intersection(a, b) {
                if pt.x >= test_min && pt.x <= test_max && pt.y >= test_min && pt.y <= test_max {
                    result_0 += 1;
                }
            }
        }
    }

    let part_2_line = solve_part_2_line((lines3[0], lines3[1], lines3[2]));
    let result_1 = (part_2_line.pos.x + part_2_line.pos.y + part_2_line.pos.z).round();
    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn solve_part_2_line(lines: (Line3, Line3, Line3)) -> Line3 {
    let a = lines.1.vel - lines.0.vel;
    let b = lines.0.pos - lines.1.pos;
    let c = lines.1.pos.cross(lines.1.vel) - lines.0.pos.cross(lines.0.vel);
    let d = lines.2.vel - lines.1.vel;
    let e = lines.1.pos - lines.2.pos;
    let f = lines.2.pos.cross(lines.2.vel) - lines.1.pos.cross(lines.1.vel);

    #[rustfmt::skip]
    let (mat, rhs) = (
        DMatrix::from_row_slice(6, 6, &[
             0.0,  a.z, -a.y,  0.0,  b.z, -b.y,
            -a.z,  0.0,  a.x, -b.z,  0.0,  b.x,
             a.y, -a.x,  0.0,  b.y, -b.x,  0.0,
             0.0,  d.z, -d.y,  0.0,  e.z, -e.y,
            -d.z,  0.0,  d.x, -e.z,  0.0,  e.x,
             d.y, -d.x,  0.0,  e.y, -e.x,  0.0,
        ]),
        DVector::from_column_slice(&[
            c.x, c.y, c.z, f.x, f.y, f.z,
        ])
    );

    let solution = mat.lu().solve(&rhs).unwrap();

    Line3 {
        pos: DVec3::from_slice(&solution.data.as_slice()[0..3]),
        vel: DVec3::from_slice(&solution.data.as_slice()[3..6]),
    }
}

fn find_intersection(a: Line2, b: Line2) -> Option<DVec2> {
    let det = a.vel.perp_dot(b.vel);
    if det.abs() < f64::EPSILON {
        return None;
    }

    let b_from_a = b.pos - a.pos;
    let t0 = (b_from_a).perp_dot(b.vel) / det;
    let t1 = (-b_from_a).perp_dot(a.vel) / (-det);

    if t0 > 0.0 && t1 > 0.0 {
        Some(a.pos + t0 * a.vel)
    } else {
        None
    }
}
