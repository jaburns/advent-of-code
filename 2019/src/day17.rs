use crate::expanse::Expanse;
use crate::intcode::vm::IntCodeMachine;
use std::cmp::min;
use std::string::ToString;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn move_point(&self, pos: (i32, i32)) -> (i32, i32) {
        let mut result = pos;
        match self {
            Direction::North => result.1 -= 1,
            Direction::South => result.1 += 1,
            Direction::West => result.0 -= 1,
            Direction::East => result.0 += 1,
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

type Scaffold = Expanse<()>;

fn load_scaffold_and_robot_pos_from_camera_view(camera_view: &str) -> (Scaffold, (i32, i32)) {
    let mut scaffold = Scaffold::new();
    let mut ix = 0i32;
    let mut iy = 0i32;

    let mut robot_pos = (0i32, 0i32);

    for c in camera_view.chars() {
        match c {
            '\n' => {
                iy += 1;
                ix = 0;
                continue;
            }
            '#' => scaffold.write(ix, iy, ()),
            '^' => robot_pos = (ix, iy),
            _ => (),
        };
        ix += 1;
    }

    (scaffold, robot_pos)
}

fn sum_scaffold_alignment_parameters(scaffold: &Scaffold) -> u32 {
    let mut sum = 0i32;

    for x in scaffold.x_range() {
        for y in scaffold.y_range() {
            if scaffold.read(x, y).is_some()
                && scaffold.read(x + 1, y).is_some()
                && scaffold.read(x - 1, y).is_some()
                && scaffold.read(x, y + 1).is_some()
                && scaffold.read(x, y - 1).is_some()
            {
                sum += x * y;
            }
        }
    }

    sum as u32
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PathTurn {
    Left,
    Right,
}

impl ToString for PathTurn {
    fn to_string(&self) -> String {
        match self {
            PathTurn::Left => String::from("L"),
            PathTurn::Right => String::from("R"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PathToken {
    Turn(PathTurn),
    Walk(u32),
}

impl ToString for PathToken {
    fn to_string(&self) -> String {
        match self {
            PathToken::Turn(x) => x.to_string(),
            PathToken::Walk(x) => x.to_string(),
        }
    }
}

impl PathToken {
    pub fn size(&self) -> u32 {
        match self {
            PathToken::Turn(_) => 1,
            PathToken::Walk(x) => {
                if *x >= 10 {
                    2
                } else {
                    1
                }
                // 3 digit numbers? YAGNI!
            }
        }
    }

    pub fn path_size(path: &[PathToken]) -> u32 {
        let tokens_size: u32 = path.iter().map(|x| x.size()).sum();
        let commas_size = path.len() as u32 - 1;
        tokens_size + commas_size
    }
}

fn get_raw_robot_path(scaffold: &Scaffold, robot_pos: (i32, i32)) -> Vec<PathToken> {
    let mut path = Vec::<PathToken>::new();
    let mut pos = robot_pos;
    let mut dir = Direction::West;
    let mut walk_count = 0u32;

    path.push(PathToken::Turn(PathTurn::Left));

    loop {
        let next_pos = dir.move_point(pos);
        if scaffold.read(next_pos.0, next_pos.1).is_some() {
            pos = next_pos;
            walk_count += 1;
        } else {
            let mut try_dir = |turned: Direction, path_turn: PathTurn| {
                let look = turned.move_point(pos);
                if scaffold.read(look.0, look.1).is_some() {
                    path.push(PathToken::Walk(walk_count));
                    path.push(PathToken::Turn(path_turn));
                    walk_count = 0;
                    Some(turned)
                } else {
                    None
                }
            };

            if let Some(new_dir) = try_dir(dir.relative_left(), PathTurn::Left) {
                dir = new_dir;
                continue;
            }

            if let Some(new_dir) = try_dir(dir.relative_right(), PathTurn::Right) {
                dir = new_dir;
                continue;
            }

            path.push(PathToken::Walk(walk_count));
            break;
        }
    }

    path
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SubRoutineCall {
    A,
    B,
    C,
}

impl ToString for SubRoutineCall {
    fn to_string(&self) -> String {
        match self {
            SubRoutineCall::A => String::from("A"),
            SubRoutineCall::B => String::from("B"),
            SubRoutineCall::C => String::from("C"),
        }
    }
}

#[derive(Debug)]
struct EncodedPath {
    pub main: Vec<SubRoutineCall>,
    pub sub_a: Vec<PathToken>,
    pub sub_b: Vec<PathToken>,
    pub sub_c: Vec<PathToken>,
}

fn join_to_string<T: ToString>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |a, x| {
            if a.len() > 0 {
                format!("{},{}", a, x)
            } else {
                x
            }
        })
}

fn encode_path(
    path: &[PathToken],
    sub_a: &[PathToken],
    sub_b: &[PathToken],
    sub_c: &[PathToken],
) -> Option<EncodedPath> {
    let mut chomped = Vec::from(path);
    let mut result = Vec::<SubRoutineCall>::new();

    loop {
        if chomped.len() == 0 {
            break;
        }

        let mut chomp = |sub: &[PathToken], call: SubRoutineCall| {
            if &chomped[0..sub.len()] == sub {
                for _ in 0..sub.len() {
                    chomped.remove(0);
                }
                result.push(call);
                true
            } else {
                false
            }
        };

        if chomp(sub_a, SubRoutineCall::A) {
            continue;
        }
        if chomp(sub_b, SubRoutineCall::B) {
            continue;
        }
        if chomp(sub_c, SubRoutineCall::C) {
            continue;
        }

        return None;
    }

    Some(EncodedPath {
        main: result,
        sub_a: Vec::from(sub_a),
        sub_b: Vec::from(sub_b),
        sub_c: Vec::from(sub_c),
    })
}

fn vacuum_and_report_dust(tape: &[i64], scaffold: &Scaffold, robot_pos: (i32, i32)) -> i64 {
    const MAX_SUBPATH_TOKEN_COUNT: usize = 10;

    let path = get_raw_robot_path(scaffold, robot_pos);

    let mut encoded_path: Option<EncodedPath> = None;

    'outer: for a_len in 1..min(path.len() - 2, MAX_SUBPATH_TOKEN_COUNT) {
        for b_len in 1..min(path.len() - a_len - 1, MAX_SUBPATH_TOKEN_COUNT) {
            for b_start in a_len..(path.len() - b_len - 1) {
                for c_len in 1..min(path.len() - b_len - a_len, MAX_SUBPATH_TOKEN_COUNT) {
                    for c_start in (b_start + b_len)..(path.len() - c_len) {
                        let a = &path[0..a_len];
                        if PathToken::path_size(a) > 20 {
                            continue;
                        };

                        let b = &path[b_start..(b_start + b_len)];
                        if PathToken::path_size(b) > 20 {
                            continue;
                        };

                        let c = &path[c_start..(c_start + c_len)];
                        if PathToken::path_size(c) > 20 {
                            continue;
                        };

                        encoded_path = encode_path(&path, a, b, c);

                        if encoded_path.is_some() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    let encoded_path = encoded_path.unwrap();

    let input_lines = vec![
        join_to_string(&encoded_path.main),
        join_to_string(&encoded_path.sub_a),
        join_to_string(&encoded_path.sub_b),
        join_to_string(&encoded_path.sub_c),
    ];

    let mut inputs = Vec::<i64>::new();

    for line in input_lines.iter() {
        for ch in line.chars() {
            inputs.push(ch as i64);
        }
        inputs.push(10);
    }

    inputs.push('n' as i64);
    inputs.push(10);

    let mut poked_tape = Vec::from(tape);
    poked_tape[0] = 2;
    let mut result = IntCodeMachine::run_all(&poked_tape, &inputs);

    result.pop().unwrap()
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day17.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let camera_view: String = IntCodeMachine::run_all(&tape, &[])
        .iter()
        .map(|x| *x as u8 as char)
        .collect();

    let (scaffold, robot_pos) = load_scaffold_and_robot_pos_from_camera_view(&camera_view);

    let result0 = sum_scaffold_alignment_parameters(&scaffold);
    let result1 = vacuum_and_report_dust(&tape, &scaffold, robot_pos);

    println!("{} {}", result0, result1);
}
