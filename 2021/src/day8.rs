use hashbrown::HashSet;
use std::fmt::Write;

fn decode_digits_from_notes(pattern_notes: &str) -> [HashSet<char>; 10] {
    let mut mapping = [
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

    let mut sets = pattern_notes
        .split(' ')
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();

    fn find_remove<T>(vec: &mut Vec<T>, func: impl Fn(&T) -> bool) -> T {
        let idx = vec.iter().position(func).unwrap();
        vec.remove(idx)
    }

    mapping[1] = find_remove(&mut sets, |x| x.len() == 2);
    mapping[4] = find_remove(&mut sets, |x| x.len() == 4);
    mapping[7] = find_remove(&mut sets, |x| x.len() == 3);
    mapping[8] = find_remove(&mut sets, |x| x.len() == 7);

    mapping[3] = find_remove(&mut sets, |x| {
        if x.len() != 5 {
            return false;
        }
        for seg in mapping[1].iter() {
            if !x.contains(seg) {
                return false;
            }
        }
        true
    });
    mapping[9] = find_remove(&mut sets, |x| {
        if x.len() != 6 {
            return false;
        }
        for seg in mapping[4].iter() {
            if !x.contains(seg) {
                return false;
            }
        }
        true
    });
    mapping[0] = find_remove(&mut sets, |x| {
        if x.len() != 6 {
            return false;
        }
        for seg in mapping[7].iter() {
            if !x.contains(seg) {
                return false;
            }
        }
        true
    });
    mapping[6] = find_remove(&mut sets, |x| x.len() == 6);

    mapping[5] = find_remove(&mut sets, |x| {
        for seg in x {
            if !mapping[6].contains(seg) {
                return false;
            }
        }
        true
    });

    mapping[2] = find_remove(&mut sets, |_| true);

    mapping
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
        let mut digits = output.split(' ').map(|x| read_decoded_digit(&mapping, x));

        let (d0, d1, d2, d3) = (
            digits.next().unwrap(),
            digits.next().unwrap(),
            digits.next().unwrap(),
            digits.next().unwrap(),
        );

        sum += d3 + 10 * d2 + 100 * d1 + 1000 * d0;
    }

    write!(out, "{}", sum).unwrap();
}
