use std::fmt::Write;

pub fn part1(raw_input: &str, out: &mut String) {
    let commands_iter = raw_input.lines().map(|x| x.split_once(' ').unwrap());

    let mut x = 0i32;
    let mut y = 0i32;

    for (cmd, valstr) in commands_iter.clone() {
        let val = valstr.parse::<i32>().unwrap();
        match cmd {
            "forward" => x += val,
            "up" => y -= val,
            "down" => y += val,
            _ => {}
        }
    }

    let result = x * y;
    write!(out, "{}", result).unwrap();
}

pub fn part2(raw_input: &str, out: &mut String) {
    let commands_iter = raw_input.lines().map(|x| x.split_once(' ').unwrap());

    let mut x = 0i32;
    let mut y = 0i32;
    let mut aim = 0i32;

    for (cmd, valstr) in commands_iter {
        let val = valstr.parse::<i32>().unwrap();
        match cmd {
            "forward" => {
                x += val;
                y += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => {}
        }
    }

    let result = x * y;
    write!(out, "{}", result).unwrap();
}
