use crate::intcode::vm::IntCodeMachine;

struct BeamDrone<'a>(&'a [i64]);

impl<'a> BeamDrone<'a> {
    pub fn query(&self, x: u32, y: u32) -> bool {
        let BeamDrone(tape) = self;
        IntCodeMachine::run_all(&tape, &[x as i64, y as i64])
            .pop()
            .unwrap()
            != 0
    }
}

fn count_beam_points_out_to_square(drone: &BeamDrone, square_bound: u32) -> u32 {
    let mut result = 0u32;

    for x in 0..square_bound {
        for y in 0..square_bound {
            if drone.query(x, y) {
                result += 1;
            }
        }
    }

    result
}

fn find_beam_location_supporting_square(drone: &BeamDrone, size: u32) -> (u32, u32) {
    let mut x = size - 1;
    let mut y = 0;

    while !drone.query(x, y) {
        y += 1;
    }

    while !drone.query(x + 1 - size, y + size - 1) {
        y += 1;
        while drone.query(x + 1, y) {
            x += 1;
        }
    }

    (x + 1 - size, y)
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let drone = BeamDrone(&tape);

    let result0 = count_beam_points_out_to_square(&drone, 50);
    let (x, y) = find_beam_location_supporting_square(&drone, 100);
    let result1 = 10_000 * x + y;

    println!("{} {}", result0, result1);
}
