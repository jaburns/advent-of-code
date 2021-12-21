use glam::IVec3;
use hashbrown::{HashMap, HashSet};
use std::fmt::Write;

//static mut DISTANCES: Option<HashSet<i32>> = None;

pub fn part1(lines: &[&str], out: &mut String) {
    //unsafe {
    //    DISTANCES = Some(HashSet::new());
    //}

    let scanners = parse_scanner_data(lines);
    let fingerprints: Vec<ScannerFingerprint> = scanners
        .iter()
        .enumerate()
        .map(|(i, x)| fingerprint_scanner(i, x))
        .collect();

    //let xxx = unsafe { DISTANCES.as_ref().unwrap().len() };

    let mut overlaps = BeaconOverlaps::default();

    for i in 1..fingerprints.len() {
        for j in 0..i {
            find_overlaps_in_fingerprints(&mut overlaps, &fingerprints[i], &fingerprints[j]);
        }
    }

    let result = overlaps.0.len();

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}

#[derive(Debug)]
struct Scanner(Vec<IVec3>);

/// Map, tagged with scanner index, from beacon squared distance to pair of beacon indices
#[derive(Debug)]
struct ScannerFingerprint(usize, HashMap<i32, (usize, usize)>);

/// Collection of sets of (scanner index, beacon index) pairs, where each item in the set
/// represents the same beacon.
#[derive(Debug, Default)]
struct BeaconOverlaps(Vec<HashSet<(usize, usize)>>);

fn parse_scanner_data(lines: &[&str]) -> Vec<Scanner> {
    let mut cur_scanner = vec![];
    let mut result = vec![];

    for line in lines.iter().skip(1) {
        if line.starts_with("---") {
            result.push(Scanner(cur_scanner));
            cur_scanner = vec![];
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',');
        cur_scanner.push(IVec3::new(
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        ));
    }

    result
}

fn fingerprint_scanner(scanner_idx: usize, scanner: &Scanner) -> ScannerFingerprint {
    let mut result = HashMap::new();

    for i in 1..scanner.0.len() {
        for j in 0..i {
            let delta = scanner.0[i] - scanner.0[j];
            let d2 = delta.dot(delta);

            //unsafe {
            //    DISTANCES.as_mut().unwrap().insert(d2);
            //}

            assert!(!result.contains_key(&d2));
            result.insert(d2, (i, j));
        }
    }

    ScannerFingerprint(scanner_idx, result)
}

fn find_overlaps_in_fingerprints(
    overlaps: &mut BeaconOverlaps,
    a: &ScannerFingerprint,
    b: &ScannerFingerprint,
) {
    for (ad, (ai, aj)) in a.1.iter() {
        for (bd, (bi, bj)) in b.1.iter() {
            if ad == bd {
                insert_overlap(overlaps, (a.0, *ai), (b.0, *bi));
                insert_overlap(overlaps, (a.0, *aj), (b.0, *bj));
            }
        }
    }
    //
}

fn insert_overlap(
    overlaps: &mut BeaconOverlaps,
    scanner_beacon_0: (usize, usize),
    scanner_beacon_1: (usize, usize),
) {
    for set in overlaps.0.iter_mut() {
        if set.contains(&scanner_beacon_0) {
            set.insert(scanner_beacon_1);
            return;
        } else if set.contains(&scanner_beacon_1) {
            set.insert(scanner_beacon_0);
            return;
        }
    }

    let mut new_set = HashSet::new();
    new_set.insert(scanner_beacon_0);
    new_set.insert(scanner_beacon_1);
    overlaps.0.push(new_set);
}
