use arrayvec::ArrayVec;
use glam::{dvec2, DVec2, DVec4};
use num_bigint::BigInt;
use std::fmt::Write;

const MAX_LINES: usize = 300;

type V4 = (BigInt, BigInt, BigInt, BigInt);

#[derive(Default, Debug, Copy, Clone)]
struct Line2 {
    pt: DVec2,
    dir: DVec2,
}

#[derive(Default, Debug, Clone)]
struct Line4 {
    pt: V4,
    dir: V4,
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
            pt: (
                BigInt::from(pt.0),
                BigInt::from(pt.1),
                BigInt::from(pt.2),
                BigInt::from(0),
            ),
            dir: (
                BigInt::from(dir.0),
                BigInt::from(dir.1),
                BigInt::from(dir.2),
                BigInt::from(0),
            ),
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
    let mut test0_t = BigInt::from(100000000000000_i64);
    let mut test1_t = BigInt::from(100000000000000_i64);
    //let _ = lines4.pop().unwrap();
    let target = lines4.pop().unwrap();
    let grad_step = BigInt::from(10);

    let result_1_line = loop {
        let p0 = dir4(&test0.pt, &test0.dir, &test0_t);
        let p1 = dir4(&test1.pt, &test1.dir, &test1_t);
        let guess = line4(&p0, &sub4(&p1, &p0));
        let err = min_dist_sqr(&target, &guess).sqrt();

        if err < BigInt::from(4) {
            println!("err {}: {} {}", err, test0_t, test1_t);
            break guess;
        }

        let p0n = dir4(
            &test0.pt,
            &test0.dir,
            &(test0_t.clone() - grad_step.clone()),
        );
        let p0p = dir4(
            &test0.pt,
            &test0.dir,
            &(test0_t.clone() + grad_step.clone()),
        );
        let guess_0n = line4(&p0n, &sub4(&p1, &p0n));
        let guess_0p = line4(&p0p, &sub4(&p1, &p0p));
        let err_0n = min_dist_sqr(&target, &guess_0n).sqrt();
        let err_0p = min_dist_sqr(&target, &guess_0p).sqrt();

        if err_0n < err {
            let step = (err.clone() - err_0n) / grad_step.clone();
            test0_t -= step * err.clone() / 1_000_000_000i64;
        } else if err_0p < err {
            let step = (err.clone() - err_0p) / grad_step.clone();
            test0_t += step * err.clone() / 1_000_000_000i64;
        } else {
            println!("Converged A {} {} {}", err, err_0n, err_0p);
        }

        let p1n = dir4(
            &test1.pt,
            &test1.dir,
            &(test1_t.clone() - grad_step.clone()),
        );
        let p1p = dir4(
            &test1.pt,
            &test1.dir,
            &(test1_t.clone() + grad_step.clone()),
        );
        let guess_1n = line4(&p0, &sub4(&p1n, &p0));
        let guess_1p = line4(&p0, &sub4(&p1p, &p0));
        let err_1n = min_dist_sqr(&target, &guess_1n).sqrt();
        let err_1p = min_dist_sqr(&target, &guess_1p).sqrt();

        if err_1n < err {
            let step = (err.clone() - err_1n) / grad_step.clone();
            test1_t -= step * err.clone() / 1_000_000_000i64;
        } else if err_1p < err {
            let step = (err.clone() - err_1p) / grad_step.clone();
            test1_t += step * err.clone() / 1_000_000_000i64;
        } else {
            println!("Converged B");
        }

        //test0_t = test0_t.max(BigInt::from(0));
        //test1_t = test1_t.max(BigInt::from(0));

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

fn line4(pt: &V4, dir: &V4) -> Line4 {
    Line4 {
        pt: pt.clone(),
        dir: dir.clone(),
    }
}

fn dot4(a: &V4, b: &V4) -> BigInt {
    a.0.clone() * b.0.clone()
        + a.1.clone() * b.1.clone()
        + a.2.clone() * b.2.clone()
        + a.3.clone() * b.3.clone()
}
fn add4(a: &V4, b: &V4) -> V4 {
    (
        a.0.clone() + b.0.clone(),
        a.1.clone() + b.1.clone(),
        a.2.clone() + b.2.clone(),
        a.3.clone() + b.3.clone(),
    )
}
fn sub4(a: &V4, b: &V4) -> V4 {
    (
        a.0.clone() - b.0.clone(),
        a.1.clone() - b.1.clone(),
        a.2.clone() - b.2.clone(),
        a.3.clone() - b.3.clone(),
    )
}
fn distsq4(a: &V4, b: &V4) -> BigInt {
    let delta = sub4(a, b);
    dot4(&delta, &delta)
}
fn scale4(scale: &BigInt, b: &V4) -> V4 {
    (
        scale * b.0.clone(),
        scale * b.1.clone(),
        scale * b.2.clone(),
        scale * b.3.clone(),
    )
}
fn dir4(pt: &V4, dir: &V4, t: &BigInt) -> V4 {
    add4(pt, &scale4(t, dir))
}

fn min_dist_sqr(line0: &Line4, line1: &Line4) -> BigInt {
    let a = &line0.pt;
    let b = &line0.dir;
    let c = &line1.pt;
    let d = &line1.dir;
    let e = sub4(a, c);

    let bb = dot4(b, b);
    let dd = dot4(d, d);
    let bd = dot4(b, d);
    let be = dot4(b, &e);
    let de = dot4(d, &e);

    let det = bd.clone() * bd.clone() - bb.clone() * dd.clone();

    if det == BigInt::from(0) {
        return distsq4(&line0.pt, &line1.pt);
    }

    let t0 = (dd.clone() * be.clone() - de.clone() * bd.clone()) / det.clone();
    let t1 = (be.clone() * bd.clone() - bb.clone() * de.clone()) / det.clone();

    let p0 = dir4(&line0.pt, &line0.dir, &t0);
    let p1 = dir4(&line1.pt, &line1.dir, &t1);

    distsq4(&p0, &p1)
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
