use num::integer::lcm;
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, SubAssign};

fn lcm3(a: usize, b: usize, c: usize) -> usize {
    lcm(lcm(a, b), lcm(b, c))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0i32,
            y: 0i32,
            z: 0i32,
        }
    }

    pub fn component_value_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    pub fn new(serialized: &str) -> Self {
        let rx = Regex::new(r"[<xyz=> \t]+").unwrap();
        let csv = rx.replace_all(serialized, "");
        let vals: Vec<i32> = csv.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        Self {
            pos: Vec3 {
                x: vals[0],
                y: vals[1],
                z: vals[2],
            },
            vel: Vec3::new(),
        }
    }

    fn delta_v(a: i32, b: i32) -> i32 {
        if b > a {
            1
        } else if b == a {
            0
        } else {
            -1
        }
    }

    pub fn apply_gravity(first: &mut Self, second: &mut Self) {
        let dv = Vec3 {
            x: Self::delta_v(first.pos.x, second.pos.x),
            y: Self::delta_v(first.pos.y, second.pos.y),
            z: Self::delta_v(first.pos.z, second.pos.z),
        };

        first.vel += dv;
        second.vel -= dv;
    }

    pub fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }

    pub fn total_energy(&self) -> i32 {
        self.pos.component_value_sum() * self.vel.component_value_sum()
    }
}

fn get_total_energy_after_n_steps(moons_in: &[Moon], n: usize) -> i32 {
    let mut moons = Vec::from(moons_in);

    for _ in 0..n {
        for i in 0..(moons.len() - 1) {
            for j in (i + 1)..moons.len() {
                let chunks = moons.split_at_mut(j);
                Moon::apply_gravity(&mut chunks.0[i], &mut chunks.1[0]);
            }
        }

        for i in 0..moons.len() {
            moons[i].apply_velocity();
        }
    }

    moons.iter().map(|x| x.total_energy()).sum()
}

fn get_step_count_to_repeat_state(moons_in: &[Moon]) -> usize {
    let mut moons = Vec::from(moons_in);

    let mut x_steps = 0usize;
    let mut y_steps = 0usize;
    let mut z_steps = 0usize;

    let mut x_hash = HashSet::<[i32; 8]>::new();
    let mut y_hash = HashSet::<[i32; 8]>::new();
    let mut z_hash = HashSet::<[i32; 8]>::new();

    let mut x_looped = false;
    let mut y_looped = false;
    let mut z_looped = false;

    loop {
        for i in 0..(moons.len() - 1) {
            for j in (i + 1)..moons.len() {
                let chunks = moons.split_at_mut(j);
                Moon::apply_gravity(&mut chunks.0[i], &mut chunks.1[0]);
            }
        }

        for i in 0..moons.len() {
            moons[i].apply_velocity();
        }

        let hx = &[
            moons[0].pos.x,
            moons[1].pos.x,
            moons[2].pos.x,
            moons[3].pos.x,
            moons[0].vel.x,
            moons[1].vel.x,
            moons[2].vel.x,
            moons[3].vel.x,
        ];
        let hy = &[
            moons[0].pos.y,
            moons[1].pos.y,
            moons[2].pos.y,
            moons[3].pos.y,
            moons[0].vel.y,
            moons[1].vel.y,
            moons[2].vel.y,
            moons[3].vel.y,
        ];
        let hz = &[
            moons[0].pos.z,
            moons[1].pos.z,
            moons[2].pos.z,
            moons[3].pos.z,
            moons[0].vel.z,
            moons[1].vel.z,
            moons[2].vel.z,
            moons[3].vel.z,
        ];

        if !x_looped {
            if x_hash.contains(hx) {
                x_looped = true;
            } else {
                x_hash.insert(*hx);
                x_steps += 1;
            }
        }

        if !y_looped {
            if y_hash.contains(hy) {
                y_looped = true;
            } else {
                y_hash.insert(*hy);
                y_steps += 1;
            }
        }

        if !z_looped {
            if z_hash.contains(hz) {
                z_looped = true;
            } else {
                z_hash.insert(*hz);
                z_steps += 1;
            }
        }

        if x_looped && y_looped && z_looped {
            break;
        }
    }

    lcm3(x_steps, y_steps, z_steps)
}

pub fn main() {
    let moons: Vec<Moon> = std::fs::read_to_string("data/day12.txt")
        .unwrap()
        .lines()
        .map(|x| Moon::new(x))
        .collect();

    let result0 = get_total_energy_after_n_steps(&moons, 1000);
    let result1 = get_step_count_to_repeat_state(&moons);

    println!("{} {}", result0, result1);
}
