use arrayvec::ArrayVec;
use derive_more::{Add, AddAssign};
use glam::{dvec2, dvec3, dvec4, vec3a, DVec2, DVec3, DVec4, Vec3A};
use std::fmt::Write;

const MAX_LINES: usize = 300;

#[derive(Default, Debug, Copy, Clone)]
struct Line2 {
    pt: DVec2,
    dir: DVec2,
}

#[derive(Default, Debug, Copy, Clone)]
struct Line4 {
    pt: DVec4,
    dir: DVec4,
}

#[derive(Default, Debug, Copy, Clone, Add, AddAssign)]
struct LVec3(i64, i64, i64);

pub fn parts_1_and_2(inputs: &[&str], out: &mut String) {
    let mut lines2 = ArrayVec::<Line2, MAX_LINES>::new();
    let mut lines4 = ArrayVec::<Line4, MAX_LINES>::new();

    for input in inputs {
        let mut sides = input.split(" @ ");
        let mut lefts = sides
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim_start().parse::<i64>().unwrap());
        let mut rights = sides
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim_start().parse::<i64>().unwrap());
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
            pt: dvec2(pt.0 as f64, pt.1 as f64),
            dir: dvec2(dir.0 as f64, dir.1 as f64),
        });
        lines4.push(Line4 {
            pt: dvec4(pt.0 as f64, pt.1 as f64, pt.2 as f64, 0.0),
            dir: dvec4(dir.0 as f64, dir.1 as f64, dir.2 as f64, 1.0),
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

    let mut test_pos = DVec3::ZERO;
    let mut test_vel = DVec3::ZERO;

    let measure_avg_distance = |pos: DVec3, vel: DVec3| -> f64 {
        let test = line4(pos.extend(0.0), vel.extend(1.0));
        let mut result = 0.0;
        for (i, line) in lines4.iter().enumerate() {
            result += min_dist_sqr(*line, test).sqrt() / lines4.len() as f64;
        }
        result
    };

    loop {
        let base_dist = measure_avg_distance(test_pos, test_vel);

        let mut dx = base_dist - measure_avg_distance(test_pos + DVec3::X, test_vel);
        let mut dy = base_dist - measure_avg_distance(test_pos + DVec3::Y, test_vel);
        let mut dz = base_dist - measure_avg_distance(test_pos + DVec3::Z, test_vel);
        dx *= base_dist / 1_000.0;
        dy *= base_dist / 1_000.0;
        dz *= base_dist / 1_000.0;
        test_pos += dvec3(dx, dy, dz);

        let mut dx = (base_dist - measure_avg_distance(test_pos, test_vel + DVec3::X));
        let mut dy = (base_dist - measure_avg_distance(test_pos, test_vel + DVec3::Y));
        let mut dz = (base_dist - measure_avg_distance(test_pos, test_vel + DVec3::Z));
        dx /= base_dist;
        dy /= base_dist;
        dz /= base_dist;
        test_vel += dvec3(dx, dy, dz);

        test_vel = test_vel.clamp(DVec3::splat(-500.0), DVec3::splat(500.0));

        println!("{}  -  {} {}", base_dist, test_pos, test_vel);
    }

    let result_1 = 0;

    // 163801440047383.16

    write!(out, "{}  {:?}", result_0, result_1).unwrap();
}

fn line4(pt: DVec4, dir: DVec4) -> Line4 {
    Line4 { pt, dir }
}

fn min_dist_sqr(line0: Line4, line1: Line4) -> f64 {
    let a = line0.pt;
    let b = line0.dir;
    let c = line1.pt;
    let d = line1.dir;
    let e = a - c;

    let bb = b.dot(b);
    let dd = d.dot(d);
    let bd = b.dot(d);
    let be = b.dot(e);
    let de = d.dot(e);

    let det = bd * bd - bb * dd;

    if det.abs() < 1e-9 {
        return line0.pt.distance_squared(line1.pt);
    }

    let t0 = (dd * be - de * bd) / det;
    let t1 = (be * bd - bb * de) / det;

    let p0 = line0.pt + t0 * line0.dir;
    let p1 = line1.pt + t1 * line1.dir;

    p0.distance_squared(p1)
}

fn find_intersection(a: Line2, b: Line2) -> Option<DVec2> {
    let det = a.dir.perp_dot(b.dir);
    if det.abs() < f64::EPSILON {
        return None;
    }

    let b_from_a = b.pt - a.pt;
    let t0 = (b_from_a).perp_dot(b.dir) / det;
    let t1 = (-b_from_a).perp_dot(a.dir) / (-det);

    if t0 > 0.0 && t1 > 0.0 {
        Some(a.pt + t0 * a.dir)
    } else {
        None
    }
}
