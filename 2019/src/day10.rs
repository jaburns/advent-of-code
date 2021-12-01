use num::integer::*;

fn asteroid_is_visible_from(
    map: &Vec<Vec<bool>>,
    rock_x: i32,
    rock_y: i32,
    from_x: i32,
    from_y: i32,
) -> bool {
    if !map[rock_y as usize][rock_x as usize] {
        return false;
    }
    if rock_x == from_x && rock_y == from_y {
        return false;
    }

    let mut dx = from_x - rock_x;
    let mut dy = from_y - rock_y;
    let gcd = dx.gcd(&dy);
    dx /= gcd;
    dy /= gcd;

    let mut sx = rock_x;
    let mut sy = rock_y;
    loop {
        sx += dx;
        sy += dy;
        if sx == from_x && sy == from_y {
            return true;
        }
        if map[sy as usize][sx as usize] {
            return false;
        }
    }
}

fn find_visible_asteroids(map: &Vec<Vec<bool>>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut results = Vec::<(i32, i32)>::new();

    for iy in 0..(map.len() as i32) {
        for ix in 0..(map[0].len() as i32) {
            if asteroid_is_visible_from(map, ix, iy, x, y) {
                results.push((ix, iy));
            }
        }
    }

    results
}

fn find_best_asteroid(map: &Vec<Vec<bool>>) -> (i32, i32, u32) {
    let mut result = (0, 0, 0);

    for iy in 0..(map.len() as i32) {
        for ix in 0..(map[0].len() as i32) {
            let count = find_visible_asteroids(map, ix, iy).len() as u32;
            if count >= result.2 {
                result = (ix, iy, count);
            }
        }
    }

    result
}

fn blast_visible_asteroids(map: &mut Vec<Vec<bool>>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut targets_with_angles: Vec<((i32, i32), f32)> = find_visible_asteroids(map, x, y)
        .iter()
        .map(|&a| (a, ((a.0 - x) as f32).atan2((a.1 - y) as f32)))
        .collect();

    targets_with_angles.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for ((tx, ty), _) in &targets_with_angles {
        map[*ty as usize][*tx as usize] = false;
    }

    targets_with_angles.iter().map(|&(a, _)| a).collect()
}

fn find_nth_blasted_asteroid(map: &mut Vec<Vec<bool>>, x: i32, y: i32, n: usize) -> (i32, i32) {
    let mut blasted_asteroids = 0usize;
    loop {
        let old_count = blasted_asteroids;
        let blasted = blast_visible_asteroids(map, x, y);
        blasted_asteroids += blasted.len();

        if blasted_asteroids >= n {
            return blasted[n - old_count - 1];
        }
    }
}

pub fn main() {
    let mut map: Vec<Vec<bool>> = std::fs::read_to_string("data/day10.txt")
        .unwrap()
        .lines()
        .map(|x| x.trim().chars().map(|x| x == '#').collect())
        .collect();

    let (x, y, result0) = find_best_asteroid(&map);
    let (bx, by) = find_nth_blasted_asteroid(&mut map, x, y, 200);
    let result1 = bx * 100 + by;

    println!("{} {}", result0, result1);
}
