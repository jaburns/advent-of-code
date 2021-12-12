use std::fmt::Write;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Chunk {
    Paren = 1,
    Square = 2,
    Brace = 3,
    Angle = 4,
}

enum ParseResult {
    Incomplete(Vec<Chunk>),
    Corrupted(u64),
}

pub fn part1(lines: &[&str], out: &mut String) {
    let result: u64 = lines
        .iter()
        .map(|x| {
            if let ParseResult::Corrupted(y) = parse_chunk_line(x) {
                y
            } else {
                0
            }
        })
        .sum();
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut stack_scores: Vec<u64> = lines
        .iter()
        .filter_map(|x| {
            if let ParseResult::Incomplete(y) = parse_chunk_line(x) {
                Some(score_remaining_stack(y))
            } else {
                None
            }
        })
        .collect();

    stack_scores.sort_unstable();

    let result = stack_scores[stack_scores.len() / 2];

    write!(out, "{}", result).unwrap();
}

fn parse_chunk_line(line: &str) -> ParseResult {
    use Chunk::*;

    let mut stack = vec![];

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

    assert!(!stack.is_empty());

    ParseResult::Incomplete(stack)
}

fn score_remaining_stack(mut stack: Vec<Chunk>) -> u64 {
    let mut score = 0;
    while let Some(ch) = stack.pop() {
        score *= 5;
        score += ch as u64;
    }
    score
}
