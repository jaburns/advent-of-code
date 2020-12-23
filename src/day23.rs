use std::collections::VecDeque;

fn step(cups: &VecDeque<u8>) -> VecDeque<u8> {
    let mut ret = cups.clone();

    let cur = ret.pop_front().unwrap();
    let hold0 = ret.pop_front().unwrap();
    let hold1 = ret.pop_front().unwrap();
    let hold2 = ret.pop_front().unwrap();

    let mut dest = if cur == 1 { 9 } else { cur - 1 };
    while dest == hold0 || dest == hold1 || dest == hold2 {
        dest = if dest == 1 { 9 } else { dest - 1 };
    }

    ret.push_back(cur);

    let insert_idx = ret.iter().position(|&x| x == dest).unwrap();
    ret.insert(insert_idx + 1, hold2);
    ret.insert(insert_idx + 1, hold1);
    ret.insert(insert_idx + 1, hold0);

    ret
}

pub fn main() {
    let cups = std::fs::read_to_string("data/day23.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u8>().unwrap())
        .collect::<VecDeque<_>>();

    let mut step_cups = step(&cups);
    for _ in 1..100 {
        step_cups = step(&step_cups);
    }

    while step_cups[0] != 1 {
        step_cups.rotate_left(1);
    }
    step_cups.pop_front();

    let part1 = step_cups
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{:?}", part1);
}
