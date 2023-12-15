use arrayvec::ArrayVec;
use std::fmt::Write;

type Board = (usize, ArrayVec<u64, 64>); // (width, rows)

#[derive(Debug)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut vert_sum_0 = 0_usize;
    let mut hori_sum_0 = 0_usize;
    let mut vert_sum_1 = 0_usize;
    let mut hori_sum_1 = 0_usize;
    let mut lines_iter = lines.iter();
    let mut done = false;

    while !done {
        let mut board = Board::default();
        let mut line = lines_iter.next().unwrap();
        board.0 = line.len();

        loop {
            let mut bit = 1_u64;
            let mut line_val = 0_u64;
            for ch in line.chars() {
                if ch == '#' {
                    line_val |= bit;
                }
                bit <<= 1;
            }
            board.1.push(line_val);

            if let Some(next) = lines_iter.next() {
                if next.is_empty() {
                    break;
                } else {
                    line = next;
                }
            } else {
                done = true;
                break;
            }
        }

        match find_mirror(&board, 0) {
            Mirror::Vertical(x) => vert_sum_0 += x,
            Mirror::Horizontal(x) => hori_sum_0 += x,
        }
        match find_mirror(&board, 1) {
            Mirror::Vertical(x) => vert_sum_1 += x,
            Mirror::Horizontal(x) => hori_sum_1 += x,
        }
    }

    let result_0 = vert_sum_0 + 100 * hori_sum_0;
    let result_1 = vert_sum_1 + 100 * hori_sum_1;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn find_mirror(board: &Board, diff_count: u32) -> Mirror {
    if let Some(x) = find_horizontal_mirror(board, diff_count) {
        Mirror::Horizontal(x)
    } else {
        Mirror::Vertical(find_horizontal_mirror(&transpose(board), diff_count).unwrap())
    }
}

fn find_horizontal_mirror(board: &Board, diff_count: u32) -> Option<usize> {
    for i in 0..(board.1.len() - 1) {
        let mut diff_bits = (board.1[i] ^ board.1[i + 1]).count_ones();

        if diff_bits <= diff_count {
            let mut j = 1_usize;
            loop {
                if j > i || i + 1 + j >= board.1.len() {
                    break;
                }
                diff_bits += (board.1[i - j] ^ board.1[i + 1 + j]).count_ones();
                j += 1;
            }
            if diff_bits == diff_count {
                return Some(i + 1);
            }
        }
    }
    None
}

fn transpose((width, rows): &Board) -> Board {
    let new_width = rows.len();
    let mut new_rows = ArrayVec::new();

    for x in 0..*width {
        let read_bit = 1_u64 << x;
        let mut new_row = 0_u64;
        let mut write_bit = 1_u64;

        for row in rows.iter() {
            if row & read_bit != 0 {
                new_row |= write_bit;
            }
            write_bit <<= 1;
        }

        new_rows.push(new_row);
    }

    (new_width, new_rows)
}
