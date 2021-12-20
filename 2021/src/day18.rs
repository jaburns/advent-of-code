use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}  {}", result, 0).unwrap();
}
