use std::fmt::Write;

const LINES_SIZE: usize = 256;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let (colon, pipe) = (lines[0].find(':').unwrap(), lines[0].find('|').unwrap());
    let mut copies = [1_i32; LINES_SIZE];

    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut winners = [false; 100];

    for (i, line) in lines.iter().enumerate() {
        winners.fill(false);

        for n in line[(colon + 1)..pipe].split_whitespace() {
            let x = n.parse::<u32>().unwrap();
            winners[x as usize] = true;
        }

        let mut count = 0;
        for n in line[(pipe + 1)..].split_whitespace() {
            let x = n.parse::<u32>().unwrap();
            if winners[x as usize] {
                count += 1;
            }
        }

        result_1 += (1 << count) >> 1;

        let mul = copies[i];
        for cp in &mut copies[(i + 1)..(i + 1 + count)] {
            *cp += mul;
            result_2 += mul;
        }
    }

    write!(out, "{}  {}", result_1, result_2).unwrap();
}
