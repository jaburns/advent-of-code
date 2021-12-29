use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let mut pos = [
        lines[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        lines[1]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    ];
    let mut score = [0_u32, 0_u32];
    let mut die = 1_u32;
    let mut player = 0_usize;
    let mut rolls = 0_u32;

    loop {
        let mut mov = die;
        die = (die % 100) + 1;
        rolls += 1;
        mov += die;
        die = (die % 100) + 1;
        rolls += 1;
        mov += die;
        die = (die % 100) + 1;
        rolls += 1;

        pos[player] = ((pos[player] + mov - 1) % 10) + 1;
        score[player] += pos[player];

        if score[player] >= 1000 {
            break;
        }

        player = (player + 1) & 1;
    }

    let result = score[0].min(score[1]) * rolls;

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
