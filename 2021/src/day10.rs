use std::fmt::Write;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Chunk {
    Paren,
    Square,
    Brace,
    Angle,
}

enum ParseResult {
    Incomplete(u64),
    Corrupted(u64),
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut stack = Vec::with_capacity(255);
    let mut corrupted_score = 0_u64;
    let mut incomplete_scores = Vec::with_capacity(255);

    for line in lines.iter() {
        stack.clear();
        match parse_chunk_line(&mut stack, line) {
            ParseResult::Corrupted(score) => {
                corrupted_score += score;
            }
            ParseResult::Incomplete(score) => {
                incomplete_scores.push(score);
            }
        }
    }

    incomplete_scores.sort_unstable();

    write!(
        out,
        " {}  {}",
        corrupted_score,
        incomplete_scores[incomplete_scores.len() / 2]
    )
    .unwrap();
}

fn parse_chunk_line(stack: &mut Vec<Chunk>, line: &str) -> ParseResult {
    use Chunk::*;

    for ch in line.chars() {
        match ch {
            '(' => stack.push(Paren),
            '[' => stack.push(Square),
            '{' => stack.push(Brace),
            '<' => stack.push(Angle),
            ')' => {
                if stack.pop().unwrap() != Paren {
                    return ParseResult::Corrupted(3);
                }
            }
            ']' => {
                if stack.pop().unwrap() != Square {
                    return ParseResult::Corrupted(57);
                }
            }
            '}' => {
                if stack.pop().unwrap() != Brace {
                    return ParseResult::Corrupted(1197);
                }
            }
            '>' => {
                if stack.pop().unwrap() != Angle {
                    return ParseResult::Corrupted(25137);
                }
            }
            _ => panic!(),
        }
    }

    let mut score = 0;
    while let Some(ch) = stack.pop() {
        score *= 5;
        score += match ch {
            Paren => 1,
            Square => 2,
            Brace => 3,
            Angle => 4,
        };
    }

    ParseResult::Incomplete(score)
}
