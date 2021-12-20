use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let target = Target::parse(lines[0]);

    let (best_y_vel, y_vels) = find_fanciest_y_vel_and_get_valid_options(&target);
    let x_vels = get_valid_x_vels(&target);

    let mut valid_count = 0_u32;
    for vx in &x_vels {
        for vy in &y_vels {
            valid_count += verify_initial_vel(&target, *vx, *vy) as u32;
        }
    }

    write!(out, "{}  {}", best_y_vel, valid_count).unwrap();
}

#[derive(Debug)]
struct Target {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl Target {
    fn parse(input: &str) -> Self {
        let input = input.replace("target area: x=", "");
        let mut parts_iter = input.split(", y=");

        let mut x_range_iter = parts_iter
            .next()
            .unwrap()
            .split("..")
            .map(|x| x.parse::<i32>().unwrap());

        let mut y_range_iter = parts_iter
            .next()
            .unwrap()
            .split("..")
            .map(|x| x.parse::<i32>().unwrap());

        Self {
            x_range: (x_range_iter.next().unwrap(), x_range_iter.next().unwrap()),
            y_range: (y_range_iter.next().unwrap(), y_range_iter.next().unwrap()),
        }
    }
}

fn find_fanciest_y_vel_and_get_valid_options(target: &Target) -> (i32, Vec<i32>) {
    let mut y_vel_test = target.y_range.0;
    let mut max_y_best = 0_i32;
    let mut valid = vec![];

    loop {
        let mut y_vel = y_vel_test;
        let mut y = 0_i32;
        let mut max_y = 0_i32;

        loop {
            y += y_vel;
            y_vel -= 1;

            if y > max_y {
                max_y = y;
            }

            if y >= 0 && y_vel < target.y_range.0 - 1 {
                return (max_y_best, valid);
            }

            if y >= target.y_range.0 && y <= target.y_range.1 {
                valid.push(y_vel_test);
                max_y_best = max_y_best.max(max_y);
                break;
            }

            if y < target.y_range.0 {
                break;
            }
        }

        y_vel_test += 1;
    }
}

fn get_valid_x_vels(target: &Target) -> Vec<i32> {
    let mut x_vel_test = 1_i32;
    let mut result = vec![];

    loop {
        let mut x_vel = x_vel_test;
        let mut x = 0_i32;

        loop {
            x += x_vel;
            if x_vel > 0 {
                x_vel -= 1;
            }

            if x >= target.x_range.0 && x <= target.x_range.1 {
                result.push(x_vel_test);
                break;
            }

            if x_vel == 0 {
                break;
            }
        }

        x_vel_test += 1;

        if x_vel_test > target.x_range.1 + 1 {
            return result;
        }
    }
}

fn verify_initial_vel(target: &Target, mut vx: i32, mut vy: i32) -> bool {
    let mut x = 0_i32;
    let mut y = 0_i32;

    loop {
        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;

        if x >= target.x_range.0
            && x <= target.x_range.1
            && y >= target.y_range.0
            && y <= target.y_range.1
        {
            return true;
        }

        if x > target.x_range.1 || y < target.y_range.0 {
            return false;
        }
    }
}
