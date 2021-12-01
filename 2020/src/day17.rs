use crate::utils::expanse::{Expanse3, Expanse4};

fn step_conway_cube_3(world: &Expanse3<bool>) -> Expanse3<bool> {
    let mut result = world.clone();

    for x in result.x_range_plus(1) {
        for y in result.y_range_plus(1) {
            for z in result.z_range_plus(1) {
                let mut neighbors = 0i32;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 {
                                continue;
                            }

                            if let Some(true) = world.read(x + dx, y + dy, z + dz) {
                                neighbors += 1;
                            }
                        }
                    }
                }

                if let Some(true) = world.read(x, y, z) {
                    if neighbors < 2 || neighbors > 3 {
                        result.write(x, y, z, false);
                    }
                } else if neighbors == 3 {
                    result.write(x, y, z, true);
                }
            }
        }
    }

    result
}

fn step_conway_cube_4(world: &Expanse4<bool>) -> Expanse4<bool> {
    let mut result = world.clone();

    for x in result.x_range_plus(1) {
        for y in result.y_range_plus(1) {
            for z in result.z_range_plus(1) {
                for w in result.w_range_plus(1) {
                    let mut neighbors = 0i32;

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            for dz in -1..=1 {
                                for dw in -1..=1 {
                                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                        continue;
                                    }

                                    if let Some(true) = world.read(x + dx, y + dy, z + dz, w + dw) {
                                        neighbors += 1;
                                    }
                                }
                            }
                        }
                    }

                    if let Some(true) = world.read(x, y, z, w) {
                        if neighbors < 2 || neighbors > 3 {
                            result.write(x, y, z, w, false);
                        }
                    } else if neighbors == 3 {
                        result.write(x, y, z, w, true);
                    }
                }
            }
        }
    }

    result
}

pub fn main() {
    let mut world3 = Expanse3::<bool>::new();
    let mut world4 = Expanse4::<bool>::new();

    for (y, line) in std::fs::read_to_string("data/day17.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars())
        .enumerate()
    {
        for (x, ch) in line.enumerate() {
            world3.write(x as i32, y as i32, 0, ch == '#');
            world4.write(x as i32, y as i32, 0, 0, ch == '#');
        }
    }

    for _ in 0..6 {
        world3 = step_conway_cube_3(&world3);
        world4 = step_conway_cube_4(&world4);
    }

    let part1 = world3.find_many(|x| *x).len();
    let part2 = world4.find_many(|x| *x).len();

    println!("{} {}", part1, part2);
}
