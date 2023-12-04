use smallvec::{smallvec, SmallVec};
use std::fmt::Write;

#[derive(Debug)]
struct Game {
    id: u32,
    samples: SmallVec<[Sample; 16]>,
}

#[derive(Copy, Clone, Debug, Default)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn parse((idx, line): (usize, &str)) -> Game {
        let mut samples = smallvec![];

        let after_color = &line[(line.find(':').unwrap() + 1)..];
        for sample in after_color.split(';').map(|x| x.trim()) {
            let parts = sample.split(',').map(|x| x.trim().split_once(' ').unwrap());

            let mut sample = Sample::default();
            for (count, color) in parts {
                let c = count.parse::<u32>().unwrap();
                match color.chars().next().unwrap() {
                    'r' => sample.red = c,
                    'g' => sample.green = c,
                    'b' => sample.blue = c,
                    _ => panic!(),
                }
            }
            samples.push(sample);
        }

        Game {
            id: idx as u32 + 1,
            samples,
        }
    }

    fn max(&self) -> Sample {
        let mut ret = Sample::default();
        for s in self.samples.iter() {
            ret.red = ret.red.max(s.red);
            ret.green = ret.green.max(s.green);
            ret.blue = ret.blue.max(s.blue);
        }
        ret
    }
}

pub fn part1(lines: &[&str], out: &mut String) {
    let mut result = 0_u32;
    for game in lines.iter().cloned().enumerate().map(Game::parse) {
        let sum = game.max();
        if sum.red <= 12 && sum.green <= 13 && sum.blue <= 14 {
            result += game.id;
        }
    }
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut result = 0_u32;
    for game in lines.iter().cloned().enumerate().map(Game::parse) {
        let sum = game.max();
        result += sum.red * sum.green * sum.blue;
    }
    write!(out, "{}", result).unwrap();
}
