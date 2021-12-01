use crate::expanse::Expanse;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PortalKind {
    Push,
    Pop,
}

impl PortalKind {
    pub fn other(&self) -> PortalKind {
        match self {
            PortalKind::Push => PortalKind::Pop,
            PortalKind::Pop => PortalKind::Push,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TileKind {
    Path,
    Portal(PortalKind, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tile {
    pub dist: u32,
    pub kind: TileKind,
}

fn is_capital_letter(ch: char) -> bool {
    ch >= 'A' && ch <= 'Z'
}

fn load_maze(chars: &Vec<Vec<char>>) -> Expanse<Tile> {
    let mut result = Expanse::new();

    let kind_at = |x: usize, y: usize| -> PortalKind {
        if x == 2 || y == 2 || x == chars[0].len() - 3 || y == chars.len() - 3 {
            PortalKind::Pop
        } else {
            PortalKind::Push
        }
    };

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] != '.' {
                continue;
            }

            let kind = if is_capital_letter(chars[y][x - 1]) {
                TileKind::Portal(
                    kind_at(x, y),
                    format!("{}{}", chars[y][x - 2], chars[y][x - 1]),
                )
            } else if is_capital_letter(chars[y][x + 1]) {
                TileKind::Portal(
                    kind_at(x, y),
                    format!("{}{}", chars[y][x + 1], chars[y][x + 2]),
                )
            } else if is_capital_letter(chars[y - 1][x]) {
                TileKind::Portal(
                    kind_at(x, y),
                    format!("{}{}", chars[y - 2][x], chars[y - 1][x]),
                )
            } else if is_capital_letter(chars[y + 1][x]) {
                TileKind::Portal(
                    kind_at(x, y),
                    format!("{}{}", chars[y + 1][x], chars[y + 2][x]),
                )
            } else {
                TileKind::Path
            };

            result.write(
                x as i32,
                y as i32,
                Tile {
                    dist: 0,
                    kind: kind,
                },
            );
        }
    }

    result
}

fn find_portals(
    maze: &Expanse<Tile>,
    id: &str,
    portal_kind: Option<PortalKind>,
) -> Vec<(i32, i32)> {
    maze.find_many(|Tile { kind, .. }| {
        if let TileKind::Portal(pk, x) = kind {
            x == id && (portal_kind.is_none() || portal_kind.unwrap() == *pk)
        } else {
            false
        }
    })
}

fn can_walk(maze: &Expanse<Tile>, x: i32, y: i32) -> bool {
    match maze.read(x, y) {
        Some(Tile { dist, .. }) => *dist == 0,
        _ => false,
    }
}

fn can_flat_portal(
    _maze_proto: &Expanse<Tile>,
    levels: &mut Vec<Expanse<Tile>>,
    x: i32,
    y: i32,
    _z: usize,
) -> Option<(i32, i32, usize)> {
    if let Some(Tile {
        kind: TileKind::Portal(_, id),
        ..
    }) = levels[0].read(x, y)
    {
        for &(px, py) in find_portals(&levels[0], id, None).iter() {
            if px != x && py != y && levels[0].read(px, py).unwrap().dist == 0 {
                return Some((px, py, 0));
            }
        }
        None
    } else {
        None
    }
}

fn can_recursive_portal(
    maze_proto: &Expanse<Tile>,
    levels: &mut Vec<Expanse<Tile>>,
    x: i32,
    y: i32,
    z: usize,
) -> Option<(i32, i32, usize)> {
    let portal_kind: PortalKind;
    let portal_id: String;

    if let Some(Tile {
        kind: TileKind::Portal(pk, id),
        ..
    }) = levels[z].read(x, y)
    {
        portal_kind = *pk;
        portal_id = id.clone();
    } else {
        return None;
    }

    if z == 0 && portal_kind == PortalKind::Pop {
        return None;
    }

    let z1 = match portal_kind {
        PortalKind::Pop => z - 1,
        PortalKind::Push => z + 1,
    };

    if z1 >= levels.len() {
        levels.push(maze_proto.clone());
    }

    for &(px, py) in find_portals(&levels[z1], &portal_id, Some(portal_kind.other())).iter() {
        if px != x && py != y && levels[z1].read(px, py).unwrap().dist == 0 {
            return Some((px, py, z1));
        }
    }

    None
}

fn solve_maze(maze: &Expanse<Tile>, recursive: bool) -> u32 {
    let mut levels = vec![maze.clone()];

    let (x, y) = find_portals(&levels[0], "AA", None)[0];
    let (zx, zy) = find_portals(&levels[0], "ZZ", None)[0];

    let mut frontier = vec![(x, y, 0)];
    let mut cur_dist = 1;

    let portal_fn = if recursive {
        can_recursive_portal
    } else {
        can_flat_portal
    };

    while frontier.len() > 0 {
        for i in 0..frontier.len() {
            let (x, y, z) = frontier[i];
            levels[z].at(x, y).unwrap().dist = cur_dist;

            if x == zx && y == zy && z == 0 {
                return cur_dist - 1;
            }
        }

        let mut new_frontier = Vec::<(i32, i32, usize)>::new();

        for i in 0..frontier.len() {
            let (x, y, z) = frontier[i];
            if can_walk(&levels[z], x + 1, y) {
                new_frontier.push((x + 1, y, z));
            }
            if can_walk(&levels[z], x - 1, y) {
                new_frontier.push((x - 1, y, z));
            }
            if can_walk(&levels[z], x, y + 1) {
                new_frontier.push((x, y + 1, z));
            }
            if can_walk(&levels[z], x, y - 1) {
                new_frontier.push((x, y - 1, z));
            }
            if let Some(new_pos) = portal_fn(maze, &mut levels, x, y, z) {
                new_frontier.push(new_pos);
            }
        }

        cur_dist += 1;
        frontier = new_frontier;
    }

    panic!()
}

pub fn main() {
    let map_chars: Vec<Vec<char>> = std::fs::read_to_string("data/day20.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let maze = load_maze(&map_chars);

    let result0 = solve_maze(&maze, false);
    let result1 = solve_maze(&maze, true);

    println!("{} {}", result0, result1);
}
