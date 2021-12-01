fn find_loop_size(subject: u64, key: u64) -> u64 {
    let mut val = 1u64;
    let mut iter = 0u64;

    while val != key {
        iter += 1;
        val *= subject;
        val = val % 20201227;
    }

    iter
}

fn apply_loop_size(subject: u64, loop_size: u64) -> u64 {
    let mut val = 1u64;

    for _ in 0..loop_size {
        val *= subject;
        val = val % 20201227;
    }

    val
}

pub fn main() {
    let keys = std::fs::read_to_string("data/day25.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = apply_loop_size(keys[0], find_loop_size(7, keys[1]));

    println!("{}", part1);
}