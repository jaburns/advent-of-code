use crate::expanse::{Expanse, TwoVec};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct World {
    width: i32,
    height: i32,
    grid: Expanse<()>,
}

impl World {
    fn new(input: &Vec<Vec<char>>) -> Self {
        let mut grid = Expanse::<()>::new();

        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == '#' {
                    grid.write(x as i32, y as i32, ());
                }
            }
        }

        Self {
            width: input[0].len() as i32,
            height: input.len() as i32,
            grid: grid,
        }
    }

    fn neighbors_at(&self, x: i32, y: i32) -> i32 {
        let a = if self.grid.read(x + 1, y).is_some() {
            1
        } else {
            0
        };
        let b = if self.grid.read(x - 1, y).is_some() {
            1
        } else {
            0
        };
        let c = if self.grid.read(x, y + 1).is_some() {
            1
        } else {
            0
        };
        let d = if self.grid.read(x, y - 1).is_some() {
            1
        } else {
            0
        };

        a + b + c + d
    }

    fn step(&self) -> World {
        let mut result = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbs = self.neighbors_at(x, y);

                if self.grid.read(x, y).is_some() {
                    if neighbs != 1 {
                        result.grid.erase(x, y);
                    }
                } else if neighbs == 1 || neighbs == 2 {
                    result.grid.write(x, y, ());
                }
            }
        }

        result
    }

    fn get_biodiversity(&self) -> u64 {
        let mut result = 0u64;
        let mut digit = 1u64;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.read(x, y).is_some() {
                    result += digit;
                }
                digit <<= 1;
            }
        }

        result
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let out = self
            .grid
            .render_to_string(false, ".", |_| String::from("#"));
        println!("{}\n", out);
    }

    pub fn solve(input: &Vec<Vec<char>>) -> u64 {
        let mut world = World::new(&input);
        let mut set = HashSet::<u64>::new();
        let mut diversity;

        loop {
            diversity = world.get_biodiversity();
            if set.contains(&diversity) {
                break;
            }
            set.insert(diversity);
            world = world.step();
        }

        diversity
    }
}

#[derive(Debug, Clone)]
struct RecursiveWorld {
    grids: TwoVec<Expanse<()>>,
}

impl RecursiveWorld {
    fn new(input: &Vec<Vec<char>>) -> Self {
        let mut grid = Expanse::<()>::new();

        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == '#' {
                    grid.write(x as i32, y as i32, ());
                }
            }
        }

        let mut vec = TwoVec::new();
        vec.expand_to_contain(-1, Expanse::<()>::new);
        vec.expand_to_contain(1, Expanse::<()>::new);
        vec[0] = grid;

        Self { grids: vec }
    }

    fn ensure_enough_layers(&mut self) {
        let start = self.grids.index_range().start;
        let end = self.grids.index_range().end - 1;

        if !self.grids[start].empty() {
            self.grids.expand_to_contain(start - 1, Expanse::<()>::new);
        }
        if !self.grids[end].empty() {
            self.grids.expand_to_contain(end + 1, Expanse::<()>::new);
        }
    }

    fn neighbors_at(&self, i: i32, x: i32, y: i32) -> i32 {
        let checks = if x == 1 && y == 2 {
            vec![
                (0, 2, i),
                (1, 1, i),
                (1, 3, i),
                (0, 0, i + 1),
                (0, 1, i + 1),
                (0, 2, i + 1),
                (0, 3, i + 1),
                (0, 4, i + 1),
            ]
        } else if x == 3 && y == 2 {
            vec![
                (4, 2, i),
                (3, 1, i),
                (3, 3, i),
                (4, 0, i + 1),
                (4, 1, i + 1),
                (4, 2, i + 1),
                (4, 3, i + 1),
                (4, 4, i + 1),
            ]
        } else if x == 2 && y == 1 {
            vec![
                (2, 0, i),
                (1, 1, i),
                (3, 1, i),
                (0, 0, i + 1),
                (1, 0, i + 1),
                (2, 0, i + 1),
                (3, 0, i + 1),
                (4, 0, i + 1),
            ]
        } else if x == 2 && y == 3 {
            vec![
                (2, 4, i),
                (1, 3, i),
                (3, 3, i),
                (0, 4, i + 1),
                (1, 4, i + 1),
                (2, 4, i + 1),
                (3, 4, i + 1),
                (4, 4, i + 1),
            ]
        } else {
            let l = if x == 0 { (1, 2, i - 1) } else { (x - 1, y, i) };
            let r = if x == 4 { (3, 2, i - 1) } else { (x + 1, y, i) };
            let u = if y == 0 { (2, 1, i - 1) } else { (x, y - 1, i) };
            let d = if y == 4 { (2, 3, i - 1) } else { (x, y + 1, i) };

            vec![l, r, u, d]
        };

        checks.iter().fold(0, |a, (x, y, i)| {
            if self.grids.index_range().contains(i) {
                a + if self.grids[*i].read(*x, *y).is_some() {
                    1
                } else {
                    0
                }
            } else {
                a
            }
        })
    }

    fn step(&self) -> Self {
        let mut result = self.clone();
        result.ensure_enough_layers();

        for i in self.grids.index_range() {
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }

                    let neighbors = self.neighbors_at(i, x, y);

                    if self.grids[i].read(x, y).is_some() {
                        if neighbors != 1 {
                            result.grids[i].erase(x, y);
                        }
                    } else if neighbors == 1 || neighbors == 2 {
                        result.grids[i].write(x, y, ());
                    }
                }
            }
        }

        result
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for i in self.grids.index_range() {
            let txt = self.grids[i].render_to_string(false, ".", |_| String::from("#"));
            println!("Depth {}:\n{}\n", i, txt);
        }
    }

    fn count_bugs(&self) -> u64 {
        let mut bug_count = 0u64;

        for i in self.grids.index_range() {
            for x in self.grids[i].x_range() {
                for y in self.grids[i].y_range() {
                    if self.grids[i].read(x, y).is_some() {
                        bug_count += 1;
                    }
                }
            }
        }

        bug_count
    }

    pub fn solve(input: &Vec<Vec<char>>) -> u64 {
        let mut world = RecursiveWorld::new(&input);

        for _ in 0..200 {
            world = world.step();
        }

        world.count_bugs()
    }
}

pub fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("data/day24.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let result0 = World::solve(&input);
    let result1 = RecursiveWorld::solve(&input);

    println!("{} {}", result0, result1);
}
