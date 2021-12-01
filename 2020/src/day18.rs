use crate::day18_gen::{eval_data_part1, eval_data_part2};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Val1(pub i64);

impl Add for Val1 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Val1(a) = self;
        let Val1(b) = other;
        Val1(a + b)
    }
}

impl Sub for Val1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let Val1(a) = self;
        let Val1(b) = other;
        Val1(a * b)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Val2(pub i64);

impl Add for Val2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Val2(a) = self;
        let Val2(b) = other;
        Val2(a * b)
    }
}

impl Mul for Val2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let Val2(a) = self;
        let Val2(b) = other;
        Val2(a + b)
    }
}

pub fn main() {
    let Val1(part1) = eval_data_part1();
    let Val2(part2) = eval_data_part2();

    println!("{} {}", part1, part2);
}
