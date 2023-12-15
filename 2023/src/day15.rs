use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let input = lines[0];

    let mut result_0 = 0;
    for chunk in input.split(',') {
        result_0 += hash(chunk.as_bytes())
    }

    let result_1 = 0;

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn hash(bytes: &[u8]) -> u64 {
    let mut hash = 0_u8;
    for b in bytes {
        hash = hash.wrapping_add(*b);
        hash = hash.wrapping_mul(17);
    }
    hash as u64
}
