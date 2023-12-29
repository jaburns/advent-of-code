use arrayvec::ArrayVec;
use glam::{dvec2, dvec4, DVec2, DVec4};
use num_bigint::BigInt;
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

    let test0 = lines4.pop().unwrap();
    let test1 = lines4.pop().unwrap();
    let mut test0_t = 100000000000000_f64;
    let mut test1_t = 100000000000000_f64;
    let grad_step = 1.0;

    let measure_error = |test: Line4| -> f64 {
        let mut result = 0.0;
        for line in lines4.iter() {
            result += min_dist_sqr(*line, test).sqrt() / lines4.len() as f64;
        }
        result
    };

    let result_1_line = loop {
        let p0 = test0.pt + test0_t * test0.dir;
        let p1 = test1.pt + test1_t * test1.dir;
        let guess = line4(p0, p1 - p0);
        let err = measure_error(guess);

        if err < 4.0 {
            break guess;
        }

        let p0n = test0.pt + test0.dir * (test0_t - grad_step);
        let p0p = test0.pt + test0.dir * (test0_t + grad_step);
        let guess_0n = line4(p0n, p1 - p0n);
        let guess_0p = line4(p0p, p1 - p0p);
        let err_0n = measure_error(guess_0n);
        let err_0p = measure_error(guess_0p);

        if err_0n < err {
            let step = (err - err_0n) / grad_step;
            test0_t -= step * err;
        } else if err_0p < err {
            let step = (err - err_0p) / grad_step;
            test0_t += step * err;
        } else {
            println!("Converged 0 {} {} {}", err, err_0n, err_0p);
        }

        let p1n = test1.pt + test1.dir * (test1_t - grad_step);
        let p1p = test1.pt + test1.dir * (test1_t + grad_step);
        let guess_1n = line4(p0, p1n - p0);
        let guess_1p = line4(p0, p1p - p0);
        let err_1n = measure_error(guess_1n);
        let err_1p = measure_error(guess_1p);

        if err_1n < err {
            let step = (err - err_1n) / grad_step;
            test1_t -= step * err;
        } else if err_1p < err {
            let step = (err - err_1p) / grad_step;
            test1_t += step * err;
        } else {
            println!("Converged 1 {} {} {}", err, err_1n, err_1p);
        }

        println!("err {}: {} {}", err, test0_t, test1_t);
    };

    println!("done {:?}", result_1_line);

    // let dirn = result_1_line.dir.normalize();
    // let amount = result_1_line.pt.w / dirn.w;
    // let result_1 = (result_1_line.pt - amount * dirn) * 10000.0;
    // let result_1 = result_1.x as i64 + result_1.y as i64 + result_1.z as i64;
    let result_1 = 0;

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

    if det < 1e-9 {
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
