use hashbrown::HashSet;
use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let result: usize = lines
        .iter()
        .map(|line| {
            let mut iter = line.split('|');
            iter.next();
            iter.next()
                .unwrap()
                .split(' ')
                .map(|x| x.trim().len())
                .filter(|&len| len == 2 || len == 4 || len == 3 || len == 7)
                .count()
        })
        .sum();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let mut sum = 0_u32;

    for line in lines {
        let mut halves = line.split('|').map(|x| x.trim());
        let notes = halves.next().unwrap();
        let output = halves.next().unwrap();

        let mapping = decode_digits_from_notes(notes);
        let digits = output.split(' ').map(|x| read_decoded_digit(&mapping, x));

        sum += digits
            .rev()
            .enumerate()
            .fold(0_u32, |acc, (i, digit)| acc + 10_u32.pow(i as u32) * digit);
    }

    write!(out, "{}", sum).unwrap();
}

fn decode_digits_from_notes(pattern_notes: &str) -> [HashSet<char>; 10] {
    let mut out_mapping = [
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
        HashSet::default(),
    ];

    let mut in_sets = pattern_notes
        .split(' ')
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();

    enum SegmentConstraint {
        Count(usize),
        SupersetOf(usize),
        SubsetOf(usize),
    }

    let mut take_digit = |digit: usize, constraints: &[SegmentConstraint]| {
        let idx = in_sets
            .iter()
            .position(|segs| {
                for cons in constraints.iter() {
                    match *cons {
                        SegmentConstraint::Count(c) => {
                            if segs.len() != c {
                                return false;
                            }
                        }
                        SegmentConstraint::SupersetOf(d) => {
                            for seg in out_mapping[d].iter() {
                                if !segs.contains(seg) {
                                    return false;
                                }
                            }
                        }
                        SegmentConstraint::SubsetOf(d) => {
                            for seg in segs.iter() {
                                if !out_mapping[d].contains(seg) {
                                    return false;
                                }
                            }
                        }
                    }
                }
                true
            })
            .unwrap();
        out_mapping[digit] = in_sets.remove(idx);
    };

    take_digit(1, &[SegmentConstraint::Count(2)]);
    take_digit(4, &[SegmentConstraint::Count(4)]);
    take_digit(7, &[SegmentConstraint::Count(3)]);
    take_digit(8, &[SegmentConstraint::Count(7)]);
    take_digit(
        3,
        &[
            SegmentConstraint::Count(5),
            SegmentConstraint::SupersetOf(1),
        ],
    );
    take_digit(
        9,
        &[
            SegmentConstraint::Count(6),
            SegmentConstraint::SupersetOf(4),
        ],
    );
    take_digit(
        0,
        &[
            SegmentConstraint::Count(6),
            SegmentConstraint::SupersetOf(7),
        ],
    );
    take_digit(6, &[SegmentConstraint::Count(6)]);
    take_digit(5, &[SegmentConstraint::SubsetOf(6)]);
    take_digit(2, &[]);

    out_mapping
}

fn read_decoded_digit(mapping: &[HashSet<char>; 10], segments: &str) -> u32 {
    let seg_chars: Vec<char> = segments.chars().collect();

    'mapping_loop: for (i, map) in mapping.iter().enumerate() {
        for chr in 'a'..='g' {
            if seg_chars.contains(&chr) && !map.contains(&chr)
                || !seg_chars.contains(&chr) && map.contains(&chr)
            {
                continue 'mapping_loop;
            }
        }
        return i as u32;
    }

    panic!("Couldn't decode digit '{}'", segments);
}
