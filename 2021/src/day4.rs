use std::fmt::Write;

#[derive(Debug)]
struct Board<const W: usize, const H: usize> {
    last_stamp: u32,
    squares: [[(u32, bool); H]; W],
}

impl<const W: usize, const H: usize> Board<W, H> {
    fn new(lines: &[&str]) -> Self {
        let mut ret = Self {
            last_stamp: 0,
            squares: [[(0, false); H]; W],
        };

        for (y, line) in lines.iter().enumerate() {
            for (x, v) in line.split_whitespace().enumerate() {
                ret.squares[x][y].0 = v.parse().unwrap();
            }
        }

        ret
    }

    fn stamp(&mut self, num: u32) {
        self.last_stamp = num;
        for x in 0..W {
            for y in 0..H {
                if self.squares[x][y].0 == num {
                    self.squares[x][y].1 = true;
                    return;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        'outer0: for x in 0..W {
            for y in 0..H {
                if !self.squares[x][y].1 {
                    continue 'outer0;
                }
            }
            return true;
        }
        'outer1: for y in 0..H {
            for x in 0..W {
                if !self.squares[x][y].1 {
                    continue 'outer1;
                }
            }
            return true;
        }
        false
    }

    fn calc_score(&self) -> u32 {
        let mut sum = 0;

        for x in 0..W {
            for y in 0..H {
                if !self.squares[x][y].1 {
                    sum += self.squares[x][y].0;
                }
            }
        }

        sum * self.last_stamp
    }
}

fn parse_inputs_and_run_game(
    lines: &[&str],
    run_stamp: impl Fn(&mut Vec<Board<5, 5>>, u32) -> Option<u32>,
) -> u32 {
    let mut boards: Vec<Board<5, 5>> = Vec::with_capacity((lines.len() - 1) / 6);
    for i in (2..lines.len()).step_by(6) {
        boards.push(Board::new(&lines[i..(i + 5)]));
    }

    let feed = lines[0]
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap());

    for num in feed {
        if let Some(x) = run_stamp(&mut boards, num) {
            return x;
        }
    }

    panic!("Game never completed");
}

pub fn part1(lines: &[&str], out: &mut String) {
    let result = parse_inputs_and_run_game(lines, |boards, num| {
        for board in boards.iter_mut() {
            board.stamp(num);
            if board.has_bingo() {
                return Some(board.calc_score());
            }
        }
        None
    });

    write!(out, "{:#?}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = parse_inputs_and_run_game(lines, |boards, num| {
        if boards.len() > 1 {
            for board in boards.iter_mut() {
                board.stamp(num);
            }
            boards.retain(|x| !x.has_bingo());
        } else {
            boards[0].stamp(num);
            if boards[0].has_bingo() {
                return Some(boards[0].calc_score());
            }
        }
        None
    });

    write!(out, "{}", result).unwrap();
}
