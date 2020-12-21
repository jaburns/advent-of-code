use std::collections::HashMap;
use crate::utils::expanse::Expanse2;

#[derive(Debug, Clone, Default)]
struct Tile {
    id: u64,
    grid: [[bool; 10]; 10],
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Debug, Clone, Copy)]
// enum Rotation {
//     None,
//     Rot90,
//     Rot180,
//     Rot270,
// }
// 
// impl Default for Rotation {
//     fn default() -> Self {
//         Self::None
//     }
// }
// 
// #[derive(Debug, Clone, Copy, Default)]
// struct Orientation {
//     rotation: Rotation,
//     mirrored: bool,
// }
// 
// #[derive(Debug, Clone)]
// struct OrientedTile {
//     tile_id: u64,
//     orientation: Orientation,
// }
// 
// impl Tile {
//     pub fn read_edge(&self, orientation: Orientation, direction: Direction) -> [bool; 10] {
//         let top = (0..10).map(|x| self.grid[x][0]).collect::<Vec<_>>().as_slice().try_into().unwrap();
//         let left = (0..10).rev().map(|y| self.grid[0][y]).collect::<Vec<_>>().as_slice().try_into().unwrap();
//         let bottom = (0..10).rev().map(|x| self.grid[x][9]).collect::<Vec<_>>().as_slice().try_into().unwrap();
//         let top = (0..10).map(|y| self.grid[9][y]).collect::<Vec<_>>().as_slice().try_into().unwrap();
// 
//         match direction {
//             Direction::Up => {
//                 match orientation.rotation {
//                     Rotation::None => (0..10).map(|x| self.grid[x][0]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot90 => (0..10).rev().map(|y| self.grid[0][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot180 => (0..10).rev().map(|x| self.grid[x][9]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot270 => (0..10).map(|y| self.grid[9][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                 }
//             },
//             Direction::Down => {
//                 match orientation.rotation {
//                     Rotation::None => (0..10).map(|x| self.grid[x][0]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot90 => (0..10).rev().map(|y| self.grid[0][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot180 => (0..10).rev().map(|x| self.grid[x][9]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                     Rotation::Rot270 => (0..10).map(|y| self.grid[9][y]).collect::<Vec<_>>().as_slice().try_into().unwrap(),
//                 }
//             },
//         }
//     }
// }

impl Tile {
    pub fn read_edge(&self, direction: Direction) -> [bool; 10] {
    }
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
        } else if i < 11 {
            for (j, chr) in line.chars().enumerate() {
                cur_tile.grid[i - 1][j] = chr == '#';
            }
        } else {
            tiles.push(cur_tile);
            cur_tile = Tile::default();
        }
    }

    tiles
}

fn find_neighbor(
    tile_id: u64,
    direction: Direction,
    all_tiles_by_id: &HashMap<u64, Tile>,
    available_tiles: &[Tile],
) -> Option<u64> {

    let my_edge = all_tiles_by_id[&tile_id].read_edge(direction);

//    for tile in available_tiles {
//        tile.read_edge(orientation, direction)
//    }
    None
}

fn load_tiles_by_id(tiles: &Vec<Tile>) -> HashMap<u64, Tile> {
    let mut ret = HashMap::new();

    for tile in tiles {
        ret.insert(tile.id, tile.clone());
    }

    ret
}

fn solve_tile_placement(tiles: &Vec<Tile>) -> Expanse2<OrientedTile> {
    let mut ret = Expanse2::new();
    let mut tile_bag = tiles.clone();
    let all_tiles = load_tiles_by_id(tiles);
    let first_tile = tile_bag.pop().unwrap();

    ret.write(
        0,
        0,
        OrientedTile {
            tile_id: first_tile.id,
            orientation: Orientation::default(),
        },
    );

    let mut new_neighbors = vec![(0i32, 0i32)];
    while new_neighbors.len() > 0 {
        let (x, y) = new_neighbors.pop().unwrap();

        if ret.at(x, y - 1).is_none() {
            if let Some(ot) =
                find_neighbor(ret.at(x, y).unwrap(), Direction::Up, &all_tiles, tile_bag.as_slice())
            {
                ret.write(x, y - 1, ot);
                new_neighbors.push((x, y - 1));
            }
        }

        if ret.at(x, y + 1).is_none() {
            if let Some(ot) =
                find_neighbor(ret.at(x, y).unwrap(), Direction::Down, &all_tiles, tile_bag.as_slice())
            {
                ret.write(x, y + 1, ot);
                new_neighbors.push((x, y + 1));
            }
        }

        if ret.at(x - 1, y).is_none() {
            if let Some(ot) =
                find_neighbor(ret.at(x, y).unwrap(), Direction::Left, &all_tiles, tile_bag.as_slice())
            {
                ret.write(x - 1, y, ot);
                new_neighbors.push((x - 1, y));
            }
        }

        if ret.at(x + 1, y).is_none() {
            if let Some(ot) =
                find_neighbor(ret.at(x, y).unwrap(), Direction::Right, &all_tiles, tile_bag.as_slice())
            {
                ret.write(x + 1, y, ot);
                new_neighbors.push((x + 1, y));
            }
        }
    }

    ret
}

fn product_of_corners(grid: &Expanse2<OrientedTile>) -> u64 {
    let min_x = grid.x_range().min().unwrap();
    let min_y = grid.y_range().min().unwrap();
    let max_x = grid.x_range().max().unwrap();
    let max_y = grid.y_range().max().unwrap();

    grid.read(min_x, min_y).unwrap().tile_id
        * grid.read(min_x, max_y).unwrap().tile_id
        * grid.read(max_x, min_y).unwrap().tile_id
        * grid.read(max_x, max_y).unwrap().tile_id
}

pub fn main() {
    let tiles = load_tiles();

    let part1_grid = solve_tile_placement(&tiles);

    println!("{:?}", part1_grid);
}
