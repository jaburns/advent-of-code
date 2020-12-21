use std::collections::HashSet;
use std::convert::TryInto;
use crate::utils::expanse::Expanse2;

#[derive(Debug, Clone, Default)]
struct Tile {
    id: u64,
    uid: u64,
    grid: [[bool; 10]; 10],
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

impl Tile {
    pub fn read_edge(&self, direction: Direction) -> [bool; 10] {
        match direction {
            Direction::Up => (0..10).map(|x| self.grid[x][0]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            Direction::Left => (0..10).map(|y| self.grid[0][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            Direction::Down => (0..10).map(|x| self.grid[x][9]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            Direction::Right => (0..10).map(|y| self.grid[9][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
        }
    }

    pub fn mirror(mut self) -> Self {
        let mut new_grid = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                new_grid[x][y] = self.grid[9-x][y];
            }
        }

        self.grid = new_grid;
        self.uid += 4;
        self
    }

    pub fn rotate90(mut self) -> Self {
        let mut new_grid = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                new_grid[x][y] = self.grid[y][9-x];
            }
        }

        self.grid = new_grid;
        self.uid += 1;
        self
    }
}

fn produce_orientations(tile:&Tile) -> Vec<Tile> {
    let mut ret = Vec::new();

    ret.push(tile.clone());
    ret.push(tile.clone().rotate90());
    ret.push(tile.clone().rotate90().rotate90());
    ret.push(tile.clone().rotate90().rotate90().rotate90());
    ret.push(tile.clone().mirror());
    ret.push(tile.clone().mirror().rotate90());
    ret.push(tile.clone().mirror().rotate90().rotate90());
    ret.push(tile.clone().mirror().rotate90().rotate90().rotate90());

    ret
}

fn load_tiles() -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut cur_tile = Tile::default();

    for (i, line) in std::fs::read_to_string("data/day20.test")
        .unwrap()
        .lines()
        .enumerate()
    {
        let i = i % 12;

        if i == 0 {
            cur_tile.id = line.replace("Tile ", "").replace(":", "").parse().unwrap();
            cur_tile.uid = 8 * cur_tile.id;
        } else if i < 11 {
            for (j, chr) in line.chars().enumerate() {
                cur_tile.grid[i - 1][j] = chr == '#';
            }
        } else {
            tiles.extend(produce_orientations(&cur_tile));
            cur_tile = Tile::default();
        }
    }

    tiles.extend(produce_orientations(&cur_tile));
    tiles
}

fn find_neighbor(
    id_uid: &(u64, u64),
    direction: Direction,
    all_tiles: &[Tile],
    used_ids: &HashSet<u64>,
) -> Option<(u64, u64)> {
    let this = all_tiles.iter().find(|x| x.uid == id_uid.1).unwrap();
    let this_edge = this.read_edge(direction);
    let match_direction = direction.reverse();

    for tile in all_tiles {
        if used_ids.contains(&tile.id) || tile.id == id_uid.0 {
            continue;
        }

        if tile.read_edge(match_direction) == this_edge {
            return Some((tile.id, tile.uid));
        }
    }

    None
}

fn solve_tile_placement(tiles: &Vec<Tile>) -> Expanse2<(u64,u64)> {
    let mut ret = Expanse2::new();
    let mut used_ids = HashSet::new();
    let first_tile = &tiles[0];

    ret.write(
        0,
        0,
        (first_tile.id, first_tile.uid),
    );
    used_ids.insert(first_tile.id);

    let mut new_neighbors = vec![(0i32, 0i32)];
    while new_neighbors.len() > 0 {
        let (x, y) = new_neighbors.pop().unwrap();

        if ret.at(x, y - 1).is_none() {
            if let Some(ot) =
                find_neighbor(ret.read(x, y).unwrap(), Direction::Up, tiles.as_slice(), &used_ids)
            {
                ret.write(x, y - 1, ot);
                used_ids.insert(ot.0);
                new_neighbors.push((x, y - 1));
            }
        }

        if ret.at(x, y + 1).is_none() {
            if let Some(ot) =
                find_neighbor(ret.read(x, y).unwrap(), Direction::Down, tiles.as_slice(),  &used_ids)
            {
                ret.write(x, y + 1, ot);
                used_ids.insert(ot.0);
                new_neighbors.push((x, y + 1));
            }
        }

        if ret.at(x - 1, y).is_none() {
            if let Some(ot) =
                find_neighbor(ret.read(x, y).unwrap(), Direction::Left, tiles.as_slice(),  &used_ids)
            {
                ret.write(x - 1, y, ot);
                used_ids.insert(ot.0);
                new_neighbors.push((x - 1, y));
            }
        }

        if ret.at(x + 1, y).is_none() {
            if let Some(ot) =
                find_neighbor(ret.read(x, y).unwrap(), Direction::Right, tiles.as_slice(),  &used_ids)
            {
                ret.write(x + 1, y, ot);
                used_ids.insert(ot.0);
                new_neighbors.push((x + 1, y));
            }
        }
    }

    ret
}

fn product_of_corners(grid: &Expanse2<(u64, u64)>) -> u64 {
    let min_x = grid.x_range().min().unwrap();
    let min_y = grid.y_range().min().unwrap();
    let max_x = grid.x_range().max().unwrap();
    let max_y = grid.y_range().max().unwrap();

    grid.read(min_x, min_y).unwrap().0
        * grid.read(min_x, max_y).unwrap().0
        * grid.read(max_x, min_y).unwrap().0
        * grid.read(max_x, max_y).unwrap().0
}

pub fn main() {
    let tiles = load_tiles();

    let part1_grid = solve_tile_placement(&tiles);
    let render = part1_grid.render_to_string(" xxxx", |(id,_)| vec![" ", id.to_string().as_str()].join(""));
    let part1 = product_of_corners(&part1_grid);

    println!("{}\n{}", render, part1);
}
