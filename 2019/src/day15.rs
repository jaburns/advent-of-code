use crate::expanse::Expanse;
use crate::intcode::vm::IntCodeMachine;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapCell {
    TastyCrumb,
    DustyCrumb,
    OxygenSystem,
    DroidStart,
    Counted,
}

type ShipMap = Expanse<MapCell>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North = 1,
    South,
    West,
    East,
}

impl Direction {
    pub fn to_command(&self) -> i64 {
        *self as i64
    }

    pub fn move_point(&self, pos: (i32, i32)) -> (i32, i32) {
        let mut result = pos;
        match self {
            Direction::North => result.1 += 1,
            Direction::South => result.1 -= 1,
            Direction::West => result.0 += 1,
            Direction::East => result.0 -= 1,
        };
        result
    }

    pub fn relative_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn relative_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

const STATUS_DROID_MOVED: i64 = 1;
const STATUS_DROID_FOUND_OXYGEN: i64 = 2;

struct Droid {
    brain: IntCodeMachine,
    pos: (i32, i32),
    direction: Direction,
}

impl Droid {
    pub fn new(brain_tape: &[i64]) -> Droid {
        Droid {
            brain: IntCodeMachine::new(brain_tape),
            pos: (0, 0),
            direction: Direction::North,
        }
    }

    fn brain_step(&mut self, dir: Direction) -> i64 {
        self.brain.run_and_provide_input(dir.to_command()).unwrap();
        self.brain.run_and_get_output().unwrap()
    }

    fn leave_crumb_at_pos(&self, map: &mut ShipMap) {
        let crumb_kind = if let Some(MapCell::TastyCrumb) = map.read(self.pos.0, self.pos.1) {
            MapCell::DustyCrumb
        } else {
            MapCell::TastyCrumb
        };

        map.write(self.pos.0, self.pos.1, crumb_kind);
    }

    pub fn step_and_mark_map(&mut self, map: &mut ShipMap) -> bool {
        let right_dir = self.direction.relative_right();

        let step_result = self.brain_step(right_dir);

        if step_result == STATUS_DROID_MOVED || step_result == STATUS_DROID_FOUND_OXYGEN {
            self.direction = right_dir;
            self.leave_crumb_at_pos(map);
            self.pos = self.direction.move_point(self.pos);

            if step_result == STATUS_DROID_FOUND_OXYGEN {
                map.write(self.pos.0, self.pos.1, MapCell::OxygenSystem);
                return true;
            } else {
                return false;
            }
        }

        let step_result = self.brain_step(self.direction);

        if step_result == STATUS_DROID_MOVED || step_result == STATUS_DROID_FOUND_OXYGEN {
            self.leave_crumb_at_pos(map);
            self.pos = self.direction.move_point(self.pos);

            if step_result == STATUS_DROID_FOUND_OXYGEN {
                map.write(self.pos.0, self.pos.1, MapCell::OxygenSystem);
                return true;
            } else {
                return false;
            }
        }

        self.direction = self.direction.relative_left();

        false
    }
}

fn walk_crumby_map_and_count_steps(map: &mut ShipMap) -> u32 {
    let mut pos = (0i32, 0i32);
    let mut dir = Direction::West;
    let mut steps = 0u32;

    loop {
        let next_pos = dir.move_point(pos);

        if let None = map.read(next_pos.0, next_pos.1) {
            let next_pos = dir.relative_left().move_point(pos);
            if let Some(MapCell::TastyCrumb) = map.read(next_pos.0, next_pos.1) {
                dir = dir.relative_left();
                pos = next_pos;
            } else {
                dir = dir.relative_right();
                pos = dir.move_point(pos);
            }
        } else {
            pos = next_pos;
        }

        steps += 1;

        if let Some(MapCell::OxygenSystem) = map.read(pos.0, pos.1) {
            break;
        } else {
            map.write(pos.0, pos.1, MapCell::Counted);
        }
    }

    steps
}

fn find_oxygen_tank_and_steps(droid_tape: &[i64]) -> ((i32, i32), u32) {
    let mut droid = Droid::new(&droid_tape);
    let mut map = Expanse::<MapCell>::new();

    while !droid.step_and_mark_map(&mut map) {
        continue;
    }

    map.write(0, 0, MapCell::DroidStart);

    let tank_location = map.find(|&cell| cell == MapCell::OxygenSystem).unwrap();

    let step_count = walk_crumby_map_and_count_steps(&mut map);

    (tank_location, step_count)
}

fn oxygen_expand(map: &mut ShipMap) -> bool {
    let mut did_expand = false;

    for x in map.x_range() {
        for y in map.y_range() {
            if let Some(MapCell::TastyCrumb) = map.read(x, y) {
                let a = map.read(x + 1, y) == Some(&MapCell::Counted);
                let b = map.read(x - 1, y) == Some(&MapCell::Counted);
                let c = map.read(x, y + 1) == Some(&MapCell::Counted);
                let d = map.read(x, y - 1) == Some(&MapCell::Counted);

                if a || b || c || d {
                    map.write(x, y, MapCell::DustyCrumb);
                    did_expand = true;
                }
            }
        }
    }

    for x in map.x_range() {
        for y in map.y_range() {
            if map.read(x, y) == Some(&MapCell::DustyCrumb) {
                map.write(x, y, MapCell::Counted);
            }
        }
    }

    did_expand
}

fn fill_ship_with_oxygen(pos: (i32, i32), droid_tape: &[i64]) -> u32 {
    let mut droid = Droid::new(&droid_tape);
    let mut map = Expanse::<MapCell>::new();

    for _ in 0..10000 {
        droid.step_and_mark_map(&mut map);
    }

    for x in map.x_range() {
        for y in map.y_range() {
            if map.read(x, y) == Some(&MapCell::DustyCrumb) {
                map.write(x, y, MapCell::TastyCrumb);
            }
        }
    }

    map.write(pos.0, pos.1, MapCell::Counted);

    let mut minutes = 0;
    while oxygen_expand(&mut map) {
        minutes += 1;
    }

    minutes
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day15.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let (tank_pos, result0) = find_oxygen_tank_and_steps(&tape);
    let result1 = fill_ship_with_oxygen(tank_pos, &tape);

    println!("{} {}", result0, result1);
}
