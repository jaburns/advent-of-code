use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(FromPrimitive, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty = 0,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub struct Game {
    pub tiles: Vec<Vec<Tile>>,
    ball_pos: (i32, i32),
    paddle_pos: (i32, i32),
    score: i64,
}

fn expand_to_contain<T, F>(vec: &mut Vec<Vec<T>>, x: usize, y: usize, fill: F)
where
    F: Fn() -> T,
{
    let original_width = vec.len();

    while x >= vec.len() {
        let mut new_column = Vec::<T>::with_capacity(y + 1);
        for _ in 0..=y {
            new_column.push(fill());
        }
        vec.push(new_column);
    }

    #[allow(clippy::needless_range_loop)] 
    for i in 0..original_width {
        while y >= vec[i].len() {
            vec[i].push(fill());
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: Vec::<Vec<Tile>>::new(),
            ball_pos: (0i32, 0i32),
            paddle_pos: (0i32, 0i32),
            score: 0,
        }
    }

    pub fn load_state_stream(&mut self, serialized: &[i64]) {
        for i in 0..(serialized.len() / 3) {
            self.write_state(
                serialized[3 * i],
                serialized[3 * i + 1],
                serialized[3 * i + 2],
            );
        }
    }

    pub fn write_state(&mut self, x: i64, y: i64, t: i64) {
        if x < 0 {
            self.score = t;
            return;
        }

        let tile: Tile = FromPrimitive::from_i64(t).unwrap();

        match tile {
            Tile::Ball => self.ball_pos = (x as i32, y as i32),
            Tile::Paddle => self.paddle_pos = (x as i32, y as i32),
            _ => (),
        };

        expand_to_contain(&mut self.tiles, x as usize, y as usize, || Tile::Empty);
        self.tiles[x as usize][y as usize] = tile;
    }

    pub fn width(&self) -> usize {
        self.tiles.len()
    }

    pub fn height(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn map_tiles<T, F>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(Tile) -> T,
    {
        let mut result: Vec<Vec<T>> = Vec::new();

        for x in 0..self.width() {
            let mut col = Vec::<T>::new();
            for y in 0..self.height() {
                col.push(f(self.tiles[x][y]));
            }
            result.push(col);
        }

        result
    }

    pub fn get_best_joystick_dir(&self) -> i64 {
        (self.ball_pos.0 - self.paddle_pos.0).signum() as i64
    }

    pub fn count_tiles(&self, kind: Tile) -> usize {
        let mut count = 0usize;

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.tiles[x][y] == kind {
                    count += 1;
                }
            }
        }

        count
    }
}
