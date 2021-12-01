use crate::expanse::Expanse;
use crate::intcode::vm::IntCodeMachine;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum PaintColor {
    Unpainted,
    Black,
    White,
}

fn color_to_int(c: PaintColor) -> i64 {
    match c {
        PaintColor::Unpainted => 0,
        PaintColor::Black => 0,
        PaintColor::White => 1,
    }
}

fn int_to_color(i: i64) -> PaintColor {
    match i {
        0 => PaintColor::Black,
        1 => PaintColor::White,
        _ => panic!(),
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum TurnCommand {
    TurnLeft,
    TurnRight,
}

fn int_to_turn_cmd(i: i64) -> TurnCommand {
    match i {
        0 => TurnCommand::TurnLeft,
        1 => TurnCommand::TurnRight,
        _ => panic!(),
    }
}

#[derive(Debug)]
struct PaintBot {
    grid: Expanse<PaintColor>,
    position: (i32, i32),
    direction: (i32, i32),
}

impl PaintBot {
    pub fn new() -> PaintBot {
        PaintBot {
            grid: Expanse::new(),
            position: (0, 0),
            direction: (0, 1),
        }
    }

    pub fn step(&mut self, paint: PaintColor, turn: TurnCommand) -> PaintColor {
        self.grid.write(self.position.0, self.position.1, paint);

        match turn {
            TurnCommand::TurnLeft => self.direction = (-self.direction.1, self.direction.0),
            TurnCommand::TurnRight => self.direction = (self.direction.1, -self.direction.0),
        };

        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        match self.grid.read(self.position.0, self.position.1) {
            Some(x) => *x,
            None => PaintColor::Unpainted,
        }
    }

    pub fn count_painted_tiles(&self) -> u32 {
        let mut result = 0u32;

        for x in self.grid.x_range() {
            for y in self.grid.y_range() {
                if self.grid.read(x, y).is_some() {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn render_image_to_string(&self) -> String {
        self.grid.render_to_string(true, " ", |color| {
            String::from(if color == &PaintColor::White {
                "X"
            } else {
                " "
            })
        })
    }
}

fn run_paint_bot(brain_tape: &[i64], start_color: PaintColor) -> PaintBot {
    let mut bot = PaintBot::new();
    let mut brain = IntCodeMachine::new(brain_tape);

    brain
        .run_and_provide_input(color_to_int(start_color))
        .unwrap();

    while let Ok(_) = (|| {
        let color_command = brain.run_and_get_output()?;
        let turn_command = brain.run_and_get_output()?;
        let new_color = bot.step(int_to_color(color_command), int_to_turn_cmd(turn_command));
        brain.run_and_provide_input(color_to_int(new_color))
    })() {}

    bot
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day11.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = run_paint_bot(&tape, PaintColor::Unpainted).count_painted_tiles();
    let result1 = run_paint_bot(&tape, PaintColor::White).render_image_to_string();

    println!("{}\n\n{}", result0, result1);
}
