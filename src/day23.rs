use std::collections::{VecDeque};

fn step(cups: &mut VecDeque<u32>, max_val: u32) {
    let cur = cups.pop_front().unwrap();
    let hold0 = cups.pop_front().unwrap();
    let hold1 = cups.pop_front().unwrap();
    let hold2 = cups.pop_front().unwrap();

    let mut dest = if cur == 1 { max_val } else { cur - 1 };
    while dest == hold0 || dest == hold1 || dest == hold2 {
        dest = if dest == 1 { max_val } else { dest - 1 };
    }

    cups.push_back(cur);

    let insert_idx = cups.iter().position(|&x| x == dest).unwrap();
    cups.insert(insert_idx + 1, hold2);
    cups.insert(insert_idx + 1, hold1);
    cups.insert(insert_idx + 1, hold0);
}

pub fn main() {
    let cups = std::fs::read_to_string("data/day23.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect::<VecDeque<_>>();

    let part1 = {
        let mut cups = cups.clone();
        for _ in 0..100 {
            step(&mut cups, 9);
        }

        while cups[0] != 1 {
            cups.rotate_left(1);
        }
        cups.pop_front();

        cups
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    };

    let part2: u64 = {
        let mut cups = cups.clone();

        for i in 10..=1_000_000 {
            cups.push_back(i as u32);
        }
        for i in 0..10_000_000 {
            if i % 1_000 == 0 {
                println!("Percent done: {}", 100f32 * i as f32 / 10_000_000f32);
            }
            step(&mut cups, 1_000_000);
        }

        while cups[0] != 1 {
            cups.rotate_left(1);
        }

        cups
            .iter()
            .skip(1)
            .take(2)
            .map(|&x| x as u64)
            .product()
    };

    println!("{} {}", part1, part2);
}
