pub fn main() {
    let raw_input = std::fs::read_to_string("data/day2.txt").unwrap();
    let commands_iter = raw_input.lines().map(|x| x.split_once(' ').unwrap());

    let part1 = {
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

        x * y
    };

    let part2 = {
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

        x * y
    };

    println!("{} {}", part1, part2);
}
