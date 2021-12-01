#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct WireStep(i32, i32);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct VerticalWireSeg {
    pub x: i32,
    pub ymin: i32,
    pub ymax: i32,
    pub min_to_max: bool,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct HorizontalWireSeg {
    pub y: i32,
    pub xmin: i32,
    pub xmax: i32,
    pub min_to_max: bool,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum WireSeg {
    Vertical(VerticalWireSeg),
    Horizontal(HorizontalWireSeg),
}

fn parse_wire_step(s: &str) -> WireStep {
    let amount = s.get(1..).unwrap().parse::<i32>().unwrap();

    match s.get(0..1).unwrap() {
        "U" => WireStep(0, amount),
        "D" => WireStep(0, -amount),
        "L" => WireStep(-amount, 0),
        "R" => WireStep(amount, 0),
        _ => panic!("Unexpected step direction"),
    }
}

fn get_segs_from_steps(steps: &[WireStep]) -> Vec<WireSeg> {
    let mut result: Vec<WireSeg> = Vec::new();

    let mut x0 = 0i32;
    let mut y0 = 0i32;
    let mut x1 = 0i32;
    let mut y1 = 0i32;

    for WireStep(dx, dy) in steps.iter() {
        x1 += dx;
        y1 += dy;

        if x0 == x1 {
            result.push(WireSeg::Vertical(VerticalWireSeg {
                x: x1,
                ymin: std::cmp::min(y0, y1),
                ymax: std::cmp::max(y0, y1),
                min_to_max: y1 > y0,
            }));
        } else {
            result.push(WireSeg::Horizontal(HorizontalWireSeg {
                y: y1,
                xmin: std::cmp::min(x0, x1),
                xmax: std::cmp::max(x0, x1),
                min_to_max: x1 > x0,
            }));
        }

        x0 = x1;
        y0 = y1;
    }

    result
}

#[allow(unused_variables)]
fn update_shortest_distance_score(
    min_score: i32,
    h: &HorizontalWireSeg,
    v: &VerticalWireSeg,
    h_steps: i32,
    v_steps: i32,
) -> i32 {
    if v.x < h.xmin || v.x > h.xmax || h.y < v.ymin || h.y > v.ymax || h.y == 0 && v.x == 0 {
        min_score
    } else {
        std::cmp::min(min_score, i32::abs(v.x) + i32::abs(h.y))
    }
}

fn update_shortest_steps_score(
    min_score: i32,
    h: &HorizontalWireSeg,
    v: &VerticalWireSeg,
    h_steps: i32,
    v_steps: i32,
) -> i32 {
    if v.x < h.xmin || v.x > h.xmax || h.y < v.ymin || h.y > v.ymax || h.y == 0 && v.x == 0 {
        min_score
    } else {
        let v_new_steps = if v.min_to_max {
            h.y - v.ymin
        } else {
            v.ymax - h.y
        };
        let h_new_steps = if h.min_to_max {
            v.x - h.xmin
        } else {
            h.xmax - v.x
        };

        let all_steps = v_steps + v_new_steps + h_steps + h_new_steps;
        std::cmp::min(min_score, all_steps)
    }
}

fn find_best_intersection_score(
    sa: &[WireSeg],
    sb: &[WireSeg],
    score_update_fn: fn(i32, &HorizontalWireSeg, &VerticalWireSeg, i32, i32) -> i32,
) -> i32 {
    let mut min_score = std::i32::MAX;
    let mut a_steps = 0i32;

    for a in sa.iter() {
        let mut b_steps = 0i32;

        for b in sb.iter() {
            if let WireSeg::Vertical(v) = a {
                if let WireSeg::Horizontal(h) = b {
                    min_score = score_update_fn(min_score, h, v, b_steps, a_steps);
                }
            } else if let WireSeg::Vertical(v) = b {
                if let WireSeg::Horizontal(h) = a {
                    min_score = score_update_fn(min_score, h, v, a_steps, b_steps);
                }
            }

            match b {
                WireSeg::Horizontal(h) => b_steps += h.xmax - h.xmin,
                WireSeg::Vertical(v) => b_steps += v.ymax - v.ymin,
            }
        }

        match a {
            WireSeg::Horizontal(h) => a_steps += h.xmax - h.xmin,
            WireSeg::Vertical(v) => a_steps += v.ymax - v.ymin,
        }
    }

    min_score
}

pub fn main() {
    let wire_steps: Vec<Vec<WireStep>> = std::fs::read_to_string("data/day3.txt")
        .unwrap()
        .lines()
        .map(|x| String::from(x).split(",").map(parse_wire_step).collect())
        .collect();

    let segs0 = get_segs_from_steps(&wire_steps[0]);
    let segs1 = get_segs_from_steps(&wire_steps[1]);

    let result0 = find_best_intersection_score(&segs0, &segs1, update_shortest_distance_score);
    let result1 = find_best_intersection_score(&segs0, &segs1, update_shortest_steps_score);

    println!("{} {}", result0, result1);
}
