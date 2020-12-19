use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Val1(i64);

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
struct Val2(i64);

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

// cat data/day18.txt | sed 's/\([0-9]\)/Val1(\1)/g;s/^/( /;s/$/ ) \+/;s/\*/-/g'
#[rustfmt::skip]
fn eval_data_part1() -> Val1 {
( (Val1(9) - Val1(3) + Val1(4)) - (Val1(7) + (Val1(2) + Val1(3) + Val1(6) - Val1(2)) + Val1(3)) - Val1(4) - Val1(5) ) +
( (Val1(8) + Val1(4) + Val1(9) + (Val1(7) - Val1(9) + Val1(4) + Val1(6) + Val1(9)) - Val1(3)) + Val1(6) + Val1(5) + ((Val1(8) - Val1(2)) + Val1(2) - Val1(9)) - Val1(5) ) +
( Val1(5) - Val1(6) - Val1(8) - (Val1(9) + Val1(7) - Val1(8) - Val1(9)) + Val1(6) + Val1(2) ) +
( Val1(9) - Val1(9) + (Val1(4) - (Val1(6) - Val1(7) - Val1(7) + Val1(4) - Val1(6)) - Val1(5) + Val1(6) - (Val1(2) - Val1(5))) - (Val1(8) + Val1(7) + Val1(6) + Val1(2) - Val1(7)) + Val1(5) ) +
( Val1(5) + (Val1(7) + Val1(3) - Val1(7) + Val1(9) - (Val1(7) + Val1(7) + Val1(4) + Val1(3) - Val1(3))) + Val1(8) - Val1(4) ) +
( (Val1(6) + Val1(2) + Val1(5) + Val1(4)) - Val1(2) - Val1(9) - Val1(2) - Val1(8) ) +
( Val1(7) + Val1(8) + ((Val1(4) + Val1(6) + Val1(6)) - Val1(6) + Val1(9) + Val1(3) - Val1(8) + Val1(9)) + Val1(5) ) +
( Val1(3) + ((Val1(6) - Val1(6) - Val1(7) - Val1(6)) - Val1(6) + Val1(6) - Val1(4)) + (Val1(2) + Val1(5) - Val1(6) + Val1(6) + (Val1(2) - Val1(4)) + Val1(6)) - Val1(9) ) +
( (Val1(2) - (Val1(4) - Val1(9) - Val1(3)) - Val1(8) + Val1(6) + (Val1(7) + Val1(2) + Val1(6))) - Val1(6) - (Val1(5) - Val1(4) + Val1(8) + Val1(7) + (Val1(7) + Val1(2) - Val1(3))) ) +
( Val1(8) - ((Val1(7) + Val1(4) + Val1(9)) - Val1(5) + Val1(6) + (Val1(4) + Val1(9) + Val1(8) - Val1(3) - Val1(8) + Val1(3)) - Val1(5) - (Val1(5) + Val1(8) + Val1(7) + Val1(3) - Val1(7) - Val1(4))) ) +
( Val1(6) - Val1(5) - Val1(5) + (Val1(8) - Val1(5) - Val1(8) - Val1(5) + Val1(7)) + ((Val1(2) + Val1(2) - Val1(6) - Val1(3)) - Val1(4)) - Val1(6) ) +
( Val1(2) + Val1(2) - Val1(9) + Val1(8) - (Val1(3) - Val1(7) - Val1(7) - (Val1(2) + Val1(5)) - Val1(3) + Val1(2)) ) +
( Val1(6) + Val1(2) ) +
( Val1(9) - Val1(2) + ((Val1(6) - Val1(7) - Val1(7) - Val1(2)) + Val1(8)) + (Val1(7) - (Val1(4) - Val1(6) + Val1(7)) - Val1(2) - Val1(2) - Val1(9)) + Val1(4) - Val1(8) ) +
( (Val1(7) - Val1(9) + Val1(9) - Val1(2) - Val1(4) - Val1(9)) + Val1(4) - Val1(2) - Val1(9) + Val1(6) ) +
( Val1(6) - Val1(3) + Val1(7) + Val1(9) + Val1(9) + ((Val1(6) + Val1(6) + Val1(7) - Val1(5)) - Val1(6) - Val1(8) - Val1(5)) ) +
( ((Val1(8) - Val1(8) - Val1(6) + Val1(6) - Val1(4)) + Val1(3) + Val1(3) + (Val1(3) - Val1(5) + Val1(3) - Val1(5) + Val1(4) + Val1(7))) - Val1(8) + (Val1(6) + Val1(7) - Val1(2) + Val1(2)) + (Val1(9) + Val1(9)) ) +
( Val1(7) + Val1(8) + Val1(3) - Val1(5) + (Val1(5) - Val1(5) + Val1(6) + Val1(6)) - (Val1(9) - Val1(6)) ) +
( Val1(5) + Val1(5) + Val1(2) ) +
( ((Val1(3) + Val1(3)) - Val1(2) + (Val1(5) + Val1(8) - Val1(2) + Val1(4) - Val1(3))) - Val1(8) ) +
( ((Val1(5) + Val1(6) + Val1(5) - Val1(8) + Val1(4)) + Val1(9) + (Val1(5) + Val1(5))) - (Val1(7) - Val1(2) + (Val1(8) + Val1(7))) + Val1(9) ) +
( Val1(9) - (Val1(9) - Val1(8)) + (Val1(3) - Val1(2) - (Val1(9) + Val1(2) - Val1(9) - Val1(5)) - Val1(4) - Val1(2)) + Val1(5) ) +
( Val1(4) - Val1(4) + (Val1(2) + Val1(5) + Val1(5) - (Val1(3) + Val1(6) - Val1(4) + Val1(7))) - (Val1(2) - Val1(2)) + Val1(6) ) +
( Val1(6) + Val1(2) ) +
( Val1(6) + Val1(5) + (Val1(4) + Val1(8)) + Val1(5) ) +
( Val1(6) - (Val1(4) - Val1(6)) + (Val1(4) - Val1(5) + Val1(4) + Val1(5) + Val1(9)) + Val1(7) - (Val1(2) + Val1(7) + Val1(9)) ) +
( Val1(9) - (Val1(3) - (Val1(5) + Val1(9) - Val1(4) - Val1(5) + Val1(3) - Val1(9))) + ((Val1(4) - Val1(4) - Val1(3) - Val1(6)) - (Val1(9) - Val1(8) - Val1(3)) - Val1(9) - Val1(9)) + Val1(3) ) +
( Val1(5) + (Val1(6) - (Val1(3) + Val1(9) + Val1(7) - Val1(6) + Val1(2)) - (Val1(2) + Val1(4) - Val1(2) - Val1(4) - Val1(6)) - Val1(9) + Val1(3)) + Val1(6) + Val1(8) + Val1(5) - Val1(3) ) +
( Val1(8) + Val1(9) - (Val1(8) + (Val1(3) + Val1(2) + Val1(4)) - Val1(4) + Val1(3) - Val1(4) - Val1(5)) ) +
( Val1(9) + Val1(5) + Val1(8) + Val1(9) - (Val1(8) + (Val1(4) - Val1(2) - Val1(9) + Val1(8) + Val1(7)) - (Val1(8) + Val1(6) - Val1(3)) - Val1(7) - Val1(7)) ) +
( Val1(6) - ((Val1(9) + Val1(2)) - (Val1(7) + Val1(8) + Val1(5) + Val1(9) - Val1(7)) - Val1(4)) - Val1(4) - ((Val1(7) + Val1(7) + Val1(4) - Val1(5) - Val1(2)) + Val1(2)) + Val1(9) ) +
( (Val1(4) + Val1(9) - Val1(8)) - (Val1(6) + (Val1(7) - Val1(6)) - Val1(9) + Val1(8) - Val1(2) - Val1(7)) + Val1(4) + Val1(9) + Val1(3) - (Val1(6) + Val1(8) + (Val1(8) - Val1(4) + Val1(7) - Val1(3)) - Val1(8)) ) +
( (Val1(5) - Val1(9) - Val1(2) - Val1(7) - Val1(8) + Val1(6)) + (Val1(5) + Val1(5)) ) +
( Val1(6) + Val1(7) - Val1(9) + Val1(3) ) +
( ((Val1(7) + Val1(8) - Val1(3) - Val1(7) + Val1(5)) - Val1(7) - Val1(5) + Val1(6) - Val1(6) + Val1(5)) + Val1(9) + Val1(4) ) +
( (Val1(5) - Val1(2) - Val1(4) - Val1(9) - Val1(2) + (Val1(8) + Val1(6) - Val1(7) + Val1(6))) + ((Val1(4) - Val1(4) - Val1(6) - Val1(9) + Val1(7)) + Val1(9) - Val1(7)) ) +
( (Val1(3) + (Val1(5) - Val1(8) + Val1(4) + Val1(4) - Val1(7))) - Val1(7) + Val1(8) + (Val1(7) + (Val1(3) - Val1(5) - Val1(4) + Val1(6) + Val1(9))) + ((Val1(7) + Val1(2) + Val1(7) - Val1(9) + Val1(6) - Val1(8)) + Val1(3) + Val1(3) + (Val1(2) - Val1(9) - Val1(7)) + Val1(4) + (Val1(6) + Val1(3) - Val1(5))) - Val1(9) ) +
( Val1(5) - Val1(4) + ((Val1(6) + Val1(8) - Val1(5) - Val1(6)) - Val1(8)) + Val1(7) + Val1(7) ) +
( Val1(5) - (Val1(4) - (Val1(2) - Val1(9) - Val1(7) - Val1(7)) - Val1(5)) + ((Val1(6) - Val1(2) - Val1(9) + Val1(2)) - Val1(3) - Val1(2) + Val1(7) + Val1(3)) + Val1(9) + Val1(8) ) +
( (Val1(7) + Val1(3) - Val1(3) - Val1(6) - Val1(2) - Val1(3)) - Val1(6) + Val1(3) - ((Val1(3) - Val1(4) - Val1(3) - Val1(9) - Val1(7)) + Val1(9) + Val1(8)) - Val1(9) ) +
( ((Val1(4) + Val1(8) - Val1(9)) + Val1(5) - Val1(4) - Val1(2) - Val1(8) - Val1(4)) - (Val1(6) + Val1(2) - (Val1(9) + Val1(5)) - Val1(9) - Val1(7) + Val1(8)) ) +
( Val1(9) + Val1(8) + (Val1(3) - Val1(3) - (Val1(9) + Val1(2) + Val1(8) - Val1(7) - Val1(9))) ) +
( (Val1(5) - Val1(8) - (Val1(7) - Val1(2) + Val1(8))) + Val1(2) - Val1(8) + Val1(8) + Val1(6) - Val1(5) ) +
( Val1(2) + (Val1(2) - Val1(9) - Val1(5) + Val1(4)) - (Val1(5) + Val1(9) - Val1(3) - Val1(4)) - Val1(4) + Val1(2) ) +
( Val1(8) - Val1(2) - (Val1(7) - Val1(7) - Val1(4) + Val1(5) + Val1(9) + Val1(4)) ) +
( Val1(4) - Val1(9) + Val1(2) - Val1(8) ) +
( Val1(6) + Val1(2) + ((Val1(5) + Val1(3) - Val1(5) + Val1(7) + Val1(9)) + Val1(4) - Val1(6) + Val1(6)) - Val1(5) ) +
( (Val1(8) - Val1(3) - Val1(7)) + (Val1(4) - Val1(5) + Val1(8) - Val1(7) + Val1(6)) ) +
( Val1(3) - Val1(7) - ((Val1(3) + Val1(3) - Val1(5) + Val1(9) + Val1(7)) + Val1(9) - Val1(2) + (Val1(4) + Val1(3))) ) +
( (Val1(4) + Val1(2) - Val1(2) + Val1(3) - Val1(2) + Val1(2)) - Val1(4) + Val1(2) - Val1(4) + Val1(6) + Val1(3) ) +
( ((Val1(9) - Val1(7) - Val1(7) + Val1(8)) - (Val1(2) - Val1(5) + Val1(2) + Val1(3)) - Val1(2) - Val1(4) - Val1(7)) - Val1(9) + Val1(5) + Val1(5) ) +
( Val1(4) + Val1(9) - Val1(2) + Val1(5) + (Val1(3) + Val1(5) + (Val1(4) + Val1(3) - Val1(3) - Val1(7) - Val1(5) - Val1(8)) + Val1(9) - Val1(2) - Val1(2)) - (Val1(4) + (Val1(2) - Val1(6)) + Val1(7) - Val1(5) - Val1(7) + Val1(2)) ) +
( Val1(6) + Val1(2) - Val1(4) - (Val1(6) + Val1(8)) - (Val1(4) + Val1(2)) - Val1(3) ) +
( Val1(3) + ((Val1(8) + Val1(8) + Val1(6)) - Val1(5)) + Val1(9) + Val1(7) + ((Val1(7) - Val1(2) - Val1(9) + Val1(5) - Val1(3) + Val1(5)) + Val1(3) + (Val1(2) - Val1(5)) + Val1(3) + Val1(5)) ) +
( Val1(4) + Val1(2) - Val1(6) + Val1(6) + Val1(6) ) +
( (Val1(5) - (Val1(4) - Val1(9) - Val1(8) + Val1(5) - Val1(8)) - Val1(6) - Val1(9) + Val1(3)) - ((Val1(2) - Val1(7) - Val1(3) + Val1(3) - Val1(4) - Val1(3)) + Val1(9) + Val1(8) + Val1(8)) + Val1(4) ) +
( Val1(4) - Val1(4) + Val1(9) - Val1(9) + (Val1(2) - (Val1(6) - Val1(5) + Val1(9) + Val1(7) - Val1(3)) + (Val1(2) - Val1(8) + Val1(3) + Val1(2) + Val1(3))) ) +
( Val1(6) - Val1(4) + Val1(5) - (Val1(6) - (Val1(6) - Val1(7) + Val1(2) - Val1(6) + Val1(4) - Val1(7))) - Val1(4) + ((Val1(5) - Val1(9) - Val1(8) - Val1(9) - Val1(5) - Val1(5)) - Val1(5)) ) +
( Val1(3) + (Val1(5) - Val1(4) + (Val1(3) + Val1(6) - Val1(2) + Val1(8) + Val1(7) - Val1(8)) + Val1(3) + (Val1(9) + Val1(8) - Val1(6))) - (Val1(5) + Val1(6) - (Val1(8) - Val1(9) + Val1(4) - Val1(9) + Val1(5)) - Val1(7) - Val1(5)) + Val1(5) ) +
( Val1(9) + Val1(2) + Val1(9) + (Val1(3) + Val1(9)) + (Val1(4) - Val1(4) + Val1(7) - Val1(5)) + Val1(5) ) +
( Val1(5) + Val1(6) ) +
( Val1(6) + (Val1(7) + Val1(2) - Val1(6) - Val1(9) - Val1(7) + Val1(5)) - Val1(9) + (Val1(4) - Val1(5) - Val1(8)) + Val1(2) - Val1(3) ) +
( Val1(8) + Val1(7) - (Val1(9) + (Val1(9) + Val1(3) - Val1(3) - Val1(7)) - (Val1(5) + Val1(6) - Val1(3) - Val1(7) + Val1(8) - Val1(2))) - Val1(7) ) +
( (Val1(5) - Val1(7) + Val1(5) + Val1(3) + Val1(6)) - Val1(8) ) +
( Val1(5) + ((Val1(9) + Val1(5)) + Val1(9) + Val1(4) - (Val1(5) - Val1(4) - Val1(7) + Val1(3)) - Val1(3)) ) +
( Val1(5) - Val1(8) - Val1(7) + Val1(8) - ((Val1(8) + Val1(9) + Val1(3) - Val1(6) - Val1(5)) - Val1(4) - Val1(9) + Val1(4)) ) +
( Val1(8) - (Val1(6) + (Val1(6) + Val1(9) - Val1(7) - Val1(8) - Val1(7)) + Val1(7) - Val1(2)) + ((Val1(2) - Val1(9)) - Val1(7)) + Val1(4) ) +
( Val1(6) + ((Val1(4) + Val1(5) + Val1(6) + Val1(7)) - Val1(7)) ) +
( Val1(8) - (Val1(5) - Val1(6) - Val1(4) - Val1(8) + Val1(6) - Val1(6)) + Val1(2) - Val1(9) ) +
( Val1(7) + Val1(3) + Val1(2) - (Val1(6) + Val1(8)) ) +
( Val1(9) - ((Val1(4) + Val1(4) - Val1(9) + Val1(6) - Val1(8) - Val1(7)) + Val1(9) + Val1(2) + Val1(6) - (Val1(4) - Val1(9) + Val1(7) + Val1(6)) - Val1(3)) - Val1(5) + Val1(7) ) +
( ((Val1(2) + Val1(3) - Val1(2) - Val1(9) + Val1(6)) + Val1(4) + Val1(8) - Val1(2) - Val1(5) + Val1(8)) - Val1(5) + Val1(5) + (Val1(9) - Val1(6)) + Val1(9) + (Val1(3) - Val1(2) - Val1(8)) ) +
( Val1(4) + (Val1(6) + Val1(2)) - Val1(9) + (Val1(5) + Val1(4) - (Val1(8) - Val1(3) + Val1(7) + Val1(6)) - Val1(4)) - ((Val1(7) + Val1(2) + Val1(3)) - Val1(6) + Val1(9) - (Val1(6) + Val1(7) - Val1(2))) ) +
( Val1(6) + Val1(2) + ((Val1(2) - Val1(4) - Val1(8)) - Val1(2) - Val1(4) - Val1(7) + Val1(9)) - (Val1(7) + Val1(8) + (Val1(6) + Val1(7) - Val1(9) + Val1(9) - Val1(7)) + Val1(4)) - Val1(8) ) +
( Val1(9) - Val1(9) + Val1(7) + Val1(9) - (Val1(9) - Val1(4)) - Val1(9) ) +
( (Val1(7) - Val1(3) + (Val1(3) - Val1(7) + Val1(4) - Val1(4) - Val1(8)) - Val1(2) - Val1(2) - (Val1(3) - Val1(3))) + Val1(8) ) +
( Val1(9) - Val1(6) + Val1(3) - (Val1(2) - Val1(5) - Val1(7) - Val1(7) + Val1(7)) - Val1(8) + (Val1(2) + Val1(4)) ) +
( (Val1(8) - Val1(9)) + Val1(8) - Val1(6) ) +
( (Val1(3) - Val1(2) + Val1(6) + (Val1(7) - Val1(8) - Val1(5) - Val1(4))) + Val1(5) - Val1(6) ) +
( Val1(4) + Val1(3) + (Val1(9) + Val1(3) + (Val1(2) - Val1(9) - Val1(6)) + Val1(7)) - Val1(8) + Val1(3) - Val1(8) ) +
( Val1(9) + Val1(6) - Val1(8) + Val1(4) + Val1(9) - (Val1(9) - (Val1(8) - Val1(4) + Val1(9)) + Val1(3)) ) +
( Val1(5) + Val1(4) - Val1(6) - (Val1(8) - (Val1(2) + Val1(9) + Val1(6)) - (Val1(6) - Val1(3) + Val1(2)) + Val1(6)) - (Val1(8) - Val1(8) - Val1(3) - (Val1(6) - Val1(9)) - Val1(3) + (Val1(4) + Val1(4) + Val1(2) - Val1(2) - Val1(6))) + Val1(6) ) +
( Val1(4) + (Val1(6) + Val1(6) - (Val1(4) - Val1(7) - Val1(7)) + Val1(5)) - Val1(5) + Val1(6) - (Val1(7) + Val1(2) - Val1(4) - Val1(2) + Val1(9) + Val1(9)) ) +
( ((Val1(8) + Val1(7)) - (Val1(7) + Val1(5) - Val1(9)) + (Val1(9) + Val1(2) - Val1(6) - Val1(2)) + Val1(6) - Val1(7)) - (Val1(3) + Val1(3) - Val1(2) + Val1(3) + Val1(7)) - Val1(4) + Val1(7) - Val1(7) ) +
( (Val1(6) + Val1(9) - Val1(7) - Val1(4) + Val1(7)) - Val1(6) + Val1(7) + Val1(5) + Val1(3) + Val1(8) ) +
( Val1(9) + Val1(2) + (Val1(9) - (Val1(8) - Val1(9) - Val1(2) - Val1(6) + Val1(2) + Val1(5))) + Val1(6) + Val1(4) + Val1(8) ) +
( Val1(6) + Val1(4) - ((Val1(2) - Val1(9)) - Val1(6) - Val1(5)) + Val1(2) - Val1(9) ) +
( (Val1(6) + Val1(8) - Val1(2) + (Val1(5) + Val1(2) - Val1(7) - Val1(7)) - Val1(5) + Val1(4)) - (Val1(2) - Val1(3) + Val1(8) - (Val1(9) + Val1(6) - Val1(4) + Val1(3) + Val1(6) + Val1(4)) - (Val1(2) + Val1(5) - Val1(8) - Val1(2) + Val1(4))) ) +
( Val1(3) + (Val1(9) - (Val1(2) - Val1(7) - Val1(6)) - Val1(9) + Val1(7) + Val1(3)) + (Val1(7) - (Val1(5) - Val1(6) + Val1(7)) + Val1(3) - (Val1(5) - Val1(7) - Val1(5) - Val1(4)) - Val1(8) + Val1(6)) - Val1(4) - Val1(5) + Val1(9) ) +
( (Val1(6) + Val1(6) + Val1(4)) + Val1(3) - Val1(4) + Val1(3) ) +
( Val1(9) - (Val1(2) - Val1(7) + Val1(9) + (Val1(4) + Val1(2)) - Val1(5)) + (Val1(2) + Val1(5) + (Val1(7) + Val1(6))) - Val1(8) ) +
( Val1(2) - (Val1(6) - Val1(6) - Val1(9) + Val1(8)) + Val1(4) - Val1(2) ) +
( ((Val1(4) - Val1(3)) + Val1(8)) + Val1(5) + Val1(5) ) +
( Val1(5) + Val1(8) - Val1(7) + Val1(6) + Val1(6) - ((Val1(9) - Val1(2) - Val1(5) + Val1(4) - Val1(7)) + Val1(8) + Val1(6) + Val1(5) + (Val1(3) + Val1(7))) ) +
( Val1(8) + (Val1(8) - Val1(8) - Val1(8) - (Val1(3) - Val1(7) - Val1(8)) - Val1(4)) - Val1(8) + Val1(5) - Val1(8) - Val1(3) ) +
( (Val1(8) - Val1(3) - Val1(8)) - Val1(4) - Val1(3) - Val1(3) - Val1(6) ) +
( Val1(5) + Val1(6) - (Val1(9) + (Val1(5) + Val1(6) + Val1(8) + Val1(3) - Val1(4)) - (Val1(6) - Val1(3) + Val1(3) + Val1(4) - Val1(6) + Val1(2))) ) +
( Val1(3) - (Val1(2) - (Val1(5) - Val1(9)) - Val1(5) + Val1(8) + Val1(5) - Val1(6)) - Val1(8) + (Val1(8) + Val1(5) + Val1(7) - (Val1(6) + Val1(5) + Val1(9) - Val1(2) - Val1(9)) + Val1(2) - (Val1(3) - Val1(2) - Val1(2) + Val1(9) - Val1(4))) ) +
( (Val1(2) - Val1(8) + Val1(5) + Val1(7) + (Val1(8) + Val1(2) + Val1(7) - Val1(5) - Val1(5) + Val1(4)) - Val1(2)) + Val1(5) - ((Val1(8) - Val1(7) + Val1(5)) - Val1(3) - Val1(4) + (Val1(4) - Val1(7) - Val1(6)) - (Val1(7) + Val1(6) + Val1(4) + Val1(7) - Val1(7)) + (Val1(8) - Val1(7) + Val1(2) - Val1(2) + Val1(9) + Val1(3))) + ((Val1(4) + Val1(8)) + Val1(9)) ) +
( Val1(3) - (Val1(8) + (Val1(2) + Val1(9)) + Val1(2) - Val1(8) + (Val1(9) - Val1(4) - Val1(9))) - Val1(2) - Val1(3) - Val1(4) ) +
( (Val1(4) + (Val1(9) - Val1(4)) + Val1(5)) - Val1(8) - (Val1(3) + Val1(7) - Val1(3) - Val1(7) + Val1(4)) - Val1(7) - Val1(2) ) +
( Val1(2) - Val1(4) - (Val1(2) - Val1(8) + Val1(9) + Val1(7) + (Val1(8) - Val1(3) - Val1(6) + Val1(3) + Val1(5)) + Val1(9)) + (Val1(5) - Val1(8) - Val1(8) + (Val1(4) - Val1(5) - Val1(2) + Val1(4) - Val1(2) + Val1(5)) - Val1(4)) + Val1(2) - Val1(6) ) +
( Val1(7) + Val1(8) + ((Val1(5) + Val1(7) + Val1(2) + Val1(9) - Val1(4) - Val1(3)) - Val1(4)) + Val1(7) + Val1(6) + Val1(8) ) +
( ((Val1(4) + Val1(4) - Val1(4) - Val1(9) - Val1(6) + Val1(9)) - (Val1(7) + Val1(9)) - Val1(2) + Val1(8)) - Val1(8) - Val1(6) + Val1(9) ) +
( (Val1(2) - (Val1(6) + Val1(7) - Val1(3) + Val1(6) - Val1(7)) + Val1(4) - Val1(2) + (Val1(3) + Val1(8) + Val1(9)) - Val1(8)) - Val1(8) + Val1(8) + Val1(8) + (Val1(2) + Val1(8) - Val1(5) - Val1(9)) ) +
( (Val1(7) + Val1(5) + (Val1(7) + Val1(9) + Val1(5)) + Val1(2)) - Val1(4) + Val1(9) ) +
( Val1(3) - Val1(6) + Val1(8) + Val1(2) - Val1(7) ) +
( Val1(6) + Val1(2) + ((Val1(7) - Val1(6) + Val1(3) + Val1(4)) - Val1(8) - (Val1(2) + Val1(6) - Val1(5)) - Val1(6)) + Val1(4) - Val1(2) ) +
( (Val1(4) - Val1(8)) + Val1(7) - Val1(3) + Val1(3) - Val1(7) + Val1(5) ) +
( Val1(5) + Val1(6) + Val1(3) + Val1(7) - (Val1(2) - Val1(7)) + (Val1(5) + (Val1(3) + Val1(4) + Val1(3) + Val1(4) + Val1(7)) + Val1(8)) ) +
( Val1(2) + ((Val1(4) + Val1(7) - Val1(6)) + Val1(8) + Val1(9) - Val1(6) - Val1(9)) ) +
( (Val1(6) - Val1(2)) - (Val1(7) + Val1(5) - (Val1(7) - Val1(6) + Val1(7) + Val1(3) + Val1(8) + Val1(7)) + Val1(2) + Val1(5) + Val1(8)) + Val1(6) ) +
( (Val1(3) - Val1(2) - Val1(8) + Val1(7)) + (Val1(5) - Val1(4) - Val1(9) + Val1(6)) + Val1(4) - Val1(3) + Val1(4) ) +
( Val1(8) + (Val1(8) + Val1(2) - Val1(3) + Val1(2) + Val1(2)) ) +
( (Val1(7) + Val1(8) - Val1(5)) - (Val1(3) - (Val1(9) + Val1(5) + Val1(6) + Val1(9) + Val1(3) + Val1(9)) - Val1(3)) + Val1(8) + Val1(7) - (Val1(5) + Val1(9) + Val1(3) - Val1(2) + Val1(4) + Val1(5)) + Val1(4) ) +
( Val1(7) - ((Val1(6) - Val1(7) - Val1(2) - Val1(9) + Val1(8)) + Val1(8) - Val1(6) - Val1(8) - Val1(5)) - Val1(2) + Val1(3) + Val1(8) - Val1(3) ) +
( (Val1(4) + (Val1(5) - Val1(7) + Val1(3) + Val1(4) + Val1(9))) - Val1(7) ) +
( Val1(7) - Val1(5) - Val1(2) - (Val1(4) + Val1(4) + Val1(6) + Val1(3) - Val1(7)) + (Val1(8) - (Val1(6) + Val1(2) + Val1(8) - Val1(9) + Val1(6)) - Val1(3) - Val1(6) + Val1(3) + (Val1(4) + Val1(5) + Val1(9))) - Val1(7) ) +
( Val1(3) - Val1(4) + Val1(2) ) +
( ((Val1(6) + Val1(9) + Val1(5) + Val1(4) + Val1(9) - Val1(7)) - Val1(9) + Val1(8) + (Val1(9) - Val1(2) - Val1(6) - Val1(4) - Val1(7) - Val1(3))) + (Val1(8) + Val1(6) + Val1(7) + Val1(4)) - Val1(9) - (Val1(8) - Val1(5)) + Val1(7) ) +
( (Val1(9) - Val1(8) + Val1(4)) + (Val1(3) - Val1(4) + (Val1(9) - Val1(5) - Val1(3) - Val1(4)) - Val1(9)) ) +
( (Val1(7) + Val1(8) - Val1(3) + Val1(6) - (Val1(4) + Val1(5) + Val1(7) - Val1(8) + Val1(9)) + Val1(4)) - Val1(3) ) +
( ((Val1(7) - Val1(4)) + (Val1(6) - Val1(7) - Val1(6)) - Val1(7) + Val1(9) - Val1(3)) + Val1(2) - (Val1(4) + Val1(8)) + (Val1(4) - Val1(5) + Val1(8) + Val1(8) + Val1(5)) - (Val1(4) + Val1(9)) - Val1(5) ) +
( (Val1(2) + Val1(3) + Val1(7)) - Val1(6) + Val1(8) + (Val1(4) + Val1(7) - Val1(6) - (Val1(3) + Val1(4) + Val1(4) + Val1(7) - Val1(6)) - Val1(8) + Val1(3)) - Val1(4) ) +
( Val1(6) - (Val1(8) - Val1(3) + Val1(4) - (Val1(6) + Val1(9) - Val1(2) + Val1(3) + Val1(8) - Val1(5)) - Val1(8) + Val1(6)) ) +
( Val1(8) + ((Val1(2) - Val1(8) + Val1(5) - Val1(5) - Val1(5) - Val1(3)) - Val1(9) + Val1(5)) + Val1(9) ) +
( (Val1(8) + Val1(4) - Val1(9) - Val1(8)) + Val1(5) - Val1(5) - (Val1(7) + Val1(7) + Val1(2) - Val1(9) + Val1(8) - Val1(6)) + Val1(6) - (Val1(6) - Val1(6) + Val1(6) + Val1(3) + Val1(9) - Val1(8)) ) +
( Val1(5) - Val1(9) + (Val1(5) + (Val1(9) - Val1(5) + Val1(7)) + (Val1(2) + Val1(2) - Val1(7)) - Val1(4)) - (Val1(9) - Val1(9) + Val1(5) - Val1(4)) + Val1(9) ) +
( Val1(7) + Val1(7) - (Val1(2) - Val1(9) - Val1(6) + Val1(3) + Val1(5)) ) +
( Val1(2) + (Val1(8) - Val1(9) - Val1(7) - (Val1(4) - Val1(9) + Val1(9) - Val1(4) + Val1(5)) + Val1(4) - Val1(4)) + Val1(5) + Val1(2) + Val1(8) ) +
( (Val1(2) + (Val1(4) + Val1(9) + Val1(4) - Val1(7) - Val1(3)) + Val1(4) + Val1(8)) + (Val1(2) + Val1(4) - Val1(5) - (Val1(3) + Val1(4)) + Val1(8) - Val1(5)) ) +
( Val1(2) + ((Val1(7) - Val1(8) + Val1(6) - Val1(6)) - Val1(4) + Val1(6) + Val1(7) - Val1(6) - Val1(9)) + ((Val1(7) + Val1(9) - Val1(5) - Val1(5)) + Val1(4) - Val1(9) + Val1(6) + Val1(7)) - ((Val1(6) + Val1(9) + Val1(4) - Val1(7) - Val1(2)) - Val1(7) + Val1(9) - (Val1(7) + Val1(2) + Val1(3) - Val1(3) + Val1(5)) - Val1(5)) ) +
( (Val1(3) - Val1(8) + Val1(2) - Val1(2) + (Val1(3) - Val1(7) + Val1(8) - Val1(5) - Val1(6)) + Val1(6)) + Val1(4) + Val1(5) ) +
( Val1(8) + Val1(5) - Val1(8) + Val1(7) - Val1(2) ) +
( (Val1(3) + (Val1(6) - Val1(9) - Val1(8) + Val1(4) + Val1(8)) - Val1(4) + Val1(2)) + Val1(8) ) +
( (Val1(7) + Val1(7)) + Val1(5) - (Val1(9) - Val1(2) + Val1(9)) + ((Val1(4) - Val1(3) - Val1(3) - Val1(4) + Val1(4) - Val1(9)) - Val1(8) - Val1(8) + (Val1(3) - Val1(9) + Val1(7) + Val1(8)) - Val1(3) + Val1(9)) ) +
( (Val1(6) + (Val1(5) + Val1(7) - Val1(5)) - Val1(8) - Val1(9)) - Val1(6) + Val1(3) ) +
( Val1(3) + Val1(2) - (Val1(5) - Val1(8)) + Val1(5) + Val1(6) ) +
( Val1(7) - Val1(4) + Val1(7) + Val1(7) - Val1(4) - (Val1(4) + Val1(9) - Val1(6) - Val1(5)) ) +
( Val1(3) + Val1(4) + (Val1(6) - Val1(9)) - (Val1(6) + (Val1(5) - Val1(5) + Val1(6) - Val1(8)) + Val1(9) - (Val1(7) - Val1(3) + Val1(6))) + Val1(8) ) +
( Val1(4) + Val1(4) - Val1(6) - ((Val1(4) - Val1(3) - Val1(2)) - Val1(4) + Val1(3) + (Val1(7) + Val1(8) + Val1(7) - Val1(6) - Val1(9) - Val1(3))) - (Val1(5) + Val1(8)) ) +
( (Val1(6) + Val1(7) - Val1(8) + Val1(3) + Val1(9)) - Val1(5) - Val1(3) - Val1(3) ) +
( Val1(8) + Val1(5) + Val1(3) - Val1(8) - Val1(2) + Val1(9) ) +
( (Val1(9) - (Val1(5) + Val1(8) + Val1(4) + Val1(4) + Val1(9) + Val1(4)) - Val1(2) + Val1(7) - (Val1(6) - Val1(4) + Val1(7) + Val1(4) + Val1(8))) + Val1(9) + Val1(5) ) +
( Val1(7) + Val1(7) - Val1(3) + Val1(8) - ((Val1(5) + Val1(3) - Val1(4) - Val1(9)) - Val1(4) - Val1(6) - Val1(8)) ) +
( Val1(2) + Val1(5) + Val1(8) + (Val1(3) + Val1(3) - Val1(4) + Val1(8)) ) +
( Val1(5) + Val1(4) - Val1(3) - Val1(9) + ((Val1(5) - Val1(2) - Val1(2)) - Val1(5) + Val1(4) + (Val1(4) - Val1(2))) ) +
( (Val1(3) - Val1(7) + Val1(2)) - Val1(3) ) +
( Val1(8) + Val1(7) - Val1(3) + Val1(9) + Val1(9) - Val1(5) ) +
( Val1(2) + (Val1(2) - Val1(6) + Val1(3) + (Val1(4) + Val1(3) + Val1(4) + Val1(2) + Val1(7) - Val1(9))) + Val1(8) + Val1(2) - Val1(7) - Val1(9) ) +
( (Val1(4) - Val1(4) + Val1(4) + Val1(7)) - Val1(8) + Val1(5) ) +
( Val1(2) - (Val1(5) - (Val1(8) - Val1(5) - Val1(2) - Val1(6) - Val1(8)) - Val1(6) - Val1(3) + Val1(8) - Val1(8)) ) +
( Val1(8) + Val1(3) - Val1(9) + Val1(5) - Val1(9) - (Val1(5) - Val1(7) + Val1(6) + Val1(5) - Val1(3) + Val1(7)) ) +
( Val1(2) + (Val1(3) + Val1(7)) - Val1(8) - Val1(9) ) +
( Val1(7) + Val1(9) + Val1(6) ) +
( Val1(4) + (Val1(6) + Val1(7) + Val1(8) + Val1(4) - Val1(5)) + Val1(2) - Val1(6) - Val1(8) - Val1(2) ) +
( (Val1(9) + Val1(3) - Val1(7) - Val1(3)) - Val1(8) + (Val1(9) + Val1(5) - Val1(2) - Val1(8) + Val1(3) - Val1(4)) - Val1(8) + Val1(2) - Val1(6) ) +
( Val1(4) + Val1(2) - Val1(3) - Val1(6) - ((Val1(3) - Val1(5) - Val1(3)) - Val1(2) + Val1(6) + Val1(5) - Val1(4)) ) +
( Val1(3) + Val1(9) - (Val1(7) - (Val1(8) + Val1(7)) - Val1(9) - Val1(7) + Val1(7)) ) +
( Val1(6) + Val1(3) - Val1(6) + Val1(4) - (Val1(7) - Val1(7) + Val1(2) - Val1(9) + Val1(4) + Val1(2)) + (Val1(6) + Val1(6) + Val1(6) + Val1(7)) ) +
( (Val1(5) - Val1(4)) - Val1(6) - (Val1(5) + Val1(4) + Val1(5) + Val1(3) - (Val1(8) - Val1(6)) - Val1(5)) - Val1(7) ) +
( (Val1(8) - Val1(4)) + Val1(2) - (Val1(2) - Val1(2) - (Val1(9) - Val1(9) + Val1(8)) + Val1(8) - Val1(9) + Val1(6)) - Val1(7) ) +
( Val1(3) + Val1(5) - (Val1(4) + Val1(9) + Val1(3)) + Val1(9) - ((Val1(5) - Val1(2) - Val1(4) + Val1(2)) + Val1(6) + Val1(7) + Val1(7) + Val1(5)) ) +
( (Val1(3) + (Val1(9) + Val1(7) - Val1(3) + Val1(6) - Val1(6))) + Val1(9) - Val1(7) - Val1(4) + (Val1(9) - Val1(4) - Val1(9) + Val1(7) - Val1(7) - Val1(7)) ) +
( (Val1(3) - Val1(7)) + Val1(4) ) +
( Val1(4) + (Val1(2) + Val1(5) - Val1(5)) + Val1(6) + Val1(2) - Val1(8) + Val1(9) ) +
( Val1(8) - (Val1(6) - (Val1(7) - Val1(3)) + Val1(8) + Val1(5) + Val1(8) + (Val1(8) - Val1(2) + Val1(6) + Val1(9) + Val1(4) - Val1(3))) ) +
( Val1(7) - Val1(8) - (Val1(5) - (Val1(2) - Val1(6) - Val1(9)) - Val1(3) - Val1(6) - (Val1(6) + Val1(8) + Val1(5))) ) +
( (Val1(5) + Val1(9) - Val1(4) - Val1(7) - Val1(7) + Val1(8)) + Val1(4) ) +
( ((Val1(8) - Val1(9) - Val1(6) + Val1(4) - Val1(6) + Val1(3)) - Val1(6) + Val1(2) - (Val1(4) + Val1(8)) - Val1(7)) + (Val1(4) + Val1(4)) + (Val1(7) + Val1(7) - Val1(7) - (Val1(4) + Val1(5) + Val1(2))) ) +
( (Val1(4) + (Val1(8) + Val1(4) - Val1(4) - Val1(2) + Val1(4)) + Val1(7) + Val1(6) - Val1(2) + Val1(6)) - Val1(9) + Val1(3) - Val1(3) - (Val1(9) - Val1(9)) ) +
( (Val1(9) - Val1(5)) - Val1(6) + ((Val1(5) + Val1(3)) + Val1(9) + Val1(4)) ) +
( Val1(7) + ((Val1(8) - Val1(9) - Val1(5) + Val1(2) + Val1(2) - Val1(7)) - Val1(9) - Val1(3) - Val1(7) - Val1(2)) - Val1(6) - Val1(2) + Val1(6) - (Val1(9) - Val1(4) - Val1(2) + (Val1(2) + Val1(7) + Val1(5)) - Val1(4) + Val1(7)) ) +
( Val1(8) + (Val1(5) - Val1(4) - Val1(7) + Val1(2)) - Val1(6) + (Val1(6) - Val1(3)) - Val1(4) ) +
( Val1(7) + (Val1(9) + Val1(9) - Val1(4) + Val1(3)) - Val1(8) - (Val1(9) + (Val1(6) + Val1(9) + Val1(2) + Val1(6) + Val1(8)) - Val1(9) - Val1(7) - (Val1(2) - Val1(8) - Val1(5) - Val1(8))) - (Val1(4) + Val1(7) + Val1(8) + Val1(2)) ) +
( Val1(6) + Val1(3) - Val1(5) - Val1(8) + ((Val1(9) + Val1(5) + Val1(6)) - Val1(9) - (Val1(3) + Val1(7) - Val1(8) + Val1(6)) - (Val1(2) - Val1(9) + Val1(5))) + (Val1(7) + Val1(6)) ) +
( Val1(5) - (Val1(9) + Val1(7)) + Val1(2) - Val1(4) + Val1(3) ) +
( Val1(7) + Val1(2) - (Val1(5) + Val1(6) + Val1(9) + Val1(4) - (Val1(7) + Val1(6) - Val1(5) - Val1(6) + Val1(8)) - Val1(4)) - Val1(6) + Val1(4) ) +
( Val1(6) - Val1(8) - Val1(8) - Val1(4) + Val1(8) - (Val1(2) - (Val1(9) - Val1(4) - Val1(7) + Val1(9)) - Val1(5)) ) +
( Val1(8) - Val1(4) + (Val1(4) + Val1(7) - Val1(3) + Val1(5) + Val1(8)) + Val1(4) - Val1(7) + ((Val1(4) - Val1(9) + Val1(3) - Val1(8)) - Val1(5) - Val1(6) + Val1(7) + Val1(5) - Val1(4)) ) +
( (Val1(9) - Val1(3) + Val1(3)) + Val1(9) + ((Val1(9) + Val1(9) + Val1(6) + Val1(8)) - Val1(9) + Val1(7)) - Val1(4) + Val1(9) - ((Val1(8) + Val1(7) + Val1(8) - Val1(2) + Val1(8)) - Val1(5) + Val1(8) + Val1(3) - (Val1(3) + Val1(5) + Val1(2) - Val1(6)) - Val1(8)) ) +
( (Val1(8) - Val1(4) - Val1(3)) + (Val1(4) - Val1(3) - Val1(8) + (Val1(5) - Val1(9)) - Val1(3) - Val1(9)) + Val1(9) + (Val1(4) - Val1(2)) + Val1(9) + Val1(4) ) +
( Val1(2) - Val1(5) + (Val1(9) + (Val1(5) + Val1(2) - Val1(2) - Val1(7) - Val1(3) - Val1(9))) ) +
( (Val1(4) - Val1(2) + Val1(8)) + (Val1(4) + Val1(5) + Val1(3) + Val1(9) + (Val1(9) - Val1(2) - Val1(4)) - (Val1(2) - Val1(6) + Val1(6))) ) +
( Val1(9) - Val1(5) + Val1(2) - ((Val1(3) - Val1(3) + Val1(3) + Val1(9) - Val1(7) + Val1(9)) - Val1(3) - Val1(3) + Val1(6)) + Val1(7) ) +
( Val1(8) + (Val1(5) + Val1(4) + Val1(9)) + ((Val1(6) + Val1(9) - Val1(6)) - Val1(8) + Val1(3) + Val1(6) - Val1(8) - Val1(6)) - ((Val1(6) + Val1(4)) - (Val1(3) + Val1(5) - Val1(7) - Val1(9) - Val1(4) + Val1(3)) + (Val1(2) - Val1(4))) + Val1(4) ) +
( Val1(5) + Val1(7) + (Val1(5) + Val1(2) + (Val1(5) + Val1(6) - Val1(2)) - Val1(7)) + Val1(4) - Val1(4) ) +
( (Val1(7) - Val1(8) - Val1(8)) - (Val1(3) - (Val1(2) - Val1(3) - Val1(4) + Val1(3) + Val1(5) + Val1(6)) + Val1(2) + Val1(9) + (Val1(7) + Val1(5) + Val1(7) - Val1(9)) + Val1(9)) + Val1(2) ) +
( Val1(2) - Val1(8) + ((Val1(4) + Val1(9) + Val1(6) + Val1(8) - Val1(2) - Val1(9)) - Val1(9) - (Val1(5) - Val1(7) + Val1(7) - Val1(4) + Val1(3) + Val1(9)) - Val1(3) + Val1(2) - Val1(8)) - (Val1(3) + Val1(7) - Val1(7) - Val1(5) - Val1(3)) - Val1(9) - Val1(8) ) +
( Val1(6) - Val1(8) - (Val1(5) - Val1(7) + Val1(3) - Val1(2) - Val1(3) + Val1(4)) + Val1(7) + (Val1(4) + Val1(2) - Val1(8) - Val1(8)) + ((Val1(9) - Val1(4) + Val1(5)) - (Val1(7) + Val1(6) + Val1(2) - Val1(4))) ) +
( Val1(4) + Val1(9) + (Val1(9) + Val1(5) + Val1(3) - Val1(8)) - Val1(5) - Val1(9) ) +
( Val1(5) + Val1(2) + ((Val1(6) + Val1(8) + Val1(3) - Val1(7)) - Val1(6) - Val1(5) + Val1(8) + Val1(3) - Val1(9)) + Val1(6) ) +
( Val1(3) + (Val1(7) + (Val1(3) - Val1(6) - Val1(7)) + Val1(9) - Val1(2) - Val1(2) + (Val1(9) - Val1(4))) - Val1(8) - Val1(8) - Val1(6) ) +
( ((Val1(5) + Val1(9) + Val1(8) + Val1(6)) + Val1(4) + (Val1(3) - Val1(4) + Val1(8)) - (Val1(2) + Val1(5) - Val1(2) - Val1(6))) + Val1(9) + Val1(8) - Val1(2) ) +
( Val1(8) - Val1(9) + Val1(2) ) +
( (Val1(3) - (Val1(3) - Val1(5) - Val1(2) - Val1(3) - Val1(3)) - Val1(3) - Val1(7)) - Val1(3) - Val1(6) - Val1(5) + Val1(8) ) +
( Val1(6) - Val1(8) + Val1(6) + Val1(9) - Val1(9) - (Val1(7) - Val1(2) - Val1(5) - Val1(3) - Val1(4)) ) +
( (Val1(8) + Val1(2) + Val1(4) - Val1(3) + (Val1(4) - Val1(6) + Val1(7) - Val1(2))) + (Val1(6) + Val1(8)) + Val1(8) ) +
( Val1(9) + Val1(4) ) +
( ((Val1(4) + Val1(8)) + Val1(8) - (Val1(4) - Val1(5) + Val1(8) + Val1(2) + Val1(8) - Val1(9)) + Val1(7) - (Val1(3) - Val1(7) - Val1(4) - Val1(8)) + Val1(4)) - Val1(6) - Val1(7) - (Val1(2) - Val1(3) + Val1(6)) ) +
( Val1(6) + (Val1(4) + Val1(5) - Val1(9)) - Val1(6) + Val1(9) ) +
( (Val1(4) - (Val1(5) - Val1(3) - Val1(3) - Val1(7) - Val1(2) + Val1(7)) + Val1(4)) + Val1(4) - Val1(8) - Val1(4) - Val1(3) ) +
( Val1(8) + (Val1(9) + Val1(8) + Val1(9)) - Val1(5) ) +
( Val1(9) + Val1(8) + Val1(8) + Val1(9) - (Val1(8) - Val1(3)) ) +
( (Val1(6) + Val1(8) - Val1(5) - (Val1(5) + Val1(7) - Val1(8) + Val1(4) + Val1(3)) - Val1(6) + Val1(4)) - (Val1(2) - Val1(4) - Val1(5)) + Val1(7) - Val1(8) + (Val1(9) - Val1(6)) ) +
( ((Val1(9) + Val1(2) + Val1(5) + Val1(5) + Val1(5)) + (Val1(9) + Val1(6) + Val1(7)) - (Val1(7) + Val1(5) + Val1(9) + Val1(5) - Val1(6))) - (Val1(4) + (Val1(7) + Val1(8) + Val1(9) + Val1(7)) - (Val1(7) + Val1(7) + Val1(6) - Val1(4) + Val1(2)) + Val1(5) + Val1(5) + Val1(6)) - Val1(3) ) +
( Val1(7) - (Val1(6) + Val1(8) - Val1(6) - (Val1(5) + Val1(6) + Val1(5) - Val1(4)) - (Val1(9) - Val1(2) - Val1(6) - Val1(4) + Val1(8))) - Val1(2) + Val1(9) ) +
( (Val1(7) - Val1(3) - Val1(3) - Val1(9) - Val1(4)) + Val1(9) - (Val1(9) - (Val1(8) - Val1(8) + Val1(2) - Val1(8) + Val1(3) + Val1(6)) - Val1(5) + (Val1(4) + Val1(3) - Val1(3) - Val1(4) + Val1(5) - Val1(8))) + Val1(9) - Val1(4) ) +
( (Val1(7) + Val1(3) + Val1(4)) - (Val1(2) + (Val1(4) - Val1(7) + Val1(5) - Val1(3) - Val1(7)) - Val1(4) - (Val1(3) + Val1(5)) + Val1(8) - (Val1(8) - Val1(9) + Val1(2) + Val1(7))) ) +
( Val1(7) + (Val1(8) - Val1(7) - Val1(2) + (Val1(7) + Val1(6) + Val1(8)) - Val1(3) - Val1(3)) + (Val1(4) - (Val1(7) - Val1(7) + Val1(4) - Val1(7) - Val1(4)) + Val1(7) - Val1(5) + (Val1(6) - Val1(6) + Val1(4))) ) +
( Val1(5) + Val1(3) + Val1(4) + (Val1(2) + Val1(4) - Val1(5) - Val1(2) + Val1(8)) ) +
( (Val1(3) + Val1(3) + Val1(2) + Val1(9) - Val1(4)) + Val1(7) + (Val1(2) - Val1(8) - Val1(7) - Val1(3) + (Val1(3) + Val1(9) + Val1(9) - Val1(2))) + Val1(7) - (Val1(9) - Val1(7)) + Val1(2) ) +
( Val1(7) - Val1(5) - (Val1(4) - Val1(8) - Val1(4) - (Val1(7) + Val1(6) - Val1(5) - Val1(5) + Val1(7) - Val1(9))) + Val1(9) ) +
( (Val1(2) - Val1(5) + Val1(2) + (Val1(6) + Val1(3) + Val1(4))) + Val1(2) + ((Val1(6) + Val1(8) + Val1(9)) - Val1(4)) + Val1(2) ) +
( Val1(8) - (Val1(2) - Val1(9) + Val1(3) + Val1(2)) + Val1(9) ) +
( (Val1(5) + (Val1(5) + Val1(9) - Val1(5) + Val1(5) - Val1(4) - Val1(9)) - Val1(7)) + (Val1(8) - Val1(8) - Val1(9) + Val1(8) - Val1(3)) + Val1(3) + Val1(6) - Val1(5) ) +
( Val1(8) + (Val1(7) - (Val1(7) + Val1(9)) + Val1(3)) ) +
( (Val1(7) + Val1(4) - Val1(5) + (Val1(8) + Val1(2) + Val1(7)) - Val1(7) + Val1(8)) + (Val1(9) - (Val1(9) - Val1(8) + Val1(7) + Val1(9) - Val1(4))) - (Val1(4) + Val1(9) - Val1(5)) + Val1(2) ) +
( (Val1(7) - Val1(5) + Val1(6) - Val1(5) - (Val1(2) - Val1(4) - Val1(3) + Val1(4) + Val1(7))) - Val1(5) - (Val1(7) - (Val1(7) + Val1(3) - Val1(4)) - Val1(9)) ) +
( Val1(4) - (Val1(6) + Val1(9) - Val1(3)) + Val1(9) + Val1(5) - Val1(9) ) +
( (Val1(9) + Val1(3) + Val1(8) - Val1(7) - Val1(2) - Val1(6)) - (Val1(8) + Val1(7) + Val1(2) + Val1(7) + Val1(6)) + (Val1(5) - Val1(6) - Val1(6) + Val1(3)) + Val1(2) + Val1(9) ) +
( Val1(5) - Val1(5) + Val1(6) + (Val1(3) - Val1(7) + Val1(6)) ) +
( Val1(2) - Val1(7) + (Val1(5) - (Val1(5) + Val1(8) + Val1(9) - Val1(5) + Val1(3)) + (Val1(3) - Val1(2)) - Val1(3) + Val1(7) - Val1(2)) + Val1(2) + Val1(5) + Val1(3) ) +
( (Val1(9) - Val1(5) - (Val1(4) - Val1(9) - Val1(6)) + Val1(3) - Val1(8)) - Val1(2) - Val1(8) - Val1(4) - ((Val1(3) - Val1(7) - Val1(9) + Val1(5)) + Val1(6) - Val1(7)) ) +
( Val1(5) - Val1(9) + ((Val1(9) + Val1(4) + Val1(5) - Val1(7)) + Val1(4) - Val1(3) + Val1(6) - (Val1(2) - Val1(2))) + Val1(7) + Val1(2) ) +
( Val1(2) - (Val1(6) + Val1(2)) - ((Val1(6) + Val1(3) - Val1(8) - Val1(3) + Val1(7)) - Val1(7) - Val1(5) + (Val1(9) + Val1(2) + Val1(3) - Val1(9) - Val1(8) - Val1(8)) - Val1(7)) + (Val1(9) - Val1(9) + Val1(8) - Val1(7) + (Val1(9) - Val1(3) - Val1(8)) + Val1(9)) ) +
( Val1(4) + Val1(2) - Val1(2) - Val1(5) + Val1(5) - Val1(4) ) +
( Val1(4) + Val1(4) - Val1(8) - Val1(7) + Val1(3) ) +
( Val1(9) + (Val1(3) + Val1(8) - (Val1(4) - Val1(2) + Val1(3) + Val1(4)) + Val1(5) + Val1(8)) - Val1(8) + Val1(4) - Val1(7) - (Val1(4) + Val1(3) - Val1(9) - Val1(4)) ) +
( Val1(6) + (Val1(3) + Val1(2) - Val1(2) + Val1(3) + Val1(7) - Val1(8)) - Val1(4) - Val1(8) + Val1(2) ) +
( (Val1(7) - Val1(4) + Val1(5) - Val1(2) - (Val1(5) - Val1(8) + Val1(3) + Val1(6) - Val1(2))) + (Val1(7) - Val1(2)) - Val1(9) - (Val1(7) + Val1(7) + Val1(5)) - (Val1(8) + Val1(2) - Val1(9)) - Val1(6) ) +
( Val1(3) + Val1(9) - (Val1(7) + Val1(2) - Val1(6) + Val1(2) + Val1(5)) - Val1(4) ) +
( Val1(9) + (Val1(8) - Val1(2) + (Val1(8) + Val1(8) + Val1(8))) - Val1(8) + Val1(7) ) +
( Val1(6) - Val1(7) - Val1(7) - Val1(4) ) +
( Val1(2) - (Val1(2) + (Val1(4) + Val1(7) - Val1(7) + Val1(6) - Val1(8))) + Val1(4) ) +
( (Val1(9) + Val1(8) + (Val1(5) + Val1(9) + Val1(4) + Val1(9) - Val1(6) + Val1(3))) - Val1(4) ) +
( Val1(6) + ((Val1(9) + Val1(2) + Val1(4) - Val1(2) + Val1(9)) - Val1(7) - Val1(3) + Val1(5) - Val1(6) - Val1(6)) + Val1(7) - Val1(4) - Val1(9) ) +
( (Val1(5) + (Val1(5) + Val1(9) + Val1(6) + Val1(7))) - Val1(5) + Val1(4) + Val1(6) ) +
( (Val1(5) - Val1(8) - Val1(6) + Val1(8) + Val1(7)) - Val1(2) + Val1(7) + Val1(8) + Val1(5) ) +
( (Val1(7) - Val1(5) + Val1(6) + Val1(5) - (Val1(6) + Val1(4) - Val1(9))) + Val1(4) - Val1(8) + Val1(3) ) +
( Val1(4) - Val1(9) - (Val1(8) - Val1(5) - Val1(3) - Val1(9) - (Val1(4) - Val1(2) - Val1(3) - Val1(4)) - Val1(3)) - ((Val1(5) + Val1(5) - Val1(3)) + Val1(9) - Val1(4) + (Val1(9) + Val1(6)) - Val1(2) - Val1(6)) ) +
( Val1(7) - ((Val1(3) + Val1(8) - Val1(3) - Val1(6) - Val1(8) - Val1(6)) + (Val1(8) - Val1(3) - Val1(7) + Val1(4) - Val1(2) - Val1(5)) - Val1(5) - Val1(8) - (Val1(7) + Val1(9) - Val1(4) + Val1(6) - Val1(2) - Val1(8)) - Val1(4)) + Val1(7) + Val1(7) ) +
( (Val1(7) - (Val1(7) + Val1(4) + Val1(4) - Val1(5) - Val1(5) + Val1(2)) - Val1(2) + Val1(5) - Val1(6)) + Val1(2) - Val1(5) + Val1(3) ) +
( (Val1(2) + Val1(7)) - Val1(6) - (Val1(3) + (Val1(9) - Val1(4) + Val1(8) - Val1(8) + Val1(5)) + (Val1(2) + Val1(7) - Val1(5) - Val1(5)) - Val1(5) + (Val1(3) - Val1(6)) - Val1(2)) + (Val1(8) + Val1(5) - Val1(9) + Val1(9) + Val1(8)) ) +
( Val1(6) - Val1(5) - Val1(3) + Val1(7) - (Val1(4) + Val1(2) + Val1(7) + Val1(9)) ) +
( (Val1(9) + Val1(6) + (Val1(2) + Val1(2) + Val1(5) + Val1(6) - Val1(5) - Val1(8))) + Val1(7) - Val1(2) ) +
( ((Val1(7) + Val1(7) - Val1(9)) + (Val1(5) + Val1(8) - Val1(2) + Val1(4) - Val1(7) - Val1(7)) + Val1(7) - Val1(3)) - (Val1(8) + Val1(2)) + Val1(7) ) +
( Val1(3) + Val1(6) - (Val1(8) - Val1(7) - Val1(3) - Val1(7)) + (Val1(5) + Val1(4) - Val1(3)) + Val1(6) ) +
( (Val1(7) + Val1(7)) + Val1(9) + (Val1(5) - (Val1(5) + Val1(9) - Val1(3) + Val1(3) - Val1(2) + Val1(3)) - Val1(4) - (Val1(8) - Val1(7)) - Val1(2) - (Val1(7) - Val1(9) - Val1(6) + Val1(4) - Val1(6) + Val1(2))) + Val1(6) ) +
( ((Val1(6) - Val1(2) - Val1(6) + Val1(3) - Val1(8)) + (Val1(9) + Val1(8) + Val1(9) + Val1(2) + Val1(3) + Val1(5)) - Val1(7) - Val1(9)) + Val1(8) + Val1(6) - Val1(3) + Val1(6) + Val1(7) ) +
( ((Val1(9) - Val1(8) + Val1(2)) - Val1(8) + Val1(6) - (Val1(4) + Val1(8) + Val1(4) - Val1(9) - Val1(9)) + Val1(9)) - Val1(7) - Val1(5) ) +
( Val1(8) + Val1(6) + (Val1(4) + (Val1(6) + Val1(6) + Val1(3) + Val1(9))) + Val1(3) - Val1(3) + Val1(4) ) +
( Val1(8) + (Val1(3) + Val1(5) - Val1(9) + Val1(3)) - Val1(9) + Val1(8) + Val1(8) + Val1(7) ) +
( Val1(5) + (Val1(3) - Val1(4) + (Val1(8) + Val1(4) + Val1(3) + Val1(4)) - Val1(8) + Val1(7)) ) +
( (Val1(9) - (Val1(6) - Val1(7) - Val1(9) + Val1(5)) - (Val1(7) + Val1(6) - Val1(2) - Val1(4) + Val1(2) + Val1(8))) + Val1(4) + Val1(8) ) +
( Val1(3) - Val1(5) - (Val1(9) + Val1(2) - (Val1(9) - Val1(6) - Val1(7) + Val1(9)) + Val1(2)) - Val1(7) + Val1(6) ) +
( Val1(4) + (Val1(3) - Val1(7)) ) +
( (Val1(3) + Val1(7) - Val1(9)) - Val1(6) - Val1(6) + Val1(3) + Val1(3) + (Val1(2) + (Val1(9) - Val1(4) + Val1(6) + Val1(7) + Val1(9))) ) +
( (Val1(4) - Val1(2) - (Val1(9) + Val1(5) - Val1(9) + Val1(5) - Val1(9) + Val1(9)) + Val1(8)) - Val1(7) - Val1(7) - Val1(3) + (Val1(9) - Val1(4) - Val1(7)) ) +
( Val1(3) - (Val1(4) - Val1(5)) + Val1(9) + ((Val1(7) - Val1(8) - Val1(4) - Val1(8)) + Val1(7) - Val1(3)) ) +
( Val1(8) + Val1(9) + Val1(4) - Val1(4) - ((Val1(5) + Val1(3) - Val1(6)) - Val1(5)) - Val1(6) ) +
( Val1(9) + Val1(8) + (Val1(9) - Val1(9) + Val1(4)) + Val1(5) ) +
( Val1(9) - ((Val1(8) - Val1(7) - Val1(2) + Val1(4) + Val1(8)) - Val1(9) + Val1(5) + (Val1(2) + Val1(4) + Val1(6) - Val1(7) - Val1(3) + Val1(3)) + (Val1(7) - Val1(9) - Val1(2) + Val1(7) - Val1(9)) - Val1(3)) + (Val1(6) - (Val1(9) + Val1(7) - Val1(3) + Val1(7) + Val1(6)) + Val1(6) + Val1(6)) ) +
( Val1(4) - ((Val1(5) - Val1(7) - Val1(9) - Val1(9) - Val1(4)) + Val1(2) + (Val1(5) - Val1(5)) - Val1(2) - Val1(7) - Val1(9)) - Val1(4) - Val1(7) + Val1(2) + Val1(9) ) +
( Val1(9) + (Val1(7) + Val1(5)) + Val1(4) + (Val1(3) + Val1(2) - Val1(4)) - Val1(8) + Val1(7) ) +
( (Val1(8) - Val1(7) + Val1(7) + Val1(6) + Val1(7) - Val1(6)) - Val1(5) + Val1(7) + Val1(5) ) +
( Val1(9) + Val1(5) - Val1(5) - (Val1(3) + Val1(8) - Val1(4) + Val1(8) + Val1(4) - Val1(5)) + Val1(3) + Val1(4) ) +
( Val1(4) - Val1(5) - Val1(6) + Val1(6) + (Val1(7) - Val1(7) + Val1(6) + Val1(4) + Val1(4) - Val1(3)) ) +
( (Val1(4) - Val1(2) + Val1(9)) + (Val1(4) - Val1(3) + Val1(9)) - Val1(9) + Val1(8) ) +
( Val1(9) - Val1(6) + ((Val1(7) + Val1(2) + Val1(6)) + (Val1(2) + Val1(2) - Val1(3))) - Val1(6) ) +
( ((Val1(5) + Val1(6) + Val1(6) - Val1(6)) + Val1(8) + (Val1(3) - Val1(8) - Val1(3) - Val1(3) - Val1(6)) - Val1(4)) - (Val1(2) + Val1(6) - Val1(7) - Val1(6) - (Val1(3) + Val1(5))) - Val1(6) - Val1(2) ) +
( (Val1(5) - Val1(3)) + (Val1(2) + Val1(9)) + Val1(2) ) +
( Val1(4) - Val1(8) + Val1(6) ) +
( Val1(2) + Val1(6) + Val1(2) + Val1(4) ) +
( Val1(5) - Val1(8) - (Val1(9) - Val1(2) - Val1(8)) + Val1(8) + Val1(9) + Val1(4) ) +
( Val1(5) - (Val1(4) + Val1(7) - (Val1(5) - Val1(7) - Val1(6) - Val1(5) - Val1(8)) + (Val1(6) + Val1(3) - Val1(6) + Val1(3) + Val1(7)) - (Val1(7) - Val1(5) - Val1(8)) - Val1(4)) ) +
( Val1(5) - Val1(9) + Val1(5) + Val1(5) - (Val1(9) - Val1(2) + Val1(4) + Val1(9) - Val1(7) + Val1(4)) - Val1(7) ) +
( Val1(5) + (Val1(4) + Val1(7) + (Val1(9) + Val1(7) + Val1(5)) + Val1(9) - Val1(6) - Val1(3)) - Val1(5) - Val1(3) + Val1(5) ) +
( (Val1(7) - Val1(8) - Val1(7)) + Val1(6) + Val1(2) - Val1(4) + Val1(6) + Val1(4) ) +
( (Val1(2) + (Val1(4) + Val1(4) + Val1(6)) + Val1(9) + Val1(5) - (Val1(2) - Val1(7) - Val1(3)) - Val1(2)) + Val1(7) + ((Val1(9) + Val1(5)) + (Val1(5) - Val1(3) + Val1(8) + Val1(5) - Val1(6) + Val1(8)) - Val1(2) + Val1(7)) - Val1(9) - Val1(3) ) +
( (Val1(2) - (Val1(9) - Val1(5)) + Val1(9) + Val1(4)) + Val1(8) - Val1(7) + Val1(8) + Val1(8) + Val1(8) ) +
( Val1(2) + Val1(3) - Val1(3) + (Val1(7) - (Val1(9) + Val1(5)) + (Val1(8) + Val1(6) + Val1(8) + Val1(2)) - Val1(7) - Val1(2) + (Val1(7) + Val1(8) + Val1(9) - Val1(3))) + Val1(5) + Val1(9) ) +
( Val1(5) + Val1(4) + (Val1(7) - Val1(3) - (Val1(2) - Val1(6) + Val1(2) - Val1(3)) + Val1(3)) - Val1(7) ) +
( Val1(9) + (Val1(3) + (Val1(7) - Val1(3)) + Val1(7) + Val1(3)) + Val1(5) + (Val1(7) - Val1(3) + Val1(7)) ) +
( Val1(7) + Val1(4) - Val1(2) + Val1(4) + (Val1(8) + Val1(3) + Val1(7)) - Val1(5) ) +
( Val1(9) - Val1(4) + (Val1(6) + Val1(9) + Val1(3)) ) +
( Val1(2) + Val1(3) + Val1(7) + Val1(7) + Val1(8) + ((Val1(4) - Val1(7) + Val1(9) + Val1(8) + Val1(6) - Val1(3)) - Val1(3) - Val1(7) - Val1(9) - Val1(6) - Val1(7)) ) +
( (Val1(7) - Val1(9) - Val1(4) - Val1(2)) + (Val1(3) + (Val1(5) + Val1(2) + Val1(4) + Val1(6) - Val1(8) - Val1(5)) + Val1(5) + Val1(5)) + Val1(4) ) +
( (Val1(6) - Val1(2) + Val1(8) + Val1(3)) - ((Val1(8) - Val1(3) - Val1(9) - Val1(2) - Val1(7)) - Val1(4) + Val1(9) + Val1(6)) - Val1(6) + Val1(8) ) +
( (Val1(3) - Val1(5) - Val1(8) - Val1(4) + (Val1(2) - Val1(5)) + Val1(2)) + Val1(5) - Val1(8) - Val1(5) - Val1(3) - Val1(4) ) +
( Val1(4) + (Val1(9) + Val1(7) - (Val1(6) + Val1(3) + Val1(9) + Val1(7))) ) +
( (Val1(9) - Val1(2) + Val1(3) + Val1(7)) - Val1(7) - (Val1(8) - Val1(2) - Val1(7)) - Val1(3) + Val1(4) ) +
( Val1(6) - (Val1(8) - Val1(2) - Val1(8) + Val1(8) + Val1(9) + (Val1(4) - Val1(7) - Val1(4) + Val1(9) + Val1(5) + Val1(5))) - Val1(4) ) +
( Val1(5) - (Val1(2) - Val1(4)) - Val1(6) ) +
( Val1(4) + Val1(4) - Val1(4) + Val1(6) + ((Val1(5) + Val1(3) + Val1(2) - Val1(2) + Val1(4)) - Val1(3) - Val1(2) - (Val1(7) + Val1(7) + Val1(8) + Val1(6) + Val1(3)) + Val1(2)) ) +
( Val1(7) - Val1(6) - Val1(8) + (Val1(9) - Val1(9) + Val1(3) + Val1(9) + Val1(3) - Val1(7)) + Val1(3) ) +
( Val1(8) - (Val1(2) - Val1(8) - (Val1(2) - Val1(4) + Val1(6) - Val1(4) - Val1(8))) - (Val1(9) - Val1(3) + Val1(8) - Val1(3) - Val1(8) - (Val1(6) + Val1(8) + Val1(6) - Val1(7) + Val1(3))) - Val1(6) - Val1(6) - Val1(7) ) +
( (Val1(3) + (Val1(7) - Val1(7) - Val1(4) + Val1(2) - Val1(4) + Val1(7))) + Val1(6) - Val1(5) + Val1(9) + Val1(4) ) +
( (Val1(2) + Val1(2) - Val1(3)) + (Val1(3) - Val1(3) + Val1(2) - Val1(3) - Val1(5) + Val1(9)) + Val1(8) - (Val1(8) + Val1(9) - (Val1(9) + Val1(9) + Val1(2) - Val1(9)) + Val1(9) + (Val1(6) + Val1(3) - Val1(7)) - Val1(5)) - Val1(4) ) +
( Val1(4) - Val1(8) - Val1(3) + Val1(5) - (Val1(6) - (Val1(5) - Val1(4) + Val1(9) + Val1(4) - Val1(6) - Val1(3))) + Val1(9) ) +
( Val1(4) - ((Val1(7) + Val1(4) - Val1(8)) - Val1(2)) - Val1(6) - Val1(6) + (Val1(7) - Val1(4)) - Val1(2) ) +
( Val1(7) + Val1(3) - Val1(3) + Val1(6) ) +
( Val1(2) - (Val1(3) + (Val1(6) + Val1(5)) + (Val1(3) - Val1(2) + Val1(6)) + Val1(4)) + Val1(6) - Val1(7) - (Val1(5) - Val1(2) + Val1(3) + Val1(9) - Val1(9)) ) +
( (Val1(7) - Val1(5) - (Val1(8) + Val1(4) + Val1(6) + Val1(3) - Val1(5))) + Val1(2) - Val1(9) ) +
( Val1(3) - Val1(4) - Val1(7) - (Val1(7) + (Val1(5) + Val1(7))) + Val1(9) ) +
( Val1(9) - ((Val1(9) + Val1(7)) + Val1(3)) ) +
( Val1(5) + ((Val1(6) - Val1(2) + Val1(8)) - Val1(7)) ) +
( Val1(7) + Val1(2) - (Val1(6) + Val1(6) + Val1(6) + Val1(3)) - Val1(2) ) +
( (Val1(9) + Val1(3) - (Val1(4) + Val1(8) + Val1(5) + Val1(5) + Val1(7) - Val1(9))) - ((Val1(7) - Val1(5) - Val1(9) - Val1(2) - Val1(6)) + Val1(5) + Val1(6) - Val1(3) + Val1(9)) - Val1(8) + Val1(7) - (Val1(5) + Val1(6)) ) +
( Val1(3) + ((Val1(8) + Val1(5) - Val1(9) - Val1(6) + Val1(6)) - Val1(2) - Val1(5) + Val1(9) - (Val1(8) - Val1(5) - Val1(8) + Val1(4) + Val1(9) - Val1(2)) - Val1(8)) - Val1(8) + Val1(7) ) +
( Val1(3) + Val1(2) - Val1(3) + ((Val1(9) + Val1(3)) - Val1(9)) ) +
( Val1(8) - (Val1(2) + (Val1(6) + Val1(2) - Val1(8))) - Val1(2) - Val1(7) + Val1(8) ) +
( (Val1(4) + Val1(9) - (Val1(8) + Val1(4) - Val1(2) + Val1(9) - Val1(6) + Val1(8)) - Val1(6) + (Val1(8) - Val1(9)) + Val1(3)) - Val1(6) ) +
( Val1(3) - (Val1(3) - Val1(9) - Val1(5) - Val1(6) + Val1(2) + Val1(4)) - Val1(9) - (Val1(8) - Val1(4) - Val1(7) - Val1(7) - Val1(8) - Val1(3)) - Val1(5) ) +
( Val1(8) - Val1(2) + (Val1(7) + Val1(4) + Val1(5) + Val1(2) - Val1(4) - Val1(8)) + (Val1(2) - Val1(4) + Val1(7) + Val1(4)) - Val1(4) ) +
( (Val1(7) + Val1(9) + Val1(9) - Val1(6)) - Val1(6) + Val1(9) - Val1(8) ) +
( Val1(5) + Val1(8) + Val1(3) - Val1(3) + (Val1(6) + (Val1(9) - Val1(8) + Val1(8) - Val1(5)) + (Val1(3) - Val1(5) - Val1(4))) ) +
( (Val1(2) - Val1(9) + Val1(3) - Val1(9) - Val1(8)) - (Val1(6) + Val1(6) - Val1(8)) - Val1(9) - Val1(4) ) +
( Val1(2) - (Val1(8) - Val1(9) + Val1(4)) ) +
( Val1(2) + Val1(5) - (Val1(5) - Val1(4) - Val1(9) - Val1(3) - Val1(8) - Val1(3)) - Val1(8) ) +
( Val1(9) - Val1(9) - (Val1(7) - Val1(3) - Val1(4)) - Val1(8) - Val1(8) ) +
( Val1(4) + (Val1(9) + Val1(8) + Val1(6) + Val1(8)) - Val1(9) + (Val1(6) + Val1(6) + Val1(9) - Val1(5)) + Val1(9) + (Val1(8) + Val1(4) - (Val1(5) + Val1(6) + Val1(3) - Val1(6) + Val1(6)) + Val1(5) - (Val1(8) - Val1(3) - Val1(8) - Val1(3) - Val1(7) - Val1(2))) ) +
( (Val1(5) - Val1(8) - Val1(7) - Val1(5) - Val1(2) + Val1(4)) - Val1(6) + Val1(8) - ((Val1(4) - Val1(5) - Val1(9) + Val1(3) + Val1(2)) - Val1(4) + Val1(4)) ) +
( Val1(2) - Val1(9) + (Val1(9) - Val1(7) - Val1(8) - Val1(6) - Val1(7) - Val1(9)) + Val1(4) + (Val1(7) + (Val1(3) + Val1(9) - Val1(5) + Val1(5) + Val1(2)) + (Val1(6) - Val1(3) + Val1(2) + Val1(7) - Val1(7))) - Val1(7) ) +
( Val1(7) - Val1(3) - Val1(7) + (Val1(2) - Val1(2) - Val1(4)) - Val1(5) ) +
( (Val1(2) + Val1(8) - Val1(6) - Val1(3) - (Val1(6) - Val1(5) + Val1(9) + Val1(3) - Val1(8) + Val1(8)) + (Val1(3) + Val1(6) + Val1(9) + Val1(4) - Val1(3) - Val1(6))) + Val1(4) + (Val1(3) - Val1(4) + Val1(2) + (Val1(5) - Val1(3) - Val1(7) + Val1(5) + Val1(9))) - ((Val1(2) + Val1(3) + Val1(6) + Val1(6) - Val1(4) - Val1(2)) + Val1(4) - Val1(9) + Val1(7) - (Val1(6) + Val1(2))) + (Val1(6) + (Val1(8) - Val1(7) + Val1(2) + Val1(2)) + Val1(8)) - (Val1(7) + (Val1(9) + Val1(3) + Val1(9))) ) +
( Val1(7) - Val1(4) - (Val1(7) + Val1(5) - Val1(3) + (Val1(9) - Val1(2) - Val1(5) - Val1(2)) + Val1(9) + Val1(2)) - (Val1(5) + Val1(4) - (Val1(8) + Val1(8) - Val1(6)) + Val1(2)) ) +
( Val1(7) - (Val1(9) - (Val1(7) + Val1(4) - Val1(3) + Val1(8) + Val1(3)) + Val1(8) + Val1(8)) + Val1(9) ) +
( ((Val1(8) + Val1(6) + Val1(4)) + (Val1(9) + Val1(6) - Val1(7) + Val1(4)) + (Val1(6) + Val1(9) - Val1(5) + Val1(4)) + Val1(3)) + Val1(5) + Val1(9) + (Val1(9) + Val1(9) - Val1(7)) - Val1(3) + Val1(2) ) +
( Val1(8) + Val1(4) + (Val1(9) - Val1(9) + Val1(2) - Val1(8) + Val1(4)) + Val1(5) - Val1(4) ) +
( Val1(9) - (Val1(3) - (Val1(7) + Val1(8) - Val1(7) - Val1(8)) + Val1(5) - Val1(7) - (Val1(5) - Val1(6) + Val1(7) - Val1(4) + Val1(6))) + Val1(9) ) +
( (Val1(8) + Val1(3) - Val1(6) + Val1(8) + Val1(8)) - Val1(5) - Val1(3) ) +
( ((Val1(5) + Val1(7) + Val1(4) + Val1(2) - Val1(6)) + (Val1(6) - Val1(4) + Val1(6) - Val1(3) + Val1(8)) - Val1(5) + Val1(8) + Val1(8) - (Val1(5) - Val1(6) + Val1(2))) + (Val1(2) - Val1(4) + Val1(9) - Val1(7) - Val1(6) - (Val1(9) - Val1(6) + Val1(6) - Val1(8))) - Val1(6) - Val1(9) + Val1(3) ) +
( ((Val1(3) - Val1(7)) - Val1(7) - Val1(2) - Val1(8) + Val1(2) - (Val1(2) + Val1(6) + Val1(7) - Val1(3) + Val1(2) - Val1(5))) + (Val1(6) - Val1(3)) - (Val1(4) - Val1(5) + Val1(4) + Val1(9)) + Val1(6) + (Val1(9) - Val1(7) - Val1(5) + Val1(5) - Val1(9)) + Val1(3) ) +
( Val1(6) + Val1(4) + Val1(5) + Val1(8) + (Val1(2) - (Val1(2) - Val1(2) + Val1(8)) - Val1(9) - Val1(7)) + Val1(6) ) +
( Val1(5) - Val1(4) + Val1(7) + (Val1(2) + Val1(6) - Val1(3) + (Val1(2) - Val1(2) - Val1(2))) ) +
( (Val1(8) - Val1(3) + (Val1(2) - Val1(6) - Val1(4) - Val1(2) - Val1(9)) + Val1(5) + Val1(9) - Val1(8)) - Val1(5) + Val1(7) ) +
( Val1(5) - Val1(5) - (Val1(3) + Val1(4) - Val1(3)) + ((Val1(7) - Val1(3) + Val1(5) + Val1(8)) + Val1(2)) + Val1(6) ) +
( ((Val1(9) - Val1(5)) + Val1(4)) - Val1(6) + Val1(4) - ((Val1(5) - Val1(2) - Val1(9)) + Val1(5) + Val1(3)) + Val1(6) + Val1(3) ) +
( Val1(9) - (Val1(2) + Val1(4) + Val1(4) - (Val1(5) - Val1(9) - Val1(3) + Val1(4) - Val1(5) + Val1(8)) - (Val1(3) - Val1(7)) - Val1(6)) + Val1(6) ) +
( (Val1(3) + Val1(2) - Val1(5) + Val1(5)) - (Val1(9) - Val1(3) + Val1(5) - Val1(7)) + Val1(3) - Val1(9) ) +
( Val1(6) - ((Val1(7) + Val1(9) + Val1(9) - Val1(2) - Val1(4)) - Val1(8)) + Val1(9) + (Val1(6) - Val1(8)) - Val1(3) ) +
( Val1(4) + Val1(9) - Val1(8) + (Val1(3) - Val1(4) - Val1(4) - Val1(7) - (Val1(7) - Val1(6) + Val1(7) - Val1(2) - Val1(7) + Val1(8))) ) +
( (Val1(7) + Val1(7)) + Val1(7) - ((Val1(7) + Val1(3) - Val1(4) + Val1(8) + Val1(8) + Val1(6)) + Val1(9) + Val1(2) - Val1(6) - Val1(9) + Val1(2)) ) +
( Val1(9) + Val1(7) + Val1(9) - Val1(9) + Val1(5) + Val1(5) ) +
( Val1(8) + Val1(2) + Val1(3) + Val1(2) + Val1(6) ) +
( Val1(8) + (Val1(4) - (Val1(3) - Val1(2)) + Val1(9) + Val1(6)) ) +
( Val1(9) - Val1(8) - Val1(2) + (Val1(6) + Val1(6) - Val1(9)) + Val1(8) - Val1(8) ) +
( Val1(7) + Val1(4) - Val1(2) - Val1(5) + Val1(4) + Val1(7) ) +
( Val1(5) - Val1(8) - Val1(4) - Val1(2) - (Val1(6) + Val1(5) + Val1(2)) ) +
( Val1(3) - Val1(5) - (Val1(3) + Val1(9) - Val1(2)) - (Val1(3) + Val1(5)) ) +
( Val1(3) - Val1(6) - Val1(7) - (Val1(2) + Val1(8) - (Val1(8) + Val1(6) + Val1(3) - Val1(9) + Val1(9) - Val1(5)) - Val1(3) + Val1(8) - (Val1(6) + Val1(3) - Val1(5) + Val1(8) + Val1(4))) - (Val1(9) + Val1(5) + Val1(9)) - Val1(5) ) +
( Val1(5) - Val1(3) ) +
( (Val1(3) - Val1(2) + Val1(7) + Val1(8)) - Val1(2) ) +
( ((Val1(7) + Val1(8) - Val1(4)) + Val1(6) + Val1(9) - (Val1(6) + Val1(5) + Val1(4) + Val1(9) + Val1(7) - Val1(6)) + Val1(9) + Val1(5)) + Val1(5) - Val1(4) ) +
( Val1(3) - Val1(9) - Val1(6) - Val1(4) - ((Val1(4) + Val1(8) + Val1(6)) + (Val1(4) + Val1(6) - Val1(2) + Val1(9) + Val1(7) - Val1(2)) + Val1(7) - Val1(2) + (Val1(5) + Val1(9) - Val1(2) + Val1(2))) ) +
( (Val1(8) + Val1(6) + Val1(5)) - Val1(4) - Val1(8) ) +
( Val1(7) - Val1(4) + Val1(8) + (Val1(4) + Val1(9) + Val1(3) + Val1(5) + Val1(2) + Val1(6)) + (Val1(7) + Val1(2) - Val1(5) - Val1(7)) + Val1(3) ) +
( Val1(6) + Val1(7) - Val1(4) + (Val1(8) - Val1(3) + Val1(8) - (Val1(7) - Val1(5) + Val1(3) - Val1(4) - Val1(5)) - Val1(9) - Val1(2)) - Val1(2) + Val1(3) ) +
( Val1(9) + Val1(6) - (Val1(6) - (Val1(4) + Val1(7))) + Val1(4) - Val1(7) ) +
( (Val1(4) + Val1(5) + Val1(8)) - Val1(8) + (Val1(8) - Val1(7) + Val1(9) - Val1(5)) - Val1(9) ) +
( Val1(5) - Val1(3) - (Val1(9) - Val1(9) + Val1(5) - Val1(6) - Val1(7) + Val1(6)) + Val1(7) + Val1(5) ) +
( Val1(7) + Val1(3) - Val1(7) - ((Val1(4) + Val1(6) - Val1(4) + Val1(8)) + Val1(5) + Val1(9) + Val1(7) - Val1(5)) ) +
( Val1(9) - (Val1(8) + Val1(9) + Val1(3) - Val1(5) - (Val1(4) + Val1(6) - Val1(4) + Val1(7) + Val1(7)) + (Val1(3) - Val1(8) - Val1(2) + Val1(2) + Val1(8))) - Val1(8) + Val1(8) - (Val1(3) - Val1(9)) + Val1(6) ) +
( Val1(2) + (Val1(7) - Val1(9) - Val1(3)) + (Val1(5) - Val1(8) - Val1(8) - (Val1(2) + Val1(2) - Val1(2) - Val1(6) - Val1(7) + Val1(6))) - (Val1(9) - Val1(6) + Val1(2)) ) +
( Val1(8) - (Val1(4) + (Val1(8) + Val1(3) - Val1(4) - Val1(2) - Val1(5)) - (Val1(8) - Val1(2)) + (Val1(2) + Val1(2) + Val1(7) - Val1(6) + Val1(3) - Val1(2)) + (Val1(2) - Val1(7))) - Val1(7) ) +
( Val1(3) - Val1(7) - Val1(3) + Val1(4) + (Val1(2) + (Val1(6) - Val1(3)) + (Val1(2) + Val1(5) + Val1(8) + Val1(3) - Val1(3)) + Val1(2) + (Val1(4) - Val1(7) + Val1(2))) - Val1(4) ) +
( (Val1(7) - Val1(7) + Val1(5) - Val1(8)) - Val1(3) + Val1(2) + Val1(9) + (Val1(2) - Val1(8) - Val1(4) + Val1(7) + Val1(4) - Val1(8)) - (Val1(7) + Val1(8) - Val1(5)) ) +
( Val1(2) - ((Val1(7) - Val1(5) - Val1(4)) - (Val1(4) + Val1(7) - Val1(9) + Val1(3) + Val1(3) - Val1(6)) + (Val1(5) + Val1(5) + Val1(9) + Val1(7) - Val1(8)) - (Val1(5) - Val1(2) - Val1(8) + Val1(4) - Val1(6)) + (Val1(5) + Val1(4) + Val1(3) - Val1(9) - Val1(4) + Val1(9))) - Val1(6) - Val1(8) - Val1(5) ) +
( (Val1(8) + Val1(2)) - Val1(3) + Val1(8) - ((Val1(5) - Val1(9) - Val1(7) - Val1(7)) - Val1(7)) + Val1(8) ) +
( (Val1(3) + Val1(7) + Val1(9) + Val1(7)) - Val1(9) - (Val1(2) + Val1(5) - Val1(8) - Val1(9)) + (Val1(5) - Val1(5) + Val1(9)) + (Val1(5) + (Val1(4) - Val1(4) - Val1(7) - Val1(9)) + Val1(4) - (Val1(2) - Val1(2) + Val1(5))) ) +
( Val1(3) + (Val1(4) + Val1(8) + Val1(3) + Val1(4) + Val1(7) + Val1(6)) + Val1(4) + Val1(3) - Val1(4) + ((Val1(5) - Val1(6)) + Val1(2) - Val1(5) - Val1(2) + Val1(8) - Val1(3)) ) +

Val1(0)
}

