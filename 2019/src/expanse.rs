use std::clone::Clone;
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone)]
pub struct TwoVec<T> {
    negative: Vec<T>,
    positive: Vec<T>,
}

impl<T> TwoVec<T> {
    pub fn new() -> TwoVec<T> {
        TwoVec {
            negative: Vec::<T>::new(),
            positive: Vec::<T>::new(),
        }
    }

    pub fn expand_to_contain<F>(&mut self, index: i32, fill: F)
    where
        F: Fn() -> T,
    {
        if index < 0 {
            while (self.negative.len() as i32) < -index {
                self.negative.push(fill());
            }
        } else {
            while (self.positive.len() as i32) < index + 1 {
                self.positive.push(fill());
            }
        }
    }

    pub fn index_range(&self) -> Range<i32> {
        Range {
            start: -(self.negative.len() as i32),
            end: self.positive.len() as i32,
        }
    }

    pub fn empty(&self) -> bool {
        self.negative.len() + self.positive.len() == 0
    }
}

impl<T> Index<i32> for TwoVec<T> {
    type Output = T;

    fn index(&self, i: i32) -> &T {
        if i < 0 {
            &self.negative[-i as usize - 1]
        } else {
            &self.positive[i as usize]
        }
    }
}

impl<T> IndexMut<i32> for TwoVec<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
        if i < 0 {
            &mut self.negative[-i as usize - 1]
        } else {
            &mut self.positive[i as usize]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expanse<T> {
    grid: TwoVec<TwoVec<Option<T>>>,
}

impl<T> Expanse<T> {
    pub fn new() -> Expanse<T> {
        Expanse {
            grid: TwoVec::new(),
        }
    }

    pub fn read(&self, x: i32, y: i32) -> Option<&T> {
        if self.grid.index_range().contains(&x) && self.grid[x].index_range().contains(&y) {
            self.grid[x][y].as_ref()
        } else {
            None
        }
    }

    pub fn at(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if self.grid.index_range().contains(&x) && self.grid[x].index_range().contains(&y) {
            self.grid[x][y].as_mut()
        } else {
            None
        }
    }

    pub fn write(&mut self, x: i32, y: i32, item: T) {
        self.grid.expand_to_contain(x, || TwoVec::new());
        for i in self.grid.index_range() {
            self.grid[i].expand_to_contain(y, || None);
        }
        self.grid[x][y] = Some(item);
    }

    pub fn empty(&self) -> bool {
        for x in self.x_range() {
            for y in self.y_range() {
                if self.read(x, y).is_some() {
                    return false;
                }
            }
        }
        true
    }

    pub fn erase(&mut self, x: i32, y: i32) {
        if self.at(x, y).is_some() {
            self.grid[x][y] = None;
        }
    }

    pub fn find_many<F>(&self, f: F) -> Vec<(i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        let mut result = Vec::<(i32, i32)>::new();

        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                match self.read(x, y) {
                    Some(z) => {
                        if f(z) {
                            result.push((x, y));
                        }
                    }
                    None => (),
                }
            }
        }

        result
    }

    pub fn find<F>(&self, f: F) -> Option<(i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                match self.read(x, y) {
                    Some(z) => {
                        if f(z) {
                            return Some((x, y));
                        }
                    }
                    None => (),
                }
            }
        }
        None
    }

    pub fn x_range(&self) -> Range<i32> {
        self.grid.index_range()
    }

    pub fn y_range(&self) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else {
            self.grid[0].index_range()
        }
    }

    pub fn render_to_string<F>(&self, y_increases_up: bool, empty: &str, f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        fn inner_fn<T, G, I: Iterator<Item = i32>>(
            this: &Expanse<T>,
            empty: &str,
            f: G,
            y_range: I,
        ) -> String
        where
            G: Fn(&T) -> String,
        {
            let mut result = String::new();

            for y in y_range {
                for x in this.x_range() {
                    match this.read(x, y) {
                        Some(x) => result.push_str(f(x).as_str()),
                        None => result.push_str(empty),
                    };
                }
                result.push_str("\n");
            }

            result
        }

        if y_increases_up {
            inner_fn(self, empty, f, self.y_range().rev())
        } else {
            inner_fn(self, empty, f, self.y_range())
        }
    }
}

impl<T: Clone> Expanse<T> {
    #[allow(dead_code)]
    pub fn map<F, U>(&self, f: F) -> Expanse<U>
    where
        F: Fn(&T) -> U,
    {
        let mut new_expanse = Expanse::<U>::new();

        for x in self.x_range() {
            for y in self.y_range() {
                if let Some(cell) = self.read(x, y) {
                    new_expanse.write(x, y, f(cell));
                }
            }
        }

        new_expanse
    }
}
