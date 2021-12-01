use crate::utils::expanse::Expanse2;

#[derive(Debug, Copy, Clone)]
enum HexDir {
    E,
    Ne,
    Se,
    W,
    Nw,
    Sw,
}

static ALL_HEX_DIRS: [HexDir; 6] = [
    HexDir::E,
    HexDir::Ne,
    HexDir::Se,
    HexDir::W,
    HexDir::Nw,
    HexDir::Sw,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HexCoord(i32, i32);

fn parse_directions(line: &str) -> Vec<HexDir> {
    let mut ret = Vec::new();
    let mut modifier = Option::<char>::None;
    let chars = line.chars().collect::<Vec<_>>();

    for i in 0..chars.len() {
        match modifier {
            None => match chars[i] {
                'e' => ret.push(HexDir::E),
                'w' => ret.push(HexDir::W),
                x => modifier = Some(x),
            },
            Some('n') => {
                match chars[i] {
                    'e' => ret.push(HexDir::Ne),
                    'w' => ret.push(HexDir::Nw),
                    _ => panic!(),
                };
                modifier = None
            }
            Some('s') => {
                match chars[i] {
                    'e' => ret.push(HexDir::Se),
                    'w' => ret.push(HexDir::Sw),
                    _ => panic!(),
                };
                modifier = None
            }
            _ => panic!(),
        }
    }

    ret
}

fn step_coordinate(coord: HexCoord, dir: &HexDir) -> HexCoord {
    match dir {
        HexDir::E => HexCoord(coord.0 + 1, coord.1),
        HexDir::Ne => HexCoord(coord.0 + 1, coord.1 - 1),
        HexDir::Se => HexCoord(coord.0, coord.1 + 1),
        HexDir::W => HexCoord(coord.0 - 1, coord.1),
        HexDir::Nw => HexCoord(coord.0, coord.1 - 1),
        HexDir::Sw => HexCoord(coord.0 - 1, coord.1 + 1),
    }
}

fn apply_flips_at_coordinates(coords: &[HexCoord]) -> Expanse2<bool> {
    let mut floor = Expanse2::<bool>::new();

    for &HexCoord(q, r) in coords {
        let to_write = floor.read(q, r) != Some(&true);
        floor.write(q, r, to_write);
    }

    floor
}

fn apply_automaton_rules(floor: &Expanse2<bool>) -> Expanse2<bool> {
    let mut ret = floor.clone();

    for q in floor.x_range_plus(1) {
        for r in floor.y_range_plus(1) {
            let mut black_neighbors = 0u32;

            for d in &ALL_HEX_DIRS {
                let HexCoord(q1, r1) = step_coordinate(HexCoord(q,r), d);
                black_neighbors += (floor.read(q1, r1) == Some(&true)) as u32;
            }

            if floor.read(q, r) == Some(&true) {
                if black_neighbors == 0 || black_neighbors > 2 {
                    ret.write(q, r, false);
                }
            }
            else if black_neighbors == 2 {
                ret.write(q, r, true);
            }
        }
    }

    ret
}

pub fn main() {
    let coords = std::fs::read_to_string("data/day24.txt")
        .unwrap()
        .lines()
        .map(|x| {
            parse_directions(x)
                .iter()
                .fold(HexCoord(0, 0), step_coordinate)
        })
        .collect::<Vec<_>>();

    let mut floor = apply_flips_at_coordinates(&coords);

    let part1 = floor.find_many(|x| *x).len();

    for _ in 0..100 {
        floor = apply_automaton_rules(&floor);
    }

    let part2 = floor.find_many(|x| *x).len();

    println!("{} {}", part1, part2);
}
