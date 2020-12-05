const GRID_WIDTH: usize = 8;
const GRID_DEPTH: usize = 128;

pub fn main() {
    let lines: Vec<_> = std::fs::read_to_string("data/day5.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let coordinates = lines
        .iter()
        .map(|x| x.split_at(7))
        .map(|(r, c)| {
            (
                parse_binary_number('F', 'B', r),
                parse_binary_number('L', 'R', c),
            )
        })
        .collect::<Vec<_>>();

    let part1 = get_highest_id(&coordinates);
    let part2 = find_missing_id(&coordinates);

    println!("{} {}", part1, part2);
}

fn parse_binary_number(zero_char: char, one_char: char, string: &str) -> usize {
    usize::from_str_radix(
        string
            .replace(one_char, "1")
            .replace(zero_char, "0")
            .as_str(),
        2,
    )
    .unwrap()
}

fn get_id_for_coordinate((r, c): &(usize, usize)) -> i32 {
    (r * 8 + c) as i32
}

fn get_highest_id(coords: &Vec<(usize, usize)>) -> i32 {
    let mut ids = coords.iter().map(get_id_for_coordinate).collect::<Vec<_>>();
    ids.sort();
    *ids.last().unwrap()
}

fn find_missing_id(coords: &Vec<(usize, usize)>) -> i32 {
    let mut grid = [[false; GRID_WIDTH]; GRID_DEPTH];

    for &(r, c) in coords {
        grid[r][c] = true;
    }

    let mut candidates: Vec<i32> = Vec::new();
    for r in 1..GRID_DEPTH {
        for c in 0..GRID_WIDTH {
            if !grid[r][c] {
                candidates.push(get_id_for_coordinate(&(r, c)));
            }
        }
    }
    candidates.sort();

    for i in 1..(candidates.len() - 1) {
        if candidates[i - 1] != candidates[i] - 1 && candidates[i + 1] != candidates[i] + 1 {
            return candidates[i];
        }
    }

    panic!()
}
