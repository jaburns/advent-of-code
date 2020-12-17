#![allow(dead_code)]

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

// ===================================================================================================================
// TODO the Expanse 2, 3, 4 definitions should be macro'd

#[derive(Debug, Clone)]
pub struct Expanse2<T> {
    grid: TwoVec<TwoVec<Option<T>>>,
}

impl<T> Expanse2<T> {
    pub fn new() -> Self {
        Self {
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

    pub fn render_to_string<F>(&self, empty: &str, f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut result = String::new();

        for y in self.y_range() {
            for x in self.x_range() {
                match self.read(x, y) {
                    Some(x) => result.push_str(f(x).as_str()),
                    None => result.push_str(empty),
                };
            }
            result.push_str("\n");
        }

        result
    }
}

impl<T: Clone> Expanse2<T> {
    #[allow(dead_code)]
    pub fn map<F, U>(&self, f: F) -> Expanse2<U>
    where
        F: Fn(&T) -> U,
    {
        let mut new_expanse = Expanse2::<U>::new();

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

// ===================================================================================================================

#[derive(Debug, Clone)]
pub struct Expanse3<T> {
    grid: TwoVec<TwoVec<TwoVec<Option<T>>>>,
}

impl<T> Expanse3<T> {
    pub fn new() -> Self {
        Self {
            grid: TwoVec::new(),
        }
    }

    pub fn read(&self, x: i32, y: i32, z: i32) -> Option<&T> {
        if self.grid.index_range().contains(&x)
            && self.grid[x].index_range().contains(&y)
            && self.grid[x][y].index_range().contains(&z)
        {
            self.grid[x][y][z].as_ref()
        } else {
            None
        }
    }

    pub fn at(&mut self, x: i32, y: i32, z: i32) -> Option<&mut T> {
        if self.grid.index_range().contains(&x)
            && self.grid[x].index_range().contains(&y)
            && self.grid[x][y].index_range().contains(&z)
        {
            self.grid[x][y][z].as_mut()
        } else {
            None
        }
    }

    pub fn write(&mut self, x: i32, y: i32, z: i32, item: T) {
        self.grid.expand_to_contain(x, || TwoVec::new());
        for i in self.grid.index_range() {
            self.grid[i].expand_to_contain(y, || TwoVec::new());
            for j in self.grid[i].index_range() {
                self.grid[i][j].expand_to_contain(z, || None);
            }
        }
        self.grid[x][y][z] = Some(item);
    }

    pub fn empty(&self) -> bool {
        for x in self.x_range() {
            for y in self.y_range() {
                for z in self.z_range() {
                    if self.read(x, y, z).is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn erase(&mut self, x: i32, y: i32, z: i32) {
        if self.at(x, y, z).is_some() {
            self.grid[x][y][z] = None;
        }
    }

    pub fn find_many<F>(&self, f: F) -> Vec<(i32, i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        let mut result = Vec::<(i32, i32, i32)>::new();

        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                for z in self.grid[x][y].index_range() {
                    match self.read(x, y, z) {
                        Some(w) => {
                            if f(w) {
                                result.push((x, y, z));
                            }
                        }
                        None => (),
                    }
                }
            }
        }

        result
    }

    pub fn find<F>(&self, f: F) -> Option<(i32, i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                for z in self.grid[x][y].index_range() {
                    match self.read(x, y, z) {
                        Some(w) => {
                            if f(w) {
                                return Some((x, y, z));
                            }
                        }
                        None => (),
                    }
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

    pub fn z_range(&self) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else {
            self.grid[0][0].index_range()
        }
    }

    pub fn x_range_plus(&self, radius: i32) -> Range<i32> {
        (self.grid.index_range().min().unwrap() - radius)
            ..(self.grid.index_range().max().unwrap() + radius + 1)
    }

    pub fn y_range_plus(&self, radius: i32) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else {
            (self.grid[0].index_range().min().unwrap() - radius)
                ..(self.grid[0].index_range().max().unwrap() + radius + 1)
        }
    }

    pub fn z_range_plus(&self, radius: i32) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else {
            (self.grid[0][0].index_range().min().unwrap() - radius)
                ..(self.grid[0][0].index_range().max().unwrap() + radius + 1)
        }
    }

    pub fn render_to_string<F>(&self, empty: &str, f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut result = String::new();

        for z in self.z_range() {
            for y in self.y_range() {
                for x in self.x_range() {
                    match self.read(x, y, z) {
                        Some(x) => result.push_str(f(x).as_str()),
                        None => result.push_str(empty),
                    };
                }
                result.push_str("\n");
            }

            result.push_str("\n\n");
        }

        result
    }
}

impl<T: Clone> Expanse3<T> {
    pub fn map<F, U>(&self, f: F) -> Expanse3<U>
    where
        F: Fn(&T) -> U,
    {
        let mut new_expanse = Expanse3::<U>::new();

        for x in self.x_range() {
            for y in self.y_range() {
                for z in self.z_range() {
                    if let Some(cell) = self.read(x, y, z) {
                        new_expanse.write(x, y, z, f(cell));
                    }
                }
            }
        }

        new_expanse
    }
}

// ===================================================================================================================

#[derive(Debug, Clone)]
pub struct Expanse4<T> {
    grid: TwoVec<TwoVec<TwoVec<TwoVec<Option<T>>>>>,
}

impl<T> Expanse4<T> {
    pub fn new() -> Self {
        Self {
            grid: TwoVec::new(),
        }
    }

    pub fn read(&self, x: i32, y: i32, z: i32, w: i32) -> Option<&T> {
        if self.grid.index_range().contains(&x)
            && self.grid[x].index_range().contains(&y)
            && self.grid[x][y].index_range().contains(&z)
            && self.grid[x][y][z].index_range().contains(&w)
        {
            self.grid[x][y][z][w].as_ref()
        } else {
            None
        }
    }

    pub fn at(&mut self, x: i32, y: i32, z: i32, w: i32) -> Option<&mut T> {
        if self.grid.index_range().contains(&x)
            && self.grid[x].index_range().contains(&y)
            && self.grid[x][y].index_range().contains(&z)
            && self.grid[x][y][z].index_range().contains(&w)
        {
            self.grid[x][y][z][w].as_mut()
        } else {
            None
        }
    }

    pub fn write(&mut self, x: i32, y: i32, z: i32, w: i32, item: T) {
        self.grid.expand_to_contain(x, || TwoVec::new());
        for i in self.grid.index_range() {
            self.grid[i].expand_to_contain(y, || TwoVec::new());
            for j in self.grid[i].index_range() {
                self.grid[i][j].expand_to_contain(z, || TwoVec::new());
                for k in self.grid[i][j].index_range() {
                    self.grid[i][j][k].expand_to_contain(w, || None);
                }
            }
        }
        self.grid[x][y][z][w] = Some(item);
    }

    pub fn empty(&self) -> bool {
        for x in self.x_range() {
            for y in self.y_range() {
                for z in self.z_range() {
                    for w in self.w_range() {
                        if self.read(x, y, z, w).is_some() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn erase(&mut self, x: i32, y: i32, z: i32, w: i32) {
        if self.at(x, y, z, w).is_some() {
            self.grid[x][y][z][w] = None;
        }
    }

    pub fn find_many<F>(&self, f: F) -> Vec<(i32, i32, i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        let mut result = Vec::<(i32, i32, i32, i32)>::new();

        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                for z in self.grid[x][y].index_range() {
                    for w in self.grid[x][y][z].index_range() {
                        match self.read(x, y, z, w) {
                            Some(v) => {
                                if f(v) {
                                    result.push((x, y, z, w));
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }

        result
    }

    pub fn find<F>(&self, f: F) -> Option<(i32, i32, i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                for z in self.grid[x][y].index_range() {
                    for w in self.grid[x][y][z].index_range() {
                        match self.read(x, y, z, w) {
                            Some(v) => {
                                if f(v) {
                                    return Some((x, y, z, w));
                                }
                            }
                            None => (),
                        }
                    }
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

    pub fn z_range(&self) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else {
            self.grid[0][0].index_range()
        }
    }

    pub fn w_range(&self) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else if self.grid[0][0].empty() {
            0..0
        } else {
            self.grid[0][0][0].index_range()
        }
    }

    pub fn x_range_plus(&self, radius: i32) -> Range<i32> {
        (self.grid.index_range().min().unwrap() - radius)
            ..(self.grid.index_range().max().unwrap() + radius + 1)
    }

    pub fn y_range_plus(&self, radius: i32) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else {
            (self.grid[0].index_range().min().unwrap() - radius)
                ..(self.grid[0].index_range().max().unwrap() + radius + 1)
        }
    }

    pub fn z_range_plus(&self, radius: i32) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else {
            (self.grid[0][0].index_range().min().unwrap() - radius)
                ..(self.grid[0][0].index_range().max().unwrap() + radius + 1)
        }
    }

    pub fn w_range_plus(&self, radius: i32) -> Range<i32> {
        if self.grid.empty() {
            0..0
        } else if self.grid[0].empty() {
            0..0
        } else if self.grid[0][0].empty() {
            0..0
        } else {
            (self.grid[0][0][0].index_range().min().unwrap() - radius)
                ..(self.grid[0][0][0].index_range().max().unwrap() + radius + 1)
        }
    }

    pub fn render_to_string<F>(&self, empty: &str, f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut result = String::new();

        for w in self.w_range() {
            for z in self.z_range() {
                for y in self.y_range() {
                    for x in self.x_range() {
                        match self.read(x, y, z, w) {
                            Some(x) => result.push_str(f(x).as_str()),
                            None => result.push_str(empty),
                        };
                    }
                    result.push_str("\n");
                }

                result.push_str("\n\n");
            }

            result.push_str("\n-------------------------------------------------------------\n");
        }

        result
    }
}

impl<T: Clone> Expanse4<T> {
    pub fn map<F, U>(&self, f: F) -> Expanse4<U>
    where
        F: Fn(&T) -> U,
    {
        let mut new_expanse = Expanse4::<U>::new();

        for x in self.x_range() {
            for y in self.y_range() {
                for z in self.z_range() {
                    for w in self.w_range() {
                        if let Some(cell) = self.read(x, y, z, w) {
                            new_expanse.write(x, y, z, w, f(cell));
                        }
                    }
                }
            }
        }

        new_expanse
    }
}
