use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let feed = lines[0].split(',');

    let mut boards = vec![];

    let mut cur_board = [[(0u64, false); 5]; 5];
    let mut cur_board_line = 0;

    for line in lines.iter().skip(2) {
        cur_board_line += 1;

        if cur_board_line == 6 {
            boards.push(cur_board);
            cur_board = [[(0u64, false); 5]; 5];
            cur_board_line = 0;
            continue;
        }

        for (i, v) in line.split_whitespace().enumerate() {
            cur_board[cur_board_line - 1][i].0 = v.parse().unwrap();
        }
    }

    boards.push(cur_board);

    let result = boards;

    write!(out, "{:#?}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
