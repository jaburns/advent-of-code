use std::fmt::Write;

type CostFn = fn(&[i32], i32) -> i32;

fn eval_simple_cost(crab_locs: &[i32], pos: i32) -> i32 {
    crab_locs.iter().map(|x| (x - pos).abs()).sum::<i32>()
}

fn eval_growing_cost(crab_locs: &[i32], pos: i32) -> i32 {
    crab_locs
        .iter()
        .map(|x| {
            let delta = (x - pos).abs();
            delta * (delta + 1) / 2
        })
        .sum::<i32>()
}

fn minimize_cost_fn(crab_locs: &[i32], cost_fn: CostFn) -> i32 {
    let max_coord = crab_locs.iter().fold(0i32, |a, x| a.max(*x));

    (0..max_coord)
        .map(|x| cost_fn(crab_locs, x))
        .fold(std::i32::MAX, |a, x| a.min(x))
}

fn run_part(input: &str, out: &mut String, cost_fn: CostFn) {
    let crab_locs: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let result = minimize_cost_fn(&crab_locs, cost_fn);
    write!(out, "{}", result).unwrap();
}

pub fn part1(lines: &[&str], out: &mut String) {
    run_part(lines[0], out, eval_simple_cost);
}

pub fn part2(lines: &[&str], out: &mut String) {
    run_part(lines[0], out, eval_growing_cost);
}
