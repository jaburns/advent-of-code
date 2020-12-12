#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    pub fn parse(s: &str) -> Self {
        let arg = s[1..].parse::<i32>().unwrap();
        match s.chars().nth(0).unwrap() {
            'N' => Self::North(arg),
            'S' => Self::South(arg),
            'E' => Self::East(arg),
            'W' => Self::West(arg),
            'L' => Self::Left(arg),
            'R' => Self::Right(arg),
            'F' => Self::Forward(arg),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Ship {
    waypoint_mode: bool,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Ship {
    pub fn new(waypoint_mode: bool, dx: i32, dy: i32) -> Self {
        Self {
            waypoint_mode: waypoint_mode,
            x: 0,
            y: 0,
            dx: dx,
            dy: dy,
        }
    }

    fn rotate(&mut self, degrees: i32) {
        let c = (degrees as f32 * std::f32::consts::PI / 180f32).cos() as i32;
        let s = (degrees as f32 * std::f32::consts::PI / 180f32).sin() as i32;

        let new_x = c * self.dx - s * self.dy;
        let new_y = s * self.dx + c * self.dy;

        self.dx = new_x;
        self.dy = new_y;
    }

    #[rustfmt::skip]
    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::North(v) => if self.waypoint_mode { self.dy += v } else { self.y += v },
            Action::South(v) => if self.waypoint_mode { self.dy -= v } else { self.y -= v },
            Action::East(v)  => if self.waypoint_mode { self.dx += v } else { self.x += v },
            Action::West(v)  => if self.waypoint_mode { self.dx -= v } else { self.x -= v },

            Action::Left(v)  => self.rotate(*v),
            Action::Right(v) => self.rotate(-*v),

            Action::Forward(v) => {
                self.x += v * self.dx;
                self.y += v * self.dy;
            }
        };
    }
}

fn get_manhattan_distance_to_action_results(
    actions: &[Action],
    waypoint_mode: bool,
    dx: i32,
    dy: i32,
) -> i32 {
    let mut ship = Ship::new(waypoint_mode, dx, dy);
    for action in actions {
        ship.apply_action(action);
    }
    ship.x.abs() + ship.y.abs()
}

pub fn main() {
    let actions = std::fs::read_to_string("data/day12.txt")
        .unwrap()
        .lines()
        .map(Action::parse)
        .collect::<Vec<_>>();

    let part1 = get_manhattan_distance_to_action_results(&actions, false, 1, 0);
    let part2 = get_manhattan_distance_to_action_results(&actions, true, 10, 1);

    println!("{} {}", part1, part2);
}
