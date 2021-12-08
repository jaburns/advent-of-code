use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let result: usize = lines
        .iter()
        .map(|line| {
            let mut iter = line.split('|');
            iter.next();
            iter.next()
                .unwrap()
                .split(' ')
                .map(|x| x.trim().len())
                .filter(|&len| len == 2 || len == 4 || len == 3 || len == 7)
                .count()
        })
        .sum();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