// cat data/day18.txt | sed 's/\([0-9]\)/Val2(\1)/g;s/^/( /;s/$/ ) \+/;s/\*/-/g;s/\+/*/g;s/-/+/g'
#[rustfmt::skip]
fn eval_data_part2() -> Val2 {
( (Val2(9) + Val2(3) * Val2(4)) + (Val2(7) * (Val2(2) * Val2(3) * Val2(6) + Val2(2)) * Val2(3)) + Val2(4) + Val2(5) ) *
( (Val2(8) * Val2(4) * Val2(9) * (Val2(7) + Val2(9) * Val2(4) * Val2(6) * Val2(9)) + Val2(3)) * Val2(6) * Val2(5) * ((Val2(8) + Val2(2)) * Val2(2) + Val2(9)) + Val2(5) ) *
( Val2(5) + Val2(6) + Val2(8) + (Val2(9) * Val2(7) + Val2(8) + Val2(9)) * Val2(6) * Val2(2) ) *
( Val2(9) + Val2(9) * (Val2(4) + (Val2(6) + Val2(7) + Val2(7) * Val2(4) + Val2(6)) + Val2(5) * Val2(6) + (Val2(2) + Val2(5))) + (Val2(8) * Val2(7) * Val2(6) * Val2(2) + Val2(7)) * Val2(5) ) *
( Val2(5) * (Val2(7) * Val2(3) + Val2(7) * Val2(9) + (Val2(7) * Val2(7) * Val2(4) * Val2(3) + Val2(3))) * Val2(8) + Val2(4) ) *
( (Val2(6) * Val2(2) * Val2(5) * Val2(4)) + Val2(2) + Val2(9) + Val2(2) + Val2(8) ) *
( Val2(7) * Val2(8) * ((Val2(4) * Val2(6) * Val2(6)) + Val2(6) * Val2(9) * Val2(3) + Val2(8) * Val2(9)) * Val2(5) ) *
( Val2(3) * ((Val2(6) + Val2(6) + Val2(7) + Val2(6)) + Val2(6) * Val2(6) + Val2(4)) * (Val2(2) * Val2(5) + Val2(6) * Val2(6) * (Val2(2) + Val2(4)) * Val2(6)) + Val2(9) ) *
( (Val2(2) + (Val2(4) + Val2(9) + Val2(3)) + Val2(8) * Val2(6) * (Val2(7) * Val2(2) * Val2(6))) + Val2(6) + (Val2(5) + Val2(4) * Val2(8) * Val2(7) * (Val2(7) * Val2(2) + Val2(3))) ) *
( Val2(8) + ((Val2(7) * Val2(4) * Val2(9)) + Val2(5) * Val2(6) * (Val2(4) * Val2(9) * Val2(8) + Val2(3) + Val2(8) * Val2(3)) + Val2(5) + (Val2(5) * Val2(8) * Val2(7) * Val2(3) + Val2(7) + Val2(4))) ) *
( Val2(6) + Val2(5) + Val2(5) * (Val2(8) + Val2(5) + Val2(8) + Val2(5) * Val2(7)) * ((Val2(2) * Val2(2) + Val2(6) + Val2(3)) + Val2(4)) + Val2(6) ) *
( Val2(2) * Val2(2) + Val2(9) * Val2(8) + (Val2(3) + Val2(7) + Val2(7) + (Val2(2) * Val2(5)) + Val2(3) * Val2(2)) ) *
( Val2(6) * Val2(2) ) *
( Val2(9) + Val2(2) * ((Val2(6) + Val2(7) + Val2(7) + Val2(2)) * Val2(8)) * (Val2(7) + (Val2(4) + Val2(6) * Val2(7)) + Val2(2) + Val2(2) + Val2(9)) * Val2(4) + Val2(8) ) *
( (Val2(7) + Val2(9) * Val2(9) + Val2(2) + Val2(4) + Val2(9)) * Val2(4) + Val2(2) + Val2(9) * Val2(6) ) *
( Val2(6) + Val2(3) * Val2(7) * Val2(9) * Val2(9) * ((Val2(6) * Val2(6) * Val2(7) + Val2(5)) + Val2(6) + Val2(8) + Val2(5)) ) *
( ((Val2(8) + Val2(8) + Val2(6) * Val2(6) + Val2(4)) * Val2(3) * Val2(3) * (Val2(3) + Val2(5) * Val2(3) + Val2(5) * Val2(4) * Val2(7))) + Val2(8) * (Val2(6) * Val2(7) + Val2(2) * Val2(2)) * (Val2(9) * Val2(9)) ) *
( Val2(7) * Val2(8) * Val2(3) + Val2(5) * (Val2(5) + Val2(5) * Val2(6) * Val2(6)) + (Val2(9) + Val2(6)) ) *
( Val2(5) * Val2(5) * Val2(2) ) *
( ((Val2(3) * Val2(3)) + Val2(2) * (Val2(5) * Val2(8) + Val2(2) * Val2(4) + Val2(3))) + Val2(8) ) *
( ((Val2(5) * Val2(6) * Val2(5) + Val2(8) * Val2(4)) * Val2(9) * (Val2(5) * Val2(5))) + (Val2(7) + Val2(2) * (Val2(8) * Val2(7))) * Val2(9) ) *
( Val2(9) + (Val2(9) + Val2(8)) * (Val2(3) + Val2(2) + (Val2(9) * Val2(2) + Val2(9) + Val2(5)) + Val2(4) + Val2(2)) * Val2(5) ) *
( Val2(4) + Val2(4) * (Val2(2) * Val2(5) * Val2(5) + (Val2(3) * Val2(6) + Val2(4) * Val2(7))) + (Val2(2) + Val2(2)) * Val2(6) ) *
( Val2(6) * Val2(2) ) *
( Val2(6) * Val2(5) * (Val2(4) * Val2(8)) * Val2(5) ) *
( Val2(6) + (Val2(4) + Val2(6)) * (Val2(4) + Val2(5) * Val2(4) * Val2(5) * Val2(9)) * Val2(7) + (Val2(2) * Val2(7) * Val2(9)) ) *
( Val2(9) + (Val2(3) + (Val2(5) * Val2(9) + Val2(4) + Val2(5) * Val2(3) + Val2(9))) * ((Val2(4) + Val2(4) + Val2(3) + Val2(6)) + (Val2(9) + Val2(8) + Val2(3)) + Val2(9) + Val2(9)) * Val2(3) ) *
( Val2(5) * (Val2(6) + (Val2(3) * Val2(9) * Val2(7) + Val2(6) * Val2(2)) + (Val2(2) * Val2(4) + Val2(2) + Val2(4) + Val2(6)) + Val2(9) * Val2(3)) * Val2(6) * Val2(8) * Val2(5) + Val2(3) ) *
( Val2(8) * Val2(9) + (Val2(8) * (Val2(3) * Val2(2) * Val2(4)) + Val2(4) * Val2(3) + Val2(4) + Val2(5)) ) *
( Val2(9) * Val2(5) * Val2(8) * Val2(9) + (Val2(8) * (Val2(4) + Val2(2) + Val2(9) * Val2(8) * Val2(7)) + (Val2(8) * Val2(6) + Val2(3)) + Val2(7) + Val2(7)) ) *
( Val2(6) + ((Val2(9) * Val2(2)) + (Val2(7) * Val2(8) * Val2(5) * Val2(9) + Val2(7)) + Val2(4)) + Val2(4) + ((Val2(7) * Val2(7) * Val2(4) + Val2(5) + Val2(2)) * Val2(2)) * Val2(9) ) *
( (Val2(4) * Val2(9) + Val2(8)) + (Val2(6) * (Val2(7) + Val2(6)) + Val2(9) * Val2(8) + Val2(2) + Val2(7)) * Val2(4) * Val2(9) * Val2(3) + (Val2(6) * Val2(8) * (Val2(8) + Val2(4) * Val2(7) + Val2(3)) + Val2(8)) ) *
( (Val2(5) + Val2(9) + Val2(2) + Val2(7) + Val2(8) * Val2(6)) * (Val2(5) * Val2(5)) ) *
( Val2(6) * Val2(7) + Val2(9) * Val2(3) ) *
( ((Val2(7) * Val2(8) + Val2(3) + Val2(7) * Val2(5)) + Val2(7) + Val2(5) * Val2(6) + Val2(6) * Val2(5)) * Val2(9) * Val2(4) ) *
( (Val2(5) + Val2(2) + Val2(4) + Val2(9) + Val2(2) * (Val2(8) * Val2(6) + Val2(7) * Val2(6))) * ((Val2(4) + Val2(4) + Val2(6) + Val2(9) * Val2(7)) * Val2(9) + Val2(7)) ) *
( (Val2(3) * (Val2(5) + Val2(8) * Val2(4) * Val2(4) + Val2(7))) + Val2(7) * Val2(8) * (Val2(7) * (Val2(3) + Val2(5) + Val2(4) * Val2(6) * Val2(9))) * ((Val2(7) * Val2(2) * Val2(7) + Val2(9) * Val2(6) + Val2(8)) * Val2(3) * Val2(3) * (Val2(2) + Val2(9) + Val2(7)) * Val2(4) * (Val2(6) * Val2(3) + Val2(5))) + Val2(9) ) *
( Val2(5) + Val2(4) * ((Val2(6) * Val2(8) + Val2(5) + Val2(6)) + Val2(8)) * Val2(7) * Val2(7) ) *
( Val2(5) + (Val2(4) + (Val2(2) + Val2(9) + Val2(7) + Val2(7)) + Val2(5)) * ((Val2(6) + Val2(2) + Val2(9) * Val2(2)) + Val2(3) + Val2(2) * Val2(7) * Val2(3)) * Val2(9) * Val2(8) ) *
( (Val2(7) * Val2(3) + Val2(3) + Val2(6) + Val2(2) + Val2(3)) + Val2(6) * Val2(3) + ((Val2(3) + Val2(4) + Val2(3) + Val2(9) + Val2(7)) * Val2(9) * Val2(8)) + Val2(9) ) *
( ((Val2(4) * Val2(8) + Val2(9)) * Val2(5) + Val2(4) + Val2(2) + Val2(8) + Val2(4)) + (Val2(6) * Val2(2) + (Val2(9) * Val2(5)) + Val2(9) + Val2(7) * Val2(8)) ) *
( Val2(9) * Val2(8) * (Val2(3) + Val2(3) + (Val2(9) * Val2(2) * Val2(8) + Val2(7) + Val2(9))) ) *
( (Val2(5) + Val2(8) + (Val2(7) + Val2(2) * Val2(8))) * Val2(2) + Val2(8) * Val2(8) * Val2(6) + Val2(5) ) *
( Val2(2) * (Val2(2) + Val2(9) + Val2(5) * Val2(4)) + (Val2(5) * Val2(9) + Val2(3) + Val2(4)) + Val2(4) * Val2(2) ) *
( Val2(8) + Val2(2) + (Val2(7) + Val2(7) + Val2(4) * Val2(5) * Val2(9) * Val2(4)) ) *
( Val2(4) + Val2(9) * Val2(2) + Val2(8) ) *
( Val2(6) * Val2(2) * ((Val2(5) * Val2(3) + Val2(5) * Val2(7) * Val2(9)) * Val2(4) + Val2(6) * Val2(6)) + Val2(5) ) *
( (Val2(8) + Val2(3) + Val2(7)) * (Val2(4) + Val2(5) * Val2(8) + Val2(7) * Val2(6)) ) *
( Val2(3) + Val2(7) + ((Val2(3) * Val2(3) + Val2(5) * Val2(9) * Val2(7)) * Val2(9) + Val2(2) * (Val2(4) * Val2(3))) ) *
( (Val2(4) * Val2(2) + Val2(2) * Val2(3) + Val2(2) * Val2(2)) + Val2(4) * Val2(2) + Val2(4) * Val2(6) * Val2(3) ) *
( ((Val2(9) + Val2(7) + Val2(7) * Val2(8)) + (Val2(2) + Val2(5) * Val2(2) * Val2(3)) + Val2(2) + Val2(4) + Val2(7)) + Val2(9) * Val2(5) * Val2(5) ) *
( Val2(4) * Val2(9) + Val2(2) * Val2(5) * (Val2(3) * Val2(5) * (Val2(4) * Val2(3) + Val2(3) + Val2(7) + Val2(5) + Val2(8)) * Val2(9) + Val2(2) + Val2(2)) + (Val2(4) * (Val2(2) + Val2(6)) * Val2(7) + Val2(5) + Val2(7) * Val2(2)) ) *
( Val2(6) * Val2(2) + Val2(4) + (Val2(6) * Val2(8)) + (Val2(4) * Val2(2)) + Val2(3) ) *
( Val2(3) * ((Val2(8) * Val2(8) * Val2(6)) + Val2(5)) * Val2(9) * Val2(7) * ((Val2(7) + Val2(2) + Val2(9) * Val2(5) + Val2(3) * Val2(5)) * Val2(3) * (Val2(2) + Val2(5)) * Val2(3) * Val2(5)) ) *
( Val2(4) * Val2(2) + Val2(6) * Val2(6) * Val2(6) ) *
( (Val2(5) + (Val2(4) + Val2(9) + Val2(8) * Val2(5) + Val2(8)) + Val2(6) + Val2(9) * Val2(3)) + ((Val2(2) + Val2(7) + Val2(3) * Val2(3) + Val2(4) + Val2(3)) * Val2(9) * Val2(8) * Val2(8)) * Val2(4) ) *
( Val2(4) + Val2(4) * Val2(9) + Val2(9) * (Val2(2) + (Val2(6) + Val2(5) * Val2(9) * Val2(7) + Val2(3)) * (Val2(2) + Val2(8) * Val2(3) * Val2(2) * Val2(3))) ) *
( Val2(6) + Val2(4) * Val2(5) + (Val2(6) + (Val2(6) + Val2(7) * Val2(2) + Val2(6) * Val2(4) + Val2(7))) + Val2(4) * ((Val2(5) + Val2(9) + Val2(8) + Val2(9) + Val2(5) + Val2(5)) + Val2(5)) ) *
( Val2(3) * (Val2(5) + Val2(4) * (Val2(3) * Val2(6) + Val2(2) * Val2(8) * Val2(7) + Val2(8)) * Val2(3) * (Val2(9) * Val2(8) + Val2(6))) + (Val2(5) * Val2(6) + (Val2(8) + Val2(9) * Val2(4) + Val2(9) * Val2(5)) + Val2(7) + Val2(5)) * Val2(5) ) *
( Val2(9) * Val2(2) * Val2(9) * (Val2(3) * Val2(9)) * (Val2(4) + Val2(4) * Val2(7) + Val2(5)) * Val2(5) ) *
( Val2(5) * Val2(6) ) *
( Val2(6) * (Val2(7) * Val2(2) + Val2(6) + Val2(9) + Val2(7) * Val2(5)) + Val2(9) * (Val2(4) + Val2(5) + Val2(8)) * Val2(2) + Val2(3) ) *
( Val2(8) * Val2(7) + (Val2(9) * (Val2(9) * Val2(3) + Val2(3) + Val2(7)) + (Val2(5) * Val2(6) + Val2(3) + Val2(7) * Val2(8) + Val2(2))) + Val2(7) ) *
( (Val2(5) + Val2(7) * Val2(5) * Val2(3) * Val2(6)) + Val2(8) ) *
( Val2(5) * ((Val2(9) * Val2(5)) * Val2(9) * Val2(4) + (Val2(5) + Val2(4) + Val2(7) * Val2(3)) + Val2(3)) ) *
( Val2(5) + Val2(8) + Val2(7) * Val2(8) + ((Val2(8) * Val2(9) * Val2(3) + Val2(6) + Val2(5)) + Val2(4) + Val2(9) * Val2(4)) ) *
( Val2(8) + (Val2(6) * (Val2(6) * Val2(9) + Val2(7) + Val2(8) + Val2(7)) * Val2(7) + Val2(2)) * ((Val2(2) + Val2(9)) + Val2(7)) * Val2(4) ) *
( Val2(6) * ((Val2(4) * Val2(5) * Val2(6) * Val2(7)) + Val2(7)) ) *
( Val2(8) + (Val2(5) + Val2(6) + Val2(4) + Val2(8) * Val2(6) + Val2(6)) * Val2(2) + Val2(9) ) *
( Val2(7) * Val2(3) * Val2(2) + (Val2(6) * Val2(8)) ) *
( Val2(9) + ((Val2(4) * Val2(4) + Val2(9) * Val2(6) + Val2(8) + Val2(7)) * Val2(9) * Val2(2) * Val2(6) + (Val2(4) + Val2(9) * Val2(7) * Val2(6)) + Val2(3)) + Val2(5) * Val2(7) ) *
( ((Val2(2) * Val2(3) + Val2(2) + Val2(9) * Val2(6)) * Val2(4) * Val2(8) + Val2(2) + Val2(5) * Val2(8)) + Val2(5) * Val2(5) * (Val2(9) + Val2(6)) * Val2(9) * (Val2(3) + Val2(2) + Val2(8)) ) *
( Val2(4) * (Val2(6) * Val2(2)) + Val2(9) * (Val2(5) * Val2(4) + (Val2(8) + Val2(3) * Val2(7) * Val2(6)) + Val2(4)) + ((Val2(7) * Val2(2) * Val2(3)) + Val2(6) * Val2(9) + (Val2(6) * Val2(7) + Val2(2))) ) *
( Val2(6) * Val2(2) * ((Val2(2) + Val2(4) + Val2(8)) + Val2(2) + Val2(4) + Val2(7) * Val2(9)) + (Val2(7) * Val2(8) * (Val2(6) * Val2(7) + Val2(9) * Val2(9) + Val2(7)) * Val2(4)) + Val2(8) ) *
( Val2(9) + Val2(9) * Val2(7) * Val2(9) + (Val2(9) + Val2(4)) + Val2(9) ) *
( (Val2(7) + Val2(3) * (Val2(3) + Val2(7) * Val2(4) + Val2(4) + Val2(8)) + Val2(2) + Val2(2) + (Val2(3) + Val2(3))) * Val2(8) ) *
( Val2(9) + Val2(6) * Val2(3) + (Val2(2) + Val2(5) + Val2(7) + Val2(7) * Val2(7)) + Val2(8) * (Val2(2) * Val2(4)) ) *
( (Val2(8) + Val2(9)) * Val2(8) + Val2(6) ) *
( (Val2(3) + Val2(2) * Val2(6) * (Val2(7) + Val2(8) + Val2(5) + Val2(4))) * Val2(5) + Val2(6) ) *
( Val2(4) * Val2(3) * (Val2(9) * Val2(3) * (Val2(2) + Val2(9) + Val2(6)) * Val2(7)) + Val2(8) * Val2(3) + Val2(8) ) *
( Val2(9) * Val2(6) + Val2(8) * Val2(4) * Val2(9) + (Val2(9) + (Val2(8) + Val2(4) * Val2(9)) * Val2(3)) ) *
( Val2(5) * Val2(4) + Val2(6) + (Val2(8) + (Val2(2) * Val2(9) * Val2(6)) + (Val2(6) + Val2(3) * Val2(2)) * Val2(6)) + (Val2(8) + Val2(8) + Val2(3) + (Val2(6) + Val2(9)) + Val2(3) * (Val2(4) * Val2(4) * Val2(2) + Val2(2) + Val2(6))) * Val2(6) ) *
( Val2(4) * (Val2(6) * Val2(6) + (Val2(4) + Val2(7) + Val2(7)) * Val2(5)) + Val2(5) * Val2(6) + (Val2(7) * Val2(2) + Val2(4) + Val2(2) * Val2(9) * Val2(9)) ) *
( ((Val2(8) * Val2(7)) + (Val2(7) * Val2(5) + Val2(9)) * (Val2(9) * Val2(2) + Val2(6) + Val2(2)) * Val2(6) + Val2(7)) + (Val2(3) * Val2(3) + Val2(2) * Val2(3) * Val2(7)) + Val2(4) * Val2(7) + Val2(7) ) *
( (Val2(6) * Val2(9) + Val2(7) + Val2(4) * Val2(7)) + Val2(6) * Val2(7) * Val2(5) * Val2(3) * Val2(8) ) *
( Val2(9) * Val2(2) * (Val2(9) + (Val2(8) + Val2(9) + Val2(2) + Val2(6) * Val2(2) * Val2(5))) * Val2(6) * Val2(4) * Val2(8) ) *
( Val2(6) * Val2(4) + ((Val2(2) + Val2(9)) + Val2(6) + Val2(5)) * Val2(2) + Val2(9) ) *
( (Val2(6) * Val2(8) + Val2(2) * (Val2(5) * Val2(2) + Val2(7) + Val2(7)) + Val2(5) * Val2(4)) + (Val2(2) + Val2(3) * Val2(8) + (Val2(9) * Val2(6) + Val2(4) * Val2(3) * Val2(6) * Val2(4)) + (Val2(2) * Val2(5) + Val2(8) + Val2(2) * Val2(4))) ) *
( Val2(3) * (Val2(9) + (Val2(2) + Val2(7) + Val2(6)) + Val2(9) * Val2(7) * Val2(3)) * (Val2(7) + (Val2(5) + Val2(6) * Val2(7)) * Val2(3) + (Val2(5) + Val2(7) + Val2(5) + Val2(4)) + Val2(8) * Val2(6)) + Val2(4) + Val2(5) * Val2(9) ) *
( (Val2(6) * Val2(6) * Val2(4)) * Val2(3) + Val2(4) * Val2(3) ) *
( Val2(9) + (Val2(2) + Val2(7) * Val2(9) * (Val2(4) * Val2(2)) + Val2(5)) * (Val2(2) * Val2(5) * (Val2(7) * Val2(6))) + Val2(8) ) *
( Val2(2) + (Val2(6) + Val2(6) + Val2(9) * Val2(8)) * Val2(4) + Val2(2) ) *
( ((Val2(4) + Val2(3)) * Val2(8)) * Val2(5) * Val2(5) ) *
( Val2(5) * Val2(8) + Val2(7) * Val2(6) * Val2(6) + ((Val2(9) + Val2(2) + Val2(5) * Val2(4) + Val2(7)) * Val2(8) * Val2(6) * Val2(5) * (Val2(3) * Val2(7))) ) *
( Val2(8) * (Val2(8) + Val2(8) + Val2(8) + (Val2(3) + Val2(7) + Val2(8)) + Val2(4)) + Val2(8) * Val2(5) + Val2(8) + Val2(3) ) *
( (Val2(8) + Val2(3) + Val2(8)) + Val2(4) + Val2(3) + Val2(3) + Val2(6) ) *
( Val2(5) * Val2(6) + (Val2(9) * (Val2(5) * Val2(6) * Val2(8) * Val2(3) + Val2(4)) + (Val2(6) + Val2(3) * Val2(3) * Val2(4) + Val2(6) * Val2(2))) ) *
( Val2(3) + (Val2(2) + (Val2(5) + Val2(9)) + Val2(5) * Val2(8) * Val2(5) + Val2(6)) + Val2(8) * (Val2(8) * Val2(5) * Val2(7) + (Val2(6) * Val2(5) * Val2(9) + Val2(2) + Val2(9)) * Val2(2) + (Val2(3) + Val2(2) + Val2(2) * Val2(9) + Val2(4))) ) *
( (Val2(2) + Val2(8) * Val2(5) * Val2(7) * (Val2(8) * Val2(2) * Val2(7) + Val2(5) + Val2(5) * Val2(4)) + Val2(2)) * Val2(5) + ((Val2(8) + Val2(7) * Val2(5)) + Val2(3) + Val2(4) * (Val2(4) + Val2(7) + Val2(6)) + (Val2(7) * Val2(6) * Val2(4) * Val2(7) + Val2(7)) * (Val2(8) + Val2(7) * Val2(2) + Val2(2) * Val2(9) * Val2(3))) * ((Val2(4) * Val2(8)) * Val2(9)) ) *
( Val2(3) + (Val2(8) * (Val2(2) * Val2(9)) * Val2(2) + Val2(8) * (Val2(9) + Val2(4) + Val2(9))) + Val2(2) + Val2(3) + Val2(4) ) *
( (Val2(4) * (Val2(9) + Val2(4)) * Val2(5)) + Val2(8) + (Val2(3) * Val2(7) + Val2(3) + Val2(7) * Val2(4)) + Val2(7) + Val2(2) ) *
( Val2(2) + Val2(4) + (Val2(2) + Val2(8) * Val2(9) * Val2(7) * (Val2(8) + Val2(3) + Val2(6) * Val2(3) * Val2(5)) * Val2(9)) * (Val2(5) + Val2(8) + Val2(8) * (Val2(4) + Val2(5) + Val2(2) * Val2(4) + Val2(2) * Val2(5)) + Val2(4)) * Val2(2) + Val2(6) ) *
( Val2(7) * Val2(8) * ((Val2(5) * Val2(7) * Val2(2) * Val2(9) + Val2(4) + Val2(3)) + Val2(4)) * Val2(7) * Val2(6) * Val2(8) ) *
( ((Val2(4) * Val2(4) + Val2(4) + Val2(9) + Val2(6) * Val2(9)) + (Val2(7) * Val2(9)) + Val2(2) * Val2(8)) + Val2(8) + Val2(6) * Val2(9) ) *
( (Val2(2) + (Val2(6) * Val2(7) + Val2(3) * Val2(6) + Val2(7)) * Val2(4) + Val2(2) * (Val2(3) * Val2(8) * Val2(9)) + Val2(8)) + Val2(8) * Val2(8) * Val2(8) * (Val2(2) * Val2(8) + Val2(5) + Val2(9)) ) *
( (Val2(7) * Val2(5) * (Val2(7) * Val2(9) * Val2(5)) * Val2(2)) + Val2(4) * Val2(9) ) *
( Val2(3) + Val2(6) * Val2(8) * Val2(2) + Val2(7) ) *
( Val2(6) * Val2(2) * ((Val2(7) + Val2(6) * Val2(3) * Val2(4)) + Val2(8) + (Val2(2) * Val2(6) + Val2(5)) + Val2(6)) * Val2(4) + Val2(2) ) *
( (Val2(4) + Val2(8)) * Val2(7) + Val2(3) * Val2(3) + Val2(7) * Val2(5) ) *
( Val2(5) * Val2(6) * Val2(3) * Val2(7) + (Val2(2) + Val2(7)) * (Val2(5) * (Val2(3) * Val2(4) * Val2(3) * Val2(4) * Val2(7)) * Val2(8)) ) *
( Val2(2) * ((Val2(4) * Val2(7) + Val2(6)) * Val2(8) * Val2(9) + Val2(6) + Val2(9)) ) *
( (Val2(6) + Val2(2)) + (Val2(7) * Val2(5) + (Val2(7) + Val2(6) * Val2(7) * Val2(3) * Val2(8) * Val2(7)) * Val2(2) * Val2(5) * Val2(8)) * Val2(6) ) *
( (Val2(3) + Val2(2) + Val2(8) * Val2(7)) * (Val2(5) + Val2(4) + Val2(9) * Val2(6)) * Val2(4) + Val2(3) * Val2(4) ) *
( Val2(8) * (Val2(8) * Val2(2) + Val2(3) * Val2(2) * Val2(2)) ) *
( (Val2(7) * Val2(8) + Val2(5)) + (Val2(3) + (Val2(9) * Val2(5) * Val2(6) * Val2(9) * Val2(3) * Val2(9)) + Val2(3)) * Val2(8) * Val2(7) + (Val2(5) * Val2(9) * Val2(3) + Val2(2) * Val2(4) * Val2(5)) * Val2(4) ) *
( Val2(7) + ((Val2(6) + Val2(7) + Val2(2) + Val2(9) * Val2(8)) * Val2(8) + Val2(6) + Val2(8) + Val2(5)) + Val2(2) * Val2(3) * Val2(8) + Val2(3) ) *
( (Val2(4) * (Val2(5) + Val2(7) * Val2(3) * Val2(4) * Val2(9))) + Val2(7) ) *
( Val2(7) + Val2(5) + Val2(2) + (Val2(4) * Val2(4) * Val2(6) * Val2(3) + Val2(7)) * (Val2(8) + (Val2(6) * Val2(2) * Val2(8) + Val2(9) * Val2(6)) + Val2(3) + Val2(6) * Val2(3) * (Val2(4) * Val2(5) * Val2(9))) + Val2(7) ) *
( Val2(3) + Val2(4) * Val2(2) ) *
( ((Val2(6) * Val2(9) * Val2(5) * Val2(4) * Val2(9) + Val2(7)) + Val2(9) * Val2(8) * (Val2(9) + Val2(2) + Val2(6) + Val2(4) + Val2(7) + Val2(3))) * (Val2(8) * Val2(6) * Val2(7) * Val2(4)) + Val2(9) + (Val2(8) + Val2(5)) * Val2(7) ) *
( (Val2(9) + Val2(8) * Val2(4)) * (Val2(3) + Val2(4) * (Val2(9) + Val2(5) + Val2(3) + Val2(4)) + Val2(9)) ) *
( (Val2(7) * Val2(8) + Val2(3) * Val2(6) + (Val2(4) * Val2(5) * Val2(7) + Val2(8) * Val2(9)) * Val2(4)) + Val2(3) ) *
( ((Val2(7) + Val2(4)) * (Val2(6) + Val2(7) + Val2(6)) + Val2(7) * Val2(9) + Val2(3)) * Val2(2) + (Val2(4) * Val2(8)) * (Val2(4) + Val2(5) * Val2(8) * Val2(8) * Val2(5)) + (Val2(4) * Val2(9)) + Val2(5) ) *
( (Val2(2) * Val2(3) * Val2(7)) + Val2(6) * Val2(8) * (Val2(4) * Val2(7) + Val2(6) + (Val2(3) * Val2(4) * Val2(4) * Val2(7) + Val2(6)) + Val2(8) * Val2(3)) + Val2(4) ) *
( Val2(6) + (Val2(8) + Val2(3) * Val2(4) + (Val2(6) * Val2(9) + Val2(2) * Val2(3) * Val2(8) + Val2(5)) + Val2(8) * Val2(6)) ) *
( Val2(8) * ((Val2(2) + Val2(8) * Val2(5) + Val2(5) + Val2(5) + Val2(3)) + Val2(9) * Val2(5)) * Val2(9) ) *
( (Val2(8) * Val2(4) + Val2(9) + Val2(8)) * Val2(5) + Val2(5) + (Val2(7) * Val2(7) * Val2(2) + Val2(9) * Val2(8) + Val2(6)) * Val2(6) + (Val2(6) + Val2(6) * Val2(6) * Val2(3) * Val2(9) + Val2(8)) ) *
( Val2(5) + Val2(9) * (Val2(5) * (Val2(9) + Val2(5) * Val2(7)) * (Val2(2) * Val2(2) + Val2(7)) + Val2(4)) + (Val2(9) + Val2(9) * Val2(5) + Val2(4)) * Val2(9) ) *
( Val2(7) * Val2(7) + (Val2(2) + Val2(9) + Val2(6) * Val2(3) * Val2(5)) ) *
( Val2(2) * (Val2(8) + Val2(9) + Val2(7) + (Val2(4) + Val2(9) * Val2(9) + Val2(4) * Val2(5)) * Val2(4) + Val2(4)) * Val2(5) * Val2(2) * Val2(8) ) *
( (Val2(2) * (Val2(4) * Val2(9) * Val2(4) + Val2(7) + Val2(3)) * Val2(4) * Val2(8)) * (Val2(2) * Val2(4) + Val2(5) + (Val2(3) * Val2(4)) * Val2(8) + Val2(5)) ) *
( Val2(2) * ((Val2(7) + Val2(8) * Val2(6) + Val2(6)) + Val2(4) * Val2(6) * Val2(7) + Val2(6) + Val2(9)) * ((Val2(7) * Val2(9) + Val2(5) + Val2(5)) * Val2(4) + Val2(9) * Val2(6) * Val2(7)) + ((Val2(6) * Val2(9) * Val2(4) + Val2(7) + Val2(2)) + Val2(7) * Val2(9) + (Val2(7) * Val2(2) * Val2(3) + Val2(3) * Val2(5)) + Val2(5)) ) *
( (Val2(3) + Val2(8) * Val2(2) + Val2(2) * (Val2(3) + Val2(7) * Val2(8) + Val2(5) + Val2(6)) * Val2(6)) * Val2(4) * Val2(5) ) *
( Val2(8) * Val2(5) + Val2(8) * Val2(7) + Val2(2) ) *
( (Val2(3) * (Val2(6) + Val2(9) + Val2(8) * Val2(4) * Val2(8)) + Val2(4) * Val2(2)) * Val2(8) ) *
( (Val2(7) * Val2(7)) * Val2(5) + (Val2(9) + Val2(2) * Val2(9)) * ((Val2(4) + Val2(3) + Val2(3) + Val2(4) * Val2(4) + Val2(9)) + Val2(8) + Val2(8) * (Val2(3) + Val2(9) * Val2(7) * Val2(8)) + Val2(3) * Val2(9)) ) *
( (Val2(6) * (Val2(5) * Val2(7) + Val2(5)) + Val2(8) + Val2(9)) + Val2(6) * Val2(3) ) *
( Val2(3) * Val2(2) + (Val2(5) + Val2(8)) * Val2(5) * Val2(6) ) *
( Val2(7) + Val2(4) * Val2(7) * Val2(7) + Val2(4) + (Val2(4) * Val2(9) + Val2(6) + Val2(5)) ) *
( Val2(3) * Val2(4) * (Val2(6) + Val2(9)) + (Val2(6) * (Val2(5) + Val2(5) * Val2(6) + Val2(8)) * Val2(9) + (Val2(7) + Val2(3) * Val2(6))) * Val2(8) ) *
( Val2(4) * Val2(4) + Val2(6) + ((Val2(4) + Val2(3) + Val2(2)) + Val2(4) * Val2(3) * (Val2(7) * Val2(8) * Val2(7) + Val2(6) + Val2(9) + Val2(3))) + (Val2(5) * Val2(8)) ) *
( (Val2(6) * Val2(7) + Val2(8) * Val2(3) * Val2(9)) + Val2(5) + Val2(3) + Val2(3) ) *
( Val2(8) * Val2(5) * Val2(3) + Val2(8) + Val2(2) * Val2(9) ) *
( (Val2(9) + (Val2(5) * Val2(8) * Val2(4) * Val2(4) * Val2(9) * Val2(4)) + Val2(2) * Val2(7) + (Val2(6) + Val2(4) * Val2(7) * Val2(4) * Val2(8))) * Val2(9) * Val2(5) ) *
( Val2(7) * Val2(7) + Val2(3) * Val2(8) + ((Val2(5) * Val2(3) + Val2(4) + Val2(9)) + Val2(4) + Val2(6) + Val2(8)) ) *
( Val2(2) * Val2(5) * Val2(8) * (Val2(3) * Val2(3) + Val2(4) * Val2(8)) ) *
( Val2(5) * Val2(4) + Val2(3) + Val2(9) * ((Val2(5) + Val2(2) + Val2(2)) + Val2(5) * Val2(4) * (Val2(4) + Val2(2))) ) *
( (Val2(3) + Val2(7) * Val2(2)) + Val2(3) ) *
( Val2(8) * Val2(7) + Val2(3) * Val2(9) * Val2(9) + Val2(5) ) *
( Val2(2) * (Val2(2) + Val2(6) * Val2(3) * (Val2(4) * Val2(3) * Val2(4) * Val2(2) * Val2(7) + Val2(9))) * Val2(8) * Val2(2) + Val2(7) + Val2(9) ) *
( (Val2(4) + Val2(4) * Val2(4) * Val2(7)) + Val2(8) * Val2(5) ) *
( Val2(2) + (Val2(5) + (Val2(8) + Val2(5) + Val2(2) + Val2(6) + Val2(8)) + Val2(6) + Val2(3) * Val2(8) + Val2(8)) ) *
( Val2(8) * Val2(3) + Val2(9) * Val2(5) + Val2(9) + (Val2(5) + Val2(7) * Val2(6) * Val2(5) + Val2(3) * Val2(7)) ) *
( Val2(2) * (Val2(3) * Val2(7)) + Val2(8) + Val2(9) ) *
( Val2(7) * Val2(9) * Val2(6) ) *
( Val2(4) * (Val2(6) * Val2(7) * Val2(8) * Val2(4) + Val2(5)) * Val2(2) + Val2(6) + Val2(8) + Val2(2) ) *
( (Val2(9) * Val2(3) + Val2(7) + Val2(3)) + Val2(8) * (Val2(9) * Val2(5) + Val2(2) + Val2(8) * Val2(3) + Val2(4)) + Val2(8) * Val2(2) + Val2(6) ) *
( Val2(4) * Val2(2) + Val2(3) + Val2(6) + ((Val2(3) + Val2(5) + Val2(3)) + Val2(2) * Val2(6) * Val2(5) + Val2(4)) ) *
( Val2(3) * Val2(9) + (Val2(7) + (Val2(8) * Val2(7)) + Val2(9) + Val2(7) * Val2(7)) ) *
( Val2(6) * Val2(3) + Val2(6) * Val2(4) + (Val2(7) + Val2(7) * Val2(2) + Val2(9) * Val2(4) * Val2(2)) * (Val2(6) * Val2(6) * Val2(6) * Val2(7)) ) *
( (Val2(5) + Val2(4)) + Val2(6) + (Val2(5) * Val2(4) * Val2(5) * Val2(3) + (Val2(8) + Val2(6)) + Val2(5)) + Val2(7) ) *
( (Val2(8) + Val2(4)) * Val2(2) + (Val2(2) + Val2(2) + (Val2(9) + Val2(9) * Val2(8)) * Val2(8) + Val2(9) * Val2(6)) + Val2(7) ) *
( Val2(3) * Val2(5) + (Val2(4) * Val2(9) * Val2(3)) * Val2(9) + ((Val2(5) + Val2(2) + Val2(4) * Val2(2)) * Val2(6) * Val2(7) * Val2(7) * Val2(5)) ) *
( (Val2(3) * (Val2(9) * Val2(7) + Val2(3) * Val2(6) + Val2(6))) * Val2(9) + Val2(7) + Val2(4) * (Val2(9) + Val2(4) + Val2(9) * Val2(7) + Val2(7) + Val2(7)) ) *
( (Val2(3) + Val2(7)) * Val2(4) ) *
( Val2(4) * (Val2(2) * Val2(5) + Val2(5)) * Val2(6) * Val2(2) + Val2(8) * Val2(9) ) *
( Val2(8) + (Val2(6) + (Val2(7) + Val2(3)) * Val2(8) * Val2(5) * Val2(8) * (Val2(8) + Val2(2) * Val2(6) * Val2(9) * Val2(4) + Val2(3))) ) *
( Val2(7) + Val2(8) + (Val2(5) + (Val2(2) + Val2(6) + Val2(9)) + Val2(3) + Val2(6) + (Val2(6) * Val2(8) * Val2(5))) ) *
( (Val2(5) * Val2(9) + Val2(4) + Val2(7) + Val2(7) * Val2(8)) * Val2(4) ) *
( ((Val2(8) + Val2(9) + Val2(6) * Val2(4) + Val2(6) * Val2(3)) + Val2(6) * Val2(2) + (Val2(4) * Val2(8)) + Val2(7)) * (Val2(4) * Val2(4)) * (Val2(7) * Val2(7) + Val2(7) + (Val2(4) * Val2(5) * Val2(2))) ) *
( (Val2(4) * (Val2(8) * Val2(4) + Val2(4) + Val2(2) * Val2(4)) * Val2(7) * Val2(6) + Val2(2) * Val2(6)) + Val2(9) * Val2(3) + Val2(3) + (Val2(9) + Val2(9)) ) *
( (Val2(9) + Val2(5)) + Val2(6) * ((Val2(5) * Val2(3)) * Val2(9) * Val2(4)) ) *
( Val2(7) * ((Val2(8) + Val2(9) + Val2(5) * Val2(2) * Val2(2) + Val2(7)) + Val2(9) + Val2(3) + Val2(7) + Val2(2)) + Val2(6) + Val2(2) * Val2(6) + (Val2(9) + Val2(4) + Val2(2) * (Val2(2) * Val2(7) * Val2(5)) + Val2(4) * Val2(7)) ) *
( Val2(8) * (Val2(5) + Val2(4) + Val2(7) * Val2(2)) + Val2(6) * (Val2(6) + Val2(3)) + Val2(4) ) *
( Val2(7) * (Val2(9) * Val2(9) + Val2(4) * Val2(3)) + Val2(8) + (Val2(9) * (Val2(6) * Val2(9) * Val2(2) * Val2(6) * Val2(8)) + Val2(9) + Val2(7) + (Val2(2) + Val2(8) + Val2(5) + Val2(8))) + (Val2(4) * Val2(7) * Val2(8) * Val2(2)) ) *
( Val2(6) * Val2(3) + Val2(5) + Val2(8) * ((Val2(9) * Val2(5) * Val2(6)) + Val2(9) + (Val2(3) * Val2(7) + Val2(8) * Val2(6)) + (Val2(2) + Val2(9) * Val2(5))) * (Val2(7) * Val2(6)) ) *
( Val2(5) + (Val2(9) * Val2(7)) * Val2(2) + Val2(4) * Val2(3) ) *
( Val2(7) * Val2(2) + (Val2(5) * Val2(6) * Val2(9) * Val2(4) + (Val2(7) * Val2(6) + Val2(5) + Val2(6) * Val2(8)) + Val2(4)) + Val2(6) * Val2(4) ) *
( Val2(6) + Val2(8) + Val2(8) + Val2(4) * Val2(8) + (Val2(2) + (Val2(9) + Val2(4) + Val2(7) * Val2(9)) + Val2(5)) ) *
( Val2(8) + Val2(4) * (Val2(4) * Val2(7) + Val2(3) * Val2(5) * Val2(8)) * Val2(4) + Val2(7) * ((Val2(4) + Val2(9) * Val2(3) + Val2(8)) + Val2(5) + Val2(6) * Val2(7) * Val2(5) + Val2(4)) ) *
( (Val2(9) + Val2(3) * Val2(3)) * Val2(9) * ((Val2(9) * Val2(9) * Val2(6) * Val2(8)) + Val2(9) * Val2(7)) + Val2(4) * Val2(9) + ((Val2(8) * Val2(7) * Val2(8) + Val2(2) * Val2(8)) + Val2(5) * Val2(8) * Val2(3) + (Val2(3) * Val2(5) * Val2(2) + Val2(6)) + Val2(8)) ) *
( (Val2(8) + Val2(4) + Val2(3)) * (Val2(4) + Val2(3) + Val2(8) * (Val2(5) + Val2(9)) + Val2(3) + Val2(9)) * Val2(9) * (Val2(4) + Val2(2)) * Val2(9) * Val2(4) ) *
( Val2(2) + Val2(5) * (Val2(9) * (Val2(5) * Val2(2) + Val2(2) + Val2(7) + Val2(3) + Val2(9))) ) *
( (Val2(4) + Val2(2) * Val2(8)) * (Val2(4) * Val2(5) * Val2(3) * Val2(9) * (Val2(9) + Val2(2) + Val2(4)) + (Val2(2) + Val2(6) * Val2(6))) ) *
( Val2(9) + Val2(5) * Val2(2) + ((Val2(3) + Val2(3) * Val2(3) * Val2(9) + Val2(7) * Val2(9)) + Val2(3) + Val2(3) * Val2(6)) * Val2(7) ) *
( Val2(8) * (Val2(5) * Val2(4) * Val2(9)) * ((Val2(6) * Val2(9) + Val2(6)) + Val2(8) * Val2(3) * Val2(6) + Val2(8) + Val2(6)) + ((Val2(6) * Val2(4)) + (Val2(3) * Val2(5) + Val2(7) + Val2(9) + Val2(4) * Val2(3)) * (Val2(2) + Val2(4))) * Val2(4) ) *
( Val2(5) * Val2(7) * (Val2(5) * Val2(2) * (Val2(5) * Val2(6) + Val2(2)) + Val2(7)) * Val2(4) + Val2(4) ) *
( (Val2(7) + Val2(8) + Val2(8)) + (Val2(3) + (Val2(2) + Val2(3) + Val2(4) * Val2(3) * Val2(5) * Val2(6)) * Val2(2) * Val2(9) * (Val2(7) * Val2(5) * Val2(7) + Val2(9)) * Val2(9)) * Val2(2) ) *
( Val2(2) + Val2(8) * ((Val2(4) * Val2(9) * Val2(6) * Val2(8) + Val2(2) + Val2(9)) + Val2(9) + (Val2(5) + Val2(7) * Val2(7) + Val2(4) * Val2(3) * Val2(9)) + Val2(3) * Val2(2) + Val2(8)) + (Val2(3) * Val2(7) + Val2(7) + Val2(5) + Val2(3)) + Val2(9) + Val2(8) ) *
( Val2(6) + Val2(8) + (Val2(5) + Val2(7) * Val2(3) + Val2(2) + Val2(3) * Val2(4)) * Val2(7) * (Val2(4) * Val2(2) + Val2(8) + Val2(8)) * ((Val2(9) + Val2(4) * Val2(5)) + (Val2(7) * Val2(6) * Val2(2) + Val2(4))) ) *
( Val2(4) * Val2(9) * (Val2(9) * Val2(5) * Val2(3) + Val2(8)) + Val2(5) + Val2(9) ) *
( Val2(5) * Val2(2) * ((Val2(6) * Val2(8) * Val2(3) + Val2(7)) + Val2(6) + Val2(5) * Val2(8) * Val2(3) + Val2(9)) * Val2(6) ) *
( Val2(3) * (Val2(7) * (Val2(3) + Val2(6) + Val2(7)) * Val2(9) + Val2(2) + Val2(2) * (Val2(9) + Val2(4))) + Val2(8) + Val2(8) + Val2(6) ) *
( ((Val2(5) * Val2(9) * Val2(8) * Val2(6)) * Val2(4) * (Val2(3) + Val2(4) * Val2(8)) + (Val2(2) * Val2(5) + Val2(2) + Val2(6))) * Val2(9) * Val2(8) + Val2(2) ) *
( Val2(8) + Val2(9) * Val2(2) ) *
( (Val2(3) + (Val2(3) + Val2(5) + Val2(2) + Val2(3) + Val2(3)) + Val2(3) + Val2(7)) + Val2(3) + Val2(6) + Val2(5) * Val2(8) ) *
( Val2(6) + Val2(8) * Val2(6) * Val2(9) + Val2(9) + (Val2(7) + Val2(2) + Val2(5) + Val2(3) + Val2(4)) ) *
( (Val2(8) * Val2(2) * Val2(4) + Val2(3) * (Val2(4) + Val2(6) * Val2(7) + Val2(2))) * (Val2(6) * Val2(8)) * Val2(8) ) *
( Val2(9) * Val2(4) ) *
( ((Val2(4) * Val2(8)) * Val2(8) + (Val2(4) + Val2(5) * Val2(8) * Val2(2) * Val2(8) + Val2(9)) * Val2(7) + (Val2(3) + Val2(7) + Val2(4) + Val2(8)) * Val2(4)) + Val2(6) + Val2(7) + (Val2(2) + Val2(3) * Val2(6)) ) *
( Val2(6) * (Val2(4) * Val2(5) + Val2(9)) + Val2(6) * Val2(9) ) *
( (Val2(4) + (Val2(5) + Val2(3) + Val2(3) + Val2(7) + Val2(2) * Val2(7)) * Val2(4)) * Val2(4) + Val2(8) + Val2(4) + Val2(3) ) *
( Val2(8) * (Val2(9) * Val2(8) * Val2(9)) + Val2(5) ) *
( Val2(9) * Val2(8) * Val2(8) * Val2(9) + (Val2(8) + Val2(3)) ) *
( (Val2(6) * Val2(8) + Val2(5) + (Val2(5) * Val2(7) + Val2(8) * Val2(4) * Val2(3)) + Val2(6) * Val2(4)) + (Val2(2) + Val2(4) + Val2(5)) * Val2(7) + Val2(8) * (Val2(9) + Val2(6)) ) *
( ((Val2(9) * Val2(2) * Val2(5) * Val2(5) * Val2(5)) * (Val2(9) * Val2(6) * Val2(7)) + (Val2(7) * Val2(5) * Val2(9) * Val2(5) + Val2(6))) + (Val2(4) * (Val2(7) * Val2(8) * Val2(9) * Val2(7)) + (Val2(7) * Val2(7) * Val2(6) + Val2(4) * Val2(2)) * Val2(5) * Val2(5) * Val2(6)) + Val2(3) ) *
( Val2(7) + (Val2(6) * Val2(8) + Val2(6) + (Val2(5) * Val2(6) * Val2(5) + Val2(4)) + (Val2(9) + Val2(2) + Val2(6) + Val2(4) * Val2(8))) + Val2(2) * Val2(9) ) *
( (Val2(7) + Val2(3) + Val2(3) + Val2(9) + Val2(4)) * Val2(9) + (Val2(9) + (Val2(8) + Val2(8) * Val2(2) + Val2(8) * Val2(3) * Val2(6)) + Val2(5) * (Val2(4) * Val2(3) + Val2(3) + Val2(4) * Val2(5) + Val2(8))) * Val2(9) + Val2(4) ) *
( (Val2(7) * Val2(3) * Val2(4)) + (Val2(2) * (Val2(4) + Val2(7) * Val2(5) + Val2(3) + Val2(7)) + Val2(4) + (Val2(3) * Val2(5)) * Val2(8) + (Val2(8) + Val2(9) * Val2(2) * Val2(7))) ) *
( Val2(7) * (Val2(8) + Val2(7) + Val2(2) * (Val2(7) * Val2(6) * Val2(8)) + Val2(3) + Val2(3)) * (Val2(4) + (Val2(7) + Val2(7) * Val2(4) + Val2(7) + Val2(4)) * Val2(7) + Val2(5) * (Val2(6) + Val2(6) * Val2(4))) ) *
( Val2(5) * Val2(3) * Val2(4) * (Val2(2) * Val2(4) + Val2(5) + Val2(2) * Val2(8)) ) *
( (Val2(3) * Val2(3) * Val2(2) * Val2(9) + Val2(4)) * Val2(7) * (Val2(2) + Val2(8) + Val2(7) + Val2(3) * (Val2(3) * Val2(9) * Val2(9) + Val2(2))) * Val2(7) + (Val2(9) + Val2(7)) * Val2(2) ) *
( Val2(7) + Val2(5) + (Val2(4) + Val2(8) + Val2(4) + (Val2(7) * Val2(6) + Val2(5) + Val2(5) * Val2(7) + Val2(9))) * Val2(9) ) *
( (Val2(2) + Val2(5) * Val2(2) * (Val2(6) * Val2(3) * Val2(4))) * Val2(2) * ((Val2(6) * Val2(8) * Val2(9)) + Val2(4)) * Val2(2) ) *
( Val2(8) + (Val2(2) + Val2(9) * Val2(3) * Val2(2)) * Val2(9) ) *
( (Val2(5) * (Val2(5) * Val2(9) + Val2(5) * Val2(5) + Val2(4) + Val2(9)) + Val2(7)) * (Val2(8) + Val2(8) + Val2(9) * Val2(8) + Val2(3)) * Val2(3) * Val2(6) + Val2(5) ) *
( Val2(8) * (Val2(7) + (Val2(7) * Val2(9)) * Val2(3)) ) *
( (Val2(7) * Val2(4) + Val2(5) * (Val2(8) * Val2(2) * Val2(7)) + Val2(7) * Val2(8)) * (Val2(9) + (Val2(9) + Val2(8) * Val2(7) * Val2(9) + Val2(4))) + (Val2(4) * Val2(9) + Val2(5)) * Val2(2) ) *
( (Val2(7) + Val2(5) * Val2(6) + Val2(5) + (Val2(2) + Val2(4) + Val2(3) * Val2(4) * Val2(7))) + Val2(5) + (Val2(7) + (Val2(7) * Val2(3) + Val2(4)) + Val2(9)) ) *
( Val2(4) + (Val2(6) * Val2(9) + Val2(3)) * Val2(9) * Val2(5) + Val2(9) ) *
( (Val2(9) * Val2(3) * Val2(8) + Val2(7) + Val2(2) + Val2(6)) + (Val2(8) * Val2(7) * Val2(2) * Val2(7) * Val2(6)) * (Val2(5) + Val2(6) + Val2(6) * Val2(3)) * Val2(2) * Val2(9) ) *
( Val2(5) + Val2(5) * Val2(6) * (Val2(3) + Val2(7) * Val2(6)) ) *
( Val2(2) + Val2(7) * (Val2(5) + (Val2(5) * Val2(8) * Val2(9) + Val2(5) * Val2(3)) * (Val2(3) + Val2(2)) + Val2(3) * Val2(7) + Val2(2)) * Val2(2) * Val2(5) * Val2(3) ) *
( (Val2(9) + Val2(5) + (Val2(4) + Val2(9) + Val2(6)) * Val2(3) + Val2(8)) + Val2(2) + Val2(8) + Val2(4) + ((Val2(3) + Val2(7) + Val2(9) * Val2(5)) * Val2(6) + Val2(7)) ) *
( Val2(5) + Val2(9) * ((Val2(9) * Val2(4) * Val2(5) + Val2(7)) * Val2(4) + Val2(3) * Val2(6) + (Val2(2) + Val2(2))) * Val2(7) * Val2(2) ) *
( Val2(2) + (Val2(6) * Val2(2)) + ((Val2(6) * Val2(3) + Val2(8) + Val2(3) * Val2(7)) + Val2(7) + Val2(5) * (Val2(9) * Val2(2) * Val2(3) + Val2(9) + Val2(8) + Val2(8)) + Val2(7)) * (Val2(9) + Val2(9) * Val2(8) + Val2(7) * (Val2(9) + Val2(3) + Val2(8)) * Val2(9)) ) *
( Val2(4) * Val2(2) + Val2(2) + Val2(5) * Val2(5) + Val2(4) ) *
( Val2(4) * Val2(4) + Val2(8) + Val2(7) * Val2(3) ) *
( Val2(9) * (Val2(3) * Val2(8) + (Val2(4) + Val2(2) * Val2(3) * Val2(4)) * Val2(5) * Val2(8)) + Val2(8) * Val2(4) + Val2(7) + (Val2(4) * Val2(3) + Val2(9) + Val2(4)) ) *
( Val2(6) * (Val2(3) * Val2(2) + Val2(2) * Val2(3) * Val2(7) + Val2(8)) + Val2(4) + Val2(8) * Val2(2) ) *
( (Val2(7) + Val2(4) * Val2(5) + Val2(2) + (Val2(5) + Val2(8) * Val2(3) * Val2(6) + Val2(2))) * (Val2(7) + Val2(2)) + Val2(9) + (Val2(7) * Val2(7) * Val2(5)) + (Val2(8) * Val2(2) + Val2(9)) + Val2(6) ) *
( Val2(3) * Val2(9) + (Val2(7) * Val2(2) + Val2(6) * Val2(2) * Val2(5)) + Val2(4) ) *
( Val2(9) * (Val2(8) + Val2(2) * (Val2(8) * Val2(8) * Val2(8))) + Val2(8) * Val2(7) ) *
( Val2(6) + Val2(7) + Val2(7) + Val2(4) ) *
( Val2(2) + (Val2(2) * (Val2(4) * Val2(7) + Val2(7) * Val2(6) + Val2(8))) * Val2(4) ) *
( (Val2(9) * Val2(8) * (Val2(5) * Val2(9) * Val2(4) * Val2(9) + Val2(6) * Val2(3))) + Val2(4) ) *
( Val2(6) * ((Val2(9) * Val2(2) * Val2(4) + Val2(2) * Val2(9)) + Val2(7) + Val2(3) * Val2(5) + Val2(6) + Val2(6)) * Val2(7) + Val2(4) + Val2(9) ) *
( (Val2(5) * (Val2(5) * Val2(9) * Val2(6) * Val2(7))) + Val2(5) * Val2(4) * Val2(6) ) *
( (Val2(5) + Val2(8) + Val2(6) * Val2(8) * Val2(7)) + Val2(2) * Val2(7) * Val2(8) * Val2(5) ) *
( (Val2(7) + Val2(5) * Val2(6) * Val2(5) + (Val2(6) * Val2(4) + Val2(9))) * Val2(4) + Val2(8) * Val2(3) ) *
( Val2(4) + Val2(9) + (Val2(8) + Val2(5) + Val2(3) + Val2(9) + (Val2(4) + Val2(2) + Val2(3) + Val2(4)) + Val2(3)) + ((Val2(5) * Val2(5) + Val2(3)) * Val2(9) + Val2(4) * (Val2(9) * Val2(6)) + Val2(2) + Val2(6)) ) *
( Val2(7) + ((Val2(3) * Val2(8) + Val2(3) + Val2(6) + Val2(8) + Val2(6)) * (Val2(8) + Val2(3) + Val2(7) * Val2(4) + Val2(2) + Val2(5)) + Val2(5) + Val2(8) + (Val2(7) * Val2(9) + Val2(4) * Val2(6) + Val2(2) + Val2(8)) + Val2(4)) * Val2(7) * Val2(7) ) *
( (Val2(7) + (Val2(7) * Val2(4) * Val2(4) + Val2(5) + Val2(5) * Val2(2)) + Val2(2) * Val2(5) + Val2(6)) * Val2(2) + Val2(5) * Val2(3) ) *
( (Val2(2) * Val2(7)) + Val2(6) + (Val2(3) * (Val2(9) + Val2(4) * Val2(8) + Val2(8) * Val2(5)) * (Val2(2) * Val2(7) + Val2(5) + Val2(5)) + Val2(5) * (Val2(3) + Val2(6)) + Val2(2)) * (Val2(8) * Val2(5) + Val2(9) * Val2(9) * Val2(8)) ) *
( Val2(6) + Val2(5) + Val2(3) * Val2(7) + (Val2(4) * Val2(2) * Val2(7) * Val2(9)) ) *
( (Val2(9) * Val2(6) * (Val2(2) * Val2(2) * Val2(5) * Val2(6) + Val2(5) + Val2(8))) * Val2(7) + Val2(2) ) *
( ((Val2(7) * Val2(7) + Val2(9)) * (Val2(5) * Val2(8) + Val2(2) * Val2(4) + Val2(7) + Val2(7)) * Val2(7) + Val2(3)) + (Val2(8) * Val2(2)) * Val2(7) ) *
( Val2(3) * Val2(6) + (Val2(8) + Val2(7) + Val2(3) + Val2(7)) * (Val2(5) * Val2(4) + Val2(3)) * Val2(6) ) *
( (Val2(7) * Val2(7)) * Val2(9) * (Val2(5) + (Val2(5) * Val2(9) + Val2(3) * Val2(3) + Val2(2) * Val2(3)) + Val2(4) + (Val2(8) + Val2(7)) + Val2(2) + (Val2(7) + Val2(9) + Val2(6) * Val2(4) + Val2(6) * Val2(2))) * Val2(6) ) *
( ((Val2(6) + Val2(2) + Val2(6) * Val2(3) + Val2(8)) * (Val2(9) * Val2(8) * Val2(9) * Val2(2) * Val2(3) * Val2(5)) + Val2(7) + Val2(9)) * Val2(8) * Val2(6) + Val2(3) * Val2(6) * Val2(7) ) *
( ((Val2(9) + Val2(8) * Val2(2)) + Val2(8) * Val2(6) + (Val2(4) * Val2(8) * Val2(4) + Val2(9) + Val2(9)) * Val2(9)) + Val2(7) + Val2(5) ) *
( Val2(8) * Val2(6) * (Val2(4) * (Val2(6) * Val2(6) * Val2(3) * Val2(9))) * Val2(3) + Val2(3) * Val2(4) ) *
( Val2(8) * (Val2(3) * Val2(5) + Val2(9) * Val2(3)) + Val2(9) * Val2(8) * Val2(8) * Val2(7) ) *
( Val2(5) * (Val2(3) + Val2(4) * (Val2(8) * Val2(4) * Val2(3) * Val2(4)) + Val2(8) * Val2(7)) ) *
( (Val2(9) + (Val2(6) + Val2(7) + Val2(9) * Val2(5)) + (Val2(7) * Val2(6) + Val2(2) + Val2(4) * Val2(2) * Val2(8))) * Val2(4) * Val2(8) ) *
( Val2(3) + Val2(5) + (Val2(9) * Val2(2) + (Val2(9) + Val2(6) + Val2(7) * Val2(9)) * Val2(2)) + Val2(7) * Val2(6) ) *
( Val2(4) * (Val2(3) + Val2(7)) ) *
( (Val2(3) * Val2(7) + Val2(9)) + Val2(6) + Val2(6) * Val2(3) * Val2(3) * (Val2(2) * (Val2(9) + Val2(4) * Val2(6) * Val2(7) * Val2(9))) ) *
( (Val2(4) + Val2(2) + (Val2(9) * Val2(5) + Val2(9) * Val2(5) + Val2(9) * Val2(9)) * Val2(8)) + Val2(7) + Val2(7) + Val2(3) * (Val2(9) + Val2(4) + Val2(7)) ) *
( Val2(3) + (Val2(4) + Val2(5)) * Val2(9) * ((Val2(7) + Val2(8) + Val2(4) + Val2(8)) * Val2(7) + Val2(3)) ) *
( Val2(8) * Val2(9) * Val2(4) + Val2(4) + ((Val2(5) * Val2(3) + Val2(6)) + Val2(5)) + Val2(6) ) *
( Val2(9) * Val2(8) * (Val2(9) + Val2(9) * Val2(4)) * Val2(5) ) *
( Val2(9) + ((Val2(8) + Val2(7) + Val2(2) * Val2(4) * Val2(8)) + Val2(9) * Val2(5) * (Val2(2) * Val2(4) * Val2(6) + Val2(7) + Val2(3) * Val2(3)) * (Val2(7) + Val2(9) + Val2(2) * Val2(7) + Val2(9)) + Val2(3)) * (Val2(6) + (Val2(9) * Val2(7) + Val2(3) * Val2(7) * Val2(6)) * Val2(6) * Val2(6)) ) *
( Val2(4) + ((Val2(5) + Val2(7) + Val2(9) + Val2(9) + Val2(4)) * Val2(2) * (Val2(5) + Val2(5)) + Val2(2) + Val2(7) + Val2(9)) + Val2(4) + Val2(7) * Val2(2) * Val2(9) ) *
( Val2(9) * (Val2(7) * Val2(5)) * Val2(4) * (Val2(3) * Val2(2) + Val2(4)) + Val2(8) * Val2(7) ) *
( (Val2(8) + Val2(7) * Val2(7) * Val2(6) * Val2(7) + Val2(6)) + Val2(5) * Val2(7) * Val2(5) ) *
( Val2(9) * Val2(5) + Val2(5) + (Val2(3) * Val2(8) + Val2(4) * Val2(8) * Val2(4) + Val2(5)) * Val2(3) * Val2(4) ) *
( Val2(4) + Val2(5) + Val2(6) * Val2(6) * (Val2(7) + Val2(7) * Val2(6) * Val2(4) * Val2(4) + Val2(3)) ) *
( (Val2(4) + Val2(2) * Val2(9)) * (Val2(4) + Val2(3) * Val2(9)) + Val2(9) * Val2(8) ) *
( Val2(9) + Val2(6) * ((Val2(7) * Val2(2) * Val2(6)) * (Val2(2) * Val2(2) + Val2(3))) + Val2(6) ) *
( ((Val2(5) * Val2(6) * Val2(6) + Val2(6)) * Val2(8) * (Val2(3) + Val2(8) + Val2(3) + Val2(3) + Val2(6)) + Val2(4)) + (Val2(2) * Val2(6) + Val2(7) + Val2(6) + (Val2(3) * Val2(5))) + Val2(6) + Val2(2) ) *
( (Val2(5) + Val2(3)) * (Val2(2) * Val2(9)) * Val2(2) ) *
( Val2(4) + Val2(8) * Val2(6) ) *
( Val2(2) * Val2(6) * Val2(2) * Val2(4) ) *
( Val2(5) + Val2(8) + (Val2(9) + Val2(2) + Val2(8)) * Val2(8) * Val2(9) * Val2(4) ) *
( Val2(5) + (Val2(4) * Val2(7) + (Val2(5) + Val2(7) + Val2(6) + Val2(5) + Val2(8)) * (Val2(6) * Val2(3) + Val2(6) * Val2(3) * Val2(7)) + (Val2(7) + Val2(5) + Val2(8)) + Val2(4)) ) *
( Val2(5) + Val2(9) * Val2(5) * Val2(5) + (Val2(9) + Val2(2) * Val2(4) * Val2(9) + Val2(7) * Val2(4)) + Val2(7) ) *
( Val2(5) * (Val2(4) * Val2(7) * (Val2(9) * Val2(7) * Val2(5)) * Val2(9) + Val2(6) + Val2(3)) + Val2(5) + Val2(3) * Val2(5) ) *
( (Val2(7) + Val2(8) + Val2(7)) * Val2(6) * Val2(2) + Val2(4) * Val2(6) * Val2(4) ) *
( (Val2(2) * (Val2(4) * Val2(4) * Val2(6)) * Val2(9) * Val2(5) + (Val2(2) + Val2(7) + Val2(3)) + Val2(2)) * Val2(7) * ((Val2(9) * Val2(5)) * (Val2(5) + Val2(3) * Val2(8) * Val2(5) + Val2(6) * Val2(8)) + Val2(2) * Val2(7)) + Val2(9) + Val2(3) ) *
( (Val2(2) + (Val2(9) + Val2(5)) * Val2(9) * Val2(4)) * Val2(8) + Val2(7) * Val2(8) * Val2(8) * Val2(8) ) *
( Val2(2) * Val2(3) + Val2(3) * (Val2(7) + (Val2(9) * Val2(5)) * (Val2(8) * Val2(6) * Val2(8) * Val2(2)) + Val2(7) + Val2(2) * (Val2(7) * Val2(8) * Val2(9) + Val2(3))) * Val2(5) * Val2(9) ) *
( Val2(5) * Val2(4) * (Val2(7) + Val2(3) + (Val2(2) + Val2(6) * Val2(2) + Val2(3)) * Val2(3)) + Val2(7) ) *
( Val2(9) * (Val2(3) * (Val2(7) + Val2(3)) * Val2(7) * Val2(3)) * Val2(5) * (Val2(7) + Val2(3) * Val2(7)) ) *
( Val2(7) * Val2(4) + Val2(2) * Val2(4) * (Val2(8) * Val2(3) * Val2(7)) + Val2(5) ) *
( Val2(9) + Val2(4) * (Val2(6) * Val2(9) * Val2(3)) ) *
( Val2(2) * Val2(3) * Val2(7) * Val2(7) * Val2(8) * ((Val2(4) + Val2(7) * Val2(9) * Val2(8) * Val2(6) + Val2(3)) + Val2(3) + Val2(7) + Val2(9) + Val2(6) + Val2(7)) ) *
( (Val2(7) + Val2(9) + Val2(4) + Val2(2)) * (Val2(3) * (Val2(5) * Val2(2) * Val2(4) * Val2(6) + Val2(8) + Val2(5)) * Val2(5) * Val2(5)) * Val2(4) ) *
( (Val2(6) + Val2(2) * Val2(8) * Val2(3)) + ((Val2(8) + Val2(3) + Val2(9) + Val2(2) + Val2(7)) + Val2(4) * Val2(9) * Val2(6)) + Val2(6) * Val2(8) ) *
( (Val2(3) + Val2(5) + Val2(8) + Val2(4) * (Val2(2) + Val2(5)) * Val2(2)) * Val2(5) + Val2(8) + Val2(5) + Val2(3) + Val2(4) ) *
( Val2(4) * (Val2(9) * Val2(7) + (Val2(6) * Val2(3) * Val2(9) * Val2(7))) ) *
( (Val2(9) + Val2(2) * Val2(3) * Val2(7)) + Val2(7) + (Val2(8) + Val2(2) + Val2(7)) + Val2(3) * Val2(4) ) *
( Val2(6) + (Val2(8) + Val2(2) + Val2(8) * Val2(8) * Val2(9) * (Val2(4) + Val2(7) + Val2(4) * Val2(9) * Val2(5) * Val2(5))) + Val2(4) ) *
( Val2(5) + (Val2(2) + Val2(4)) + Val2(6) ) *
( Val2(4) * Val2(4) + Val2(4) * Val2(6) * ((Val2(5) * Val2(3) * Val2(2) + Val2(2) * Val2(4)) + Val2(3) + Val2(2) + (Val2(7) * Val2(7) * Val2(8) * Val2(6) * Val2(3)) * Val2(2)) ) *
( Val2(7) + Val2(6) + Val2(8) * (Val2(9) + Val2(9) * Val2(3) * Val2(9) * Val2(3) + Val2(7)) * Val2(3) ) *
( Val2(8) + (Val2(2) + Val2(8) + (Val2(2) + Val2(4) * Val2(6) + Val2(4) + Val2(8))) + (Val2(9) + Val2(3) * Val2(8) + Val2(3) + Val2(8) + (Val2(6) * Val2(8) * Val2(6) + Val2(7) * Val2(3))) + Val2(6) + Val2(6) + Val2(7) ) *
( (Val2(3) * (Val2(7) + Val2(7) + Val2(4) * Val2(2) + Val2(4) * Val2(7))) * Val2(6) + Val2(5) * Val2(9) * Val2(4) ) *
( (Val2(2) * Val2(2) + Val2(3)) * (Val2(3) + Val2(3) * Val2(2) + Val2(3) + Val2(5) * Val2(9)) * Val2(8) + (Val2(8) * Val2(9) + (Val2(9) * Val2(9) * Val2(2) + Val2(9)) * Val2(9) * (Val2(6) * Val2(3) + Val2(7)) + Val2(5)) + Val2(4) ) *
( Val2(4) + Val2(8) + Val2(3) * Val2(5) + (Val2(6) + (Val2(5) + Val2(4) * Val2(9) * Val2(4) + Val2(6) + Val2(3))) * Val2(9) ) *
( Val2(4) + ((Val2(7) * Val2(4) + Val2(8)) + Val2(2)) + Val2(6) + Val2(6) * (Val2(7) + Val2(4)) + Val2(2) ) *
( Val2(7) * Val2(3) + Val2(3) * Val2(6) ) *
( Val2(2) + (Val2(3) * (Val2(6) * Val2(5)) * (Val2(3) + Val2(2) * Val2(6)) * Val2(4)) * Val2(6) + Val2(7) + (Val2(5) + Val2(2) * Val2(3) * Val2(9) + Val2(9)) ) *
( (Val2(7) + Val2(5) + (Val2(8) * Val2(4) * Val2(6) * Val2(3) + Val2(5))) * Val2(2) + Val2(9) ) *
( Val2(3) + Val2(4) + Val2(7) + (Val2(7) * (Val2(5) * Val2(7))) * Val2(9) ) *
( Val2(9) + ((Val2(9) * Val2(7)) * Val2(3)) ) *
( Val2(5) * ((Val2(6) + Val2(2) * Val2(8)) + Val2(7)) ) *
( Val2(7) * Val2(2) + (Val2(6) * Val2(6) * Val2(6) * Val2(3)) + Val2(2) ) *
( (Val2(9) * Val2(3) + (Val2(4) * Val2(8) * Val2(5) * Val2(5) * Val2(7) + Val2(9))) + ((Val2(7) + Val2(5) + Val2(9) + Val2(2) + Val2(6)) * Val2(5) * Val2(6) + Val2(3) * Val2(9)) + Val2(8) * Val2(7) + (Val2(5) * Val2(6)) ) *
( Val2(3) * ((Val2(8) * Val2(5) + Val2(9) + Val2(6) * Val2(6)) + Val2(2) + Val2(5) * Val2(9) + (Val2(8) + Val2(5) + Val2(8) * Val2(4) * Val2(9) + Val2(2)) + Val2(8)) + Val2(8) * Val2(7) ) *
( Val2(3) * Val2(2) + Val2(3) * ((Val2(9) * Val2(3)) + Val2(9)) ) *
( Val2(8) + (Val2(2) * (Val2(6) * Val2(2) + Val2(8))) + Val2(2) + Val2(7) * Val2(8) ) *
( (Val2(4) * Val2(9) + (Val2(8) * Val2(4) + Val2(2) * Val2(9) + Val2(6) * Val2(8)) + Val2(6) * (Val2(8) + Val2(9)) * Val2(3)) + Val2(6) ) *
( Val2(3) + (Val2(3) + Val2(9) + Val2(5) + Val2(6) * Val2(2) * Val2(4)) + Val2(9) + (Val2(8) + Val2(4) + Val2(7) + Val2(7) + Val2(8) + Val2(3)) + Val2(5) ) *
( Val2(8) + Val2(2) * (Val2(7) * Val2(4) * Val2(5) * Val2(2) + Val2(4) + Val2(8)) * (Val2(2) + Val2(4) * Val2(7) * Val2(4)) + Val2(4) ) *
( (Val2(7) * Val2(9) * Val2(9) + Val2(6)) + Val2(6) * Val2(9) + Val2(8) ) *
( Val2(5) * Val2(8) * Val2(3) + Val2(3) * (Val2(6) * (Val2(9) + Val2(8) * Val2(8) + Val2(5)) * (Val2(3) + Val2(5) + Val2(4))) ) *
( (Val2(2) + Val2(9) * Val2(3) + Val2(9) + Val2(8)) + (Val2(6) * Val2(6) + Val2(8)) + Val2(9) + Val2(4) ) *
( Val2(2) + (Val2(8) + Val2(9) * Val2(4)) ) *
( Val2(2) * Val2(5) + (Val2(5) + Val2(4) + Val2(9) + Val2(3) + Val2(8) + Val2(3)) + Val2(8) ) *
( Val2(9) + Val2(9) + (Val2(7) + Val2(3) + Val2(4)) + Val2(8) + Val2(8) ) *
( Val2(4) * (Val2(9) * Val2(8) * Val2(6) * Val2(8)) + Val2(9) * (Val2(6) * Val2(6) * Val2(9) + Val2(5)) * Val2(9) * (Val2(8) * Val2(4) + (Val2(5) * Val2(6) * Val2(3) + Val2(6) * Val2(6)) * Val2(5) + (Val2(8) + Val2(3) + Val2(8) + Val2(3) + Val2(7) + Val2(2))) ) *
( (Val2(5) + Val2(8) + Val2(7) + Val2(5) + Val2(2) * Val2(4)) + Val2(6) * Val2(8) + ((Val2(4) + Val2(5) + Val2(9) * Val2(3) * Val2(2)) + Val2(4) * Val2(4)) ) *
( Val2(2) + Val2(9) * (Val2(9) + Val2(7) + Val2(8) + Val2(6) + Val2(7) + Val2(9)) * Val2(4) * (Val2(7) * (Val2(3) * Val2(9) + Val2(5) * Val2(5) * Val2(2)) * (Val2(6) + Val2(3) * Val2(2) * Val2(7) + Val2(7))) + Val2(7) ) *
( Val2(7) + Val2(3) + Val2(7) * (Val2(2) + Val2(2) + Val2(4)) + Val2(5) ) *
( (Val2(2) * Val2(8) + Val2(6) + Val2(3) + (Val2(6) + Val2(5) * Val2(9) * Val2(3) + Val2(8) * Val2(8)) * (Val2(3) * Val2(6) * Val2(9) * Val2(4) + Val2(3) + Val2(6))) * Val2(4) * (Val2(3) + Val2(4) * Val2(2) * (Val2(5) + Val2(3) + Val2(7) * Val2(5) * Val2(9))) + ((Val2(2) * Val2(3) * Val2(6) * Val2(6) + Val2(4) + Val2(2)) * Val2(4) + Val2(9) * Val2(7) + (Val2(6) * Val2(2))) * (Val2(6) * (Val2(8) + Val2(7) * Val2(2) * Val2(2)) * Val2(8)) + (Val2(7) * (Val2(9) * Val2(3) * Val2(9))) ) *
( Val2(7) + Val2(4) + (Val2(7) * Val2(5) + Val2(3) * (Val2(9) + Val2(2) + Val2(5) + Val2(2)) * Val2(9) * Val2(2)) + (Val2(5) * Val2(4) + (Val2(8) * Val2(8) + Val2(6)) * Val2(2)) ) *
( Val2(7) + (Val2(9) + (Val2(7) * Val2(4) + Val2(3) * Val2(8) * Val2(3)) * Val2(8) * Val2(8)) * Val2(9) ) *
( ((Val2(8) * Val2(6) * Val2(4)) * (Val2(9) * Val2(6) + Val2(7) * Val2(4)) * (Val2(6) * Val2(9) + Val2(5) * Val2(4)) * Val2(3)) * Val2(5) * Val2(9) * (Val2(9) * Val2(9) + Val2(7)) + Val2(3) * Val2(2) ) *
( Val2(8) * Val2(4) * (Val2(9) + Val2(9) * Val2(2) + Val2(8) * Val2(4)) * Val2(5) + Val2(4) ) *
( Val2(9) + (Val2(3) + (Val2(7) * Val2(8) + Val2(7) + Val2(8)) * Val2(5) + Val2(7) + (Val2(5) + Val2(6) * Val2(7) + Val2(4) * Val2(6))) * Val2(9) ) *
( (Val2(8) * Val2(3) + Val2(6) * Val2(8) * Val2(8)) + Val2(5) + Val2(3) ) *
( ((Val2(5) * Val2(7) * Val2(4) * Val2(2) + Val2(6)) * (Val2(6) + Val2(4) * Val2(6) + Val2(3) * Val2(8)) + Val2(5) * Val2(8) * Val2(8) + (Val2(5) + Val2(6) * Val2(2))) * (Val2(2) + Val2(4) * Val2(9) + Val2(7) + Val2(6) + (Val2(9) + Val2(6) * Val2(6) + Val2(8))) + Val2(6) + Val2(9) * Val2(3) ) *
( ((Val2(3) + Val2(7)) + Val2(7) + Val2(2) + Val2(8) * Val2(2) + (Val2(2) * Val2(6) * Val2(7) + Val2(3) * Val2(2) + Val2(5))) * (Val2(6) + Val2(3)) + (Val2(4) + Val2(5) * Val2(4) * Val2(9)) * Val2(6) * (Val2(9) + Val2(7) + Val2(5) * Val2(5) + Val2(9)) * Val2(3) ) *
( Val2(6) * Val2(4) * Val2(5) * Val2(8) * (Val2(2) + (Val2(2) + Val2(2) * Val2(8)) + Val2(9) + Val2(7)) * Val2(6) ) *
( Val2(5) + Val2(4) * Val2(7) * (Val2(2) * Val2(6) + Val2(3) * (Val2(2) + Val2(2) + Val2(2))) ) *
( (Val2(8) + Val2(3) * (Val2(2) + Val2(6) + Val2(4) + Val2(2) + Val2(9)) * Val2(5) * Val2(9) + Val2(8)) + Val2(5) * Val2(7) ) *
( Val2(5) + Val2(5) + (Val2(3) * Val2(4) + Val2(3)) * ((Val2(7) + Val2(3) * Val2(5) * Val2(8)) * Val2(2)) * Val2(6) ) *
( ((Val2(9) + Val2(5)) * Val2(4)) + Val2(6) * Val2(4) + ((Val2(5) + Val2(2) + Val2(9)) * Val2(5) * Val2(3)) * Val2(6) * Val2(3) ) *
( Val2(9) + (Val2(2) * Val2(4) * Val2(4) + (Val2(5) + Val2(9) + Val2(3) * Val2(4) + Val2(5) * Val2(8)) + (Val2(3) + Val2(7)) + Val2(6)) * Val2(6) ) *
( (Val2(3) * Val2(2) + Val2(5) * Val2(5)) + (Val2(9) + Val2(3) * Val2(5) + Val2(7)) * Val2(3) + Val2(9) ) *
( Val2(6) + ((Val2(7) * Val2(9) * Val2(9) + Val2(2) + Val2(4)) + Val2(8)) * Val2(9) * (Val2(6) + Val2(8)) + Val2(3) ) *
( Val2(4) * Val2(9) + Val2(8) * (Val2(3) + Val2(4) + Val2(4) + Val2(7) + (Val2(7) + Val2(6) * Val2(7) + Val2(2) + Val2(7) * Val2(8))) ) *
( (Val2(7) * Val2(7)) * Val2(7) + ((Val2(7) * Val2(3) + Val2(4) * Val2(8) * Val2(8) * Val2(6)) * Val2(9) * Val2(2) + Val2(6) + Val2(9) * Val2(2)) ) *
( Val2(9) * Val2(7) * Val2(9) + Val2(9) * Val2(5) * Val2(5) ) *
( Val2(8) * Val2(2) * Val2(3) * Val2(2) * Val2(6) ) *
( Val2(8) * (Val2(4) + (Val2(3) + Val2(2)) * Val2(9) * Val2(6)) ) *
( Val2(9) + Val2(8) + Val2(2) * (Val2(6) * Val2(6) + Val2(9)) * Val2(8) + Val2(8) ) *
( Val2(7) * Val2(4) + Val2(2) + Val2(5) * Val2(4) * Val2(7) ) *
( Val2(5) + Val2(8) + Val2(4) + Val2(2) + (Val2(6) * Val2(5) * Val2(2)) ) *
( Val2(3) + Val2(5) + (Val2(3) * Val2(9) + Val2(2)) + (Val2(3) * Val2(5)) ) *
( Val2(3) + Val2(6) + Val2(7) + (Val2(2) * Val2(8) + (Val2(8) * Val2(6) * Val2(3) + Val2(9) * Val2(9) + Val2(5)) + Val2(3) * Val2(8) + (Val2(6) * Val2(3) + Val2(5) * Val2(8) * Val2(4))) + (Val2(9) * Val2(5) * Val2(9)) + Val2(5) ) *
( Val2(5) + Val2(3) ) *
( (Val2(3) + Val2(2) * Val2(7) * Val2(8)) + Val2(2) ) *
( ((Val2(7) * Val2(8) + Val2(4)) * Val2(6) * Val2(9) + (Val2(6) * Val2(5) * Val2(4) * Val2(9) * Val2(7) + Val2(6)) * Val2(9) * Val2(5)) * Val2(5) + Val2(4) ) *
( Val2(3) + Val2(9) + Val2(6) + Val2(4) + ((Val2(4) * Val2(8) * Val2(6)) * (Val2(4) * Val2(6) + Val2(2) * Val2(9) * Val2(7) + Val2(2)) * Val2(7) + Val2(2) * (Val2(5) * Val2(9) + Val2(2) * Val2(2))) ) *
( (Val2(8) * Val2(6) * Val2(5)) + Val2(4) + Val2(8) ) *
( Val2(7) + Val2(4) * Val2(8) * (Val2(4) * Val2(9) * Val2(3) * Val2(5) * Val2(2) * Val2(6)) * (Val2(7) * Val2(2) + Val2(5) + Val2(7)) * Val2(3) ) *
( Val2(6) * Val2(7) + Val2(4) * (Val2(8) + Val2(3) * Val2(8) + (Val2(7) + Val2(5) * Val2(3) + Val2(4) + Val2(5)) + Val2(9) + Val2(2)) + Val2(2) * Val2(3) ) *
( Val2(9) * Val2(6) + (Val2(6) + (Val2(4) * Val2(7))) * Val2(4) + Val2(7) ) *
( (Val2(4) * Val2(5) * Val2(8)) + Val2(8) * (Val2(8) + Val2(7) * Val2(9) + Val2(5)) + Val2(9) ) *
( Val2(5) + Val2(3) + (Val2(9) + Val2(9) * Val2(5) + Val2(6) + Val2(7) * Val2(6)) * Val2(7) * Val2(5) ) *
( Val2(7) * Val2(3) + Val2(7) + ((Val2(4) * Val2(6) + Val2(4) * Val2(8)) * Val2(5) * Val2(9) * Val2(7) + Val2(5)) ) *
( Val2(9) + (Val2(8) * Val2(9) * Val2(3) + Val2(5) + (Val2(4) * Val2(6) + Val2(4) * Val2(7) * Val2(7)) * (Val2(3) + Val2(8) + Val2(2) * Val2(2) * Val2(8))) + Val2(8) * Val2(8) + (Val2(3) + Val2(9)) * Val2(6) ) *
( Val2(2) * (Val2(7) + Val2(9) + Val2(3)) * (Val2(5) + Val2(8) + Val2(8) + (Val2(2) * Val2(2) + Val2(2) + Val2(6) + Val2(7) * Val2(6))) + (Val2(9) + Val2(6) * Val2(2)) ) *
( Val2(8) + (Val2(4) * (Val2(8) * Val2(3) + Val2(4) + Val2(2) + Val2(5)) + (Val2(8) + Val2(2)) * (Val2(2) * Val2(2) * Val2(7) + Val2(6) * Val2(3) + Val2(2)) * (Val2(2) + Val2(7))) + Val2(7) ) *
( Val2(3) + Val2(7) + Val2(3) * Val2(4) * (Val2(2) * (Val2(6) + Val2(3)) * (Val2(2) * Val2(5) * Val2(8) * Val2(3) + Val2(3)) * Val2(2) * (Val2(4) + Val2(7) * Val2(2))) + Val2(4) ) *
( (Val2(7) + Val2(7) * Val2(5) + Val2(8)) + Val2(3) * Val2(2) * Val2(9) * (Val2(2) + Val2(8) + Val2(4) * Val2(7) * Val2(4) + Val2(8)) + (Val2(7) * Val2(8) + Val2(5)) ) *
( Val2(2) + ((Val2(7) + Val2(5) + Val2(4)) + (Val2(4) * Val2(7) + Val2(9) * Val2(3) * Val2(3) + Val2(6)) * (Val2(5) * Val2(5) * Val2(9) * Val2(7) + Val2(8)) + (Val2(5) + Val2(2) + Val2(8) * Val2(4) + Val2(6)) * (Val2(5) * Val2(4) * Val2(3) + Val2(9) + Val2(4) * Val2(9))) + Val2(6) + Val2(8) + Val2(5) ) *
( (Val2(8) * Val2(2)) + Val2(3) * Val2(8) + ((Val2(5) + Val2(9) + Val2(7) + Val2(7)) + Val2(7)) * Val2(8) ) *
( (Val2(3) * Val2(7) * Val2(9) * Val2(7)) + Val2(9) + (Val2(2) * Val2(5) + Val2(8) + Val2(9)) * (Val2(5) + Val2(5) * Val2(9)) * (Val2(5) * (Val2(4) + Val2(4) + Val2(7) + Val2(9)) * Val2(4) + (Val2(2) + Val2(2) * Val2(5))) ) *
( Val2(3) * (Val2(4) * Val2(8) * Val2(3) * Val2(4) * Val2(7) * Val2(6)) * Val2(4) * Val2(3) + Val2(4) * ((Val2(5) + Val2(6)) * Val2(2) + Val2(5) + Val2(2) * Val2(8) + Val2(3)) ) *

Val2(0)
}
