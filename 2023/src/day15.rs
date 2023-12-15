use arrayvec::ArrayVec;
use std::{fmt::Write, mem::zeroed};

const MAX_BIN_DEPTH: usize = 8;

type FocalType = u8;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut result_0 = 0;
    let mut bins: [ArrayVec<(&str, FocalType), MAX_BIN_DEPTH>; 256] = unsafe { zeroed() };

    for chunk in lines[0].split(',') {
        result_0 += hash(chunk.as_bytes()) as u32;

        if chunk.ends_with('-') {
            let label = &chunk[0..(chunk.len() - 1)];
            let h = hash(label.as_bytes());
            let bin = bins.get_mut(h as usize).unwrap();

            bin.retain(|x| x.0 != label);
        } else {
            let (label, value) = chunk.split_once('=').unwrap();
            let value = value.parse::<FocalType>().unwrap();
            let h = hash(label.as_bytes());
            let bin = bins.get_mut(h as usize).unwrap();

            let mut replaced = false;
            for (l, v) in bin.iter_mut() {
                if *l == label {
                    *v = value;
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                bin.push((label, value));
            }
        }
    }

    let mut result_1 = 0;
    for (bin_idx, bin) in bins.iter().enumerate() {
        for (slot_idx, (_, value)) in bin.iter().enumerate() {
            result_1 += (1 + bin_idx) * (1 + slot_idx) * (*value as usize);
        }
    }

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

fn hash(bytes: &[u8]) -> u8 {
    let mut hash = 0_u8;
    for b in bytes {
        hash = hash.wrapping_add(*b);
        hash = hash.wrapping_mul(17);
    }
    hash
}
