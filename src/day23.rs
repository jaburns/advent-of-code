fn run_game(cups: &Vec<usize>, size: usize, iterations: usize) -> Vec<usize> {
    let mut next_cup = vec![0usize; size + 1];

    let mut prev_cup = if size > cups.len() {
        size
    } else {
        cups[cups.len() - 1]
    };

    for i in 0..size {
        if i < cups.len() {
            next_cup[prev_cup] = cups[i];
            prev_cup = cups[i];
        } else {
            next_cup[prev_cup] = i + 1;
            prev_cup = i + 1;
        }
    }

    let mut cur_cup = cups[0];
    for _ in 0..iterations {
        let grab0 = next_cup[cur_cup];
        let grab1 = next_cup[grab0];
        let grab2 = next_cup[grab1];

        let mut dest = if cur_cup == 1 { size } else { cur_cup - 1 };
        while dest == grab0 || dest == grab1 || dest == grab2 {
            dest = if dest == 1 { size } else { dest - 1 };
        }

        next_cup[cur_cup] = next_cup[grab2];
        let old_dest_next = next_cup[dest];
        next_cup[dest] = grab0;
        next_cup[grab2] = old_dest_next;
        cur_cup = next_cup[cur_cup];
    }

    let mut ret = Vec::new();
    cur_cup = next_cup[1];
    for _ in 0..8 {
        ret.push( cur_cup);
        cur_cup = next_cup[cur_cup];
    }
    ret
}

pub fn main() {
    let cups = std::fs::read_to_string("data/day23.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = run_game(&cups, 9, 100)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let part2 = run_game(&cups, 1_000_000, 10_000_000)
        .iter()
        .take(2)
        .product::<usize>();

    println!("{} {}", part1, part2);
}
