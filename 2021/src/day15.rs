use hashbrown::HashSet;
use std::fmt::Write;

fn parse_cave(lines: &[&str]) -> Vec<Vec<u32>> {
    // cave[row][col]
    lines
        .iter()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Clone)]
struct Walk {
    coord: (usize, usize),
    visited: HashSet<(usize, usize)>,
    total_risk: u32,
}

fn step_walks(cave: &[Vec<u32>], walks: &mut Vec<Walk>) -> Option<u32> {
    let w = walks.remove(0);

    if w.coord.0 == cave[0].len() - 1 && w.coord.1 == cave.len() - 1 {
        return Some(w.total_risk);
    }

    if w.coord.0 < cave[0].len() - 1 {
        let (x1, y1) = (w.coord.0 + 1, w.coord.1);
        if !w.visited.contains(&(x1, y1)) {
            let mut visited = w.visited.clone();
            visited.insert((x1, y1));
            walks.push(Walk {
                coord: (x1, y1),
                visited,
                total_risk: w.total_risk + cave[y1][x1],
            });
        }
    }
    if w.coord.1 < cave.len() - 1 {
        let (x1, y1) = (w.coord.0, w.coord.1 + 1);
        if !w.visited.contains(&(x1, y1)) {
            let mut visited = w.visited.clone();
            visited.insert((x1, y1));
            walks.push(Walk {
                coord: (x1, y1),
                visited,
                total_risk: w.total_risk + cave[y1][x1],
            });
        }
    }
    if w.coord.0 > 0 {
        let (x1, y1) = (w.coord.0 - 1, w.coord.1);
        if !w.visited.contains(&(x1, y1)) {
            let mut visited = w.visited.clone();
            visited.insert((x1, y1));
            walks.push(Walk {
                coord: (x1, y1),
                visited,
                total_risk: w.total_risk + cave[y1][x1],
            });
        }
    }
    if w.coord.1 > 0 {
        let (x1, y1) = (w.coord.0, w.coord.1 - 1);
        if !w.visited.contains(&(x1, y1)) {
            let mut visited = w.visited.clone();
            visited.insert((x1, y1));
            walks.push(Walk {
                coord: (x1, y1),
                visited,
                total_risk: w.total_risk + cave[y1][x1],
            });
        }
    }

    walks.sort_unstable_by(|a, b| a.total_risk.cmp(&b.total_risk));

    while walks.len() > 5000 {
        walks.pop().unwrap();
    }

    None
}

pub fn part1(lines: &[&str], out: &mut String) {
    let cave = parse_cave(lines);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut walks = vec![Walk {
        coord: (0, 0),
        visited,
        total_risk: 0,
    }];

    let result = loop {
        if let Some(result) = step_walks(&cave, &mut walks) {
            break result;
        }
    };

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
