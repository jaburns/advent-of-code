use std::fmt::Write;

const AREA_SIZE_X: usize = 10;
const AREA_SIZE_Y: usize = 10;
const AREA_SIZE_Z: usize = 310;

type Area = [[[u16; AREA_SIZE_Z]; AREA_SIZE_Y]; AREA_SIZE_X];

pub fn parts_1_and_2(_lines: &[&str], out: &mut String) {
    let result_0 = 0;
    let result_1 = 0;
    write!(out, "{}  {}", result_0, result_1).unwrap();
}
