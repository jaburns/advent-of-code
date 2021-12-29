use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    East,
    South,
    Empty,
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

fn step_world(world0: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let rows = world0.len();
    let cols = world0[0].len();

    let mut world1 = Vec::from(world0);

    // Move east
    for (row, cells) in world0.iter().enumerate() {
        for (col, cell) in cells.iter().enumerate() {
            if *cell != Cell::East {
                continue;
            }

            if world0[row][(col + 1) % cols] != Cell::Empty {
                continue;
            }

            let write_source = world1.get_mut(row).unwrap().get_mut(col).unwrap();
            *write_source = Cell::Empty;

            let write_target = world1
                .get_mut(row)
                .unwrap()
                .get_mut((col + 1) % cols)
                .unwrap();
            *write_target = Cell::East;
        }
    }

    let mut world2 = world1.clone();

    // Move south
    for (row, cells) in world1.iter().enumerate() {
        for (col, cell) in cells.iter().enumerate() {
            if *cell != Cell::South {
                continue;
            }

            if world1[(row + 1) % rows][col] != Cell::Empty {
                continue;
            }

            let write_source = world2.get_mut(row).unwrap().get_mut(col).unwrap();
            *write_source = Cell::Empty;

            let write_target = world2
                .get_mut((row + 1) % rows)
                .unwrap()
                .get_mut(col)
                .unwrap();
            *write_target = Cell::South;
        }
    }

    world2
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut world = lines
        .iter()
        .map(|x| x.chars().map(Cell::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    loop {
        count += 1;
        let new_world = step_world(&world);
        if new_world == world {
            break;
        }
        world = new_world;
    }

    let result = count;

    write!(out, "{:?}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
