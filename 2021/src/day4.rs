use std::fmt::Write;

mod board {
    #[derive(Debug)]
    pub struct Board<const W: usize, const H: usize> {
        last_stamp: u32,
        squares: [[(u32, bool); H]; W],
        lookup: [Option<(u8, u8)>; 100],
        bingo: bool,
    }

    impl<const W: usize, const H: usize> Board<W, H> {
        pub fn new(lines: &[&str]) -> Self {
            let mut ret = Self {
                last_stamp: 0,
                squares: [[(0, false); H]; W],
                lookup: [None; 100],
                bingo: false,
            };

            for (y, line) in lines.iter().enumerate() {
                for (x, v) in line.split_whitespace().enumerate() {
                    let val = v.parse().unwrap();
                    ret.squares[x][y].0 = val;
                    ret.lookup[val as usize] = Some((x as u8, y as u8));
                }
            }

            ret
        }

        pub fn stamp(&mut self, num: u32) {
            if let Some((x, y)) = self.lookup[num as usize] {
                self.last_stamp = num;
                self.squares[x as usize][y as usize].1 = true;

                // Check the stamp point row for a bingo
                let mut failed = false;
                for ix in 0..W {
                    if !self.squares[ix][y as usize].1 {
                        failed = true;
                        break;
                    }
                }
                if !failed {
                    self.bingo = true;
                    return;
                }

                // Check the stamp point column for a bingo
                let mut failed = false;
                for iy in 0..H {
                    if !self.squares[x as usize][iy].1 {
                        failed = true;
                        break;
                    }
                }
                if !failed {
                    self.bingo = true;
                }
            }
        }

        pub fn has_bingo(&self) -> bool {
            self.bingo
        }

        pub fn calc_score(&self) -> u32 {
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
}

use board::Board;

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

    write!(out, "{}", result).unwrap();
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
