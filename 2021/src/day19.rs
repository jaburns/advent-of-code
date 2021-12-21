#![allow(clippy::never_loop)]
use glam::{Affine3A, IVec3};
use hashbrown::{HashMap, HashSet};
use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let mut scanners = parse_scanner_data(lines);

    let mut pairwise_match_scores = vec![];
    let mut pairwise_seen = HashSet::new();
    for (i, vi) in scanners.iter() {
        for (j, vj) in scanners.iter() {
            if j == i || pairwise_seen.contains(&(*i, *j)) {
                continue;
            }
            pairwise_seen.insert((*i, *j));
            pairwise_seen.insert((*j, *i));
            pairwise_match_scores.push((*i, *j, vi.match_score(vj)));
        }
    }

    let first_scanner = scanners.remove(&0).unwrap();
    let mut placed_scanners = HashMap::new();
    placed_scanners.insert(0_u8, first_scanner.place_original());

    while !scanners.is_empty() {
        pairwise_match_scores.sort_by(|(i0, j0, score0), (i1, j1, score1)| {
            let has_any_0 = placed_scanners.contains_key(i0) || placed_scanners.contains_key(j0);
            let has_any_1 = placed_scanners.contains_key(i1) || placed_scanners.contains_key(j1);

            if has_any_0 && !has_any_1 {
                std::cmp::Ordering::Less
            } else if has_any_1 && !has_any_0 {
                std::cmp::Ordering::Greater
            } else {
                score1.cmp(score0)
            }
        });

        let (i, j, _) = pairwise_match_scores[0];

        let (next_id, next_placement, relative_scanner) = if placed_scanners.contains_key(&i) {
            println!("Placing {} relative to {}", j, i);
            (
                j,
                scanners.remove(&j).unwrap(),
                placed_scanners.get(&i).unwrap(),
            )
        } else {
            println!("Placing {} relative to {}", i, j);
            (
                i,
                scanners.remove(&i).unwrap(),
                placed_scanners.get(&j).unwrap(),
            )
        };

        let next_placed_scanner = next_placement.place_relative(relative_scanner);
        placed_scanners.insert(next_id, next_placed_scanner);

        pairwise_match_scores.retain(|(i, j, _)| {
            !placed_scanners.contains_key(i) || !placed_scanners.contains_key(j)
        });
    }

    let result = count_unique_beacon_positions(&placed_scanners);

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}

#[derive(Debug)]
struct ScannerData {
    relative_beacons: Vec<IVec3>,
    internal_distances: HashMap<i32, (usize, usize)>,
}

#[derive(Debug)]
struct PlacedScanner {
    placement: Affine3A,
    absolute_beacons: Vec<IVec3>,
    data: ScannerData,
}

fn parse_scanner_data(lines: &[&str]) -> HashMap<u8, ScannerData> {
    let mut result = HashMap::new();
    let mut relative_beacons = vec![];
    let mut id = 0_u8;

    for line in lines.iter().skip(1) {
        if line.starts_with("---") {
            let internal_distances = get_internal_distances_for_relative_beacons(&relative_beacons);
            result.insert(
                id,
                ScannerData {
                    relative_beacons,
                    internal_distances,
                },
            );
            relative_beacons = vec![];
            id += 1;
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',');
        relative_beacons.push(IVec3::new(
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        ));
    }

    let internal_distances = get_internal_distances_for_relative_beacons(&relative_beacons);
    result.insert(
        id,
        ScannerData {
            relative_beacons,
            internal_distances,
        },
    );

    result
}

fn get_internal_distances_for_relative_beacons(beacons: &[IVec3]) -> HashMap<i32, (usize, usize)> {
    let mut result = HashMap::new();

    for i in 1..beacons.len() {
        for j in 0..i {
            let delta = beacons[i] - beacons[j];
            let d2 = delta.dot(delta);
            assert!(!result.contains_key(&d2));
            result.insert(d2, (i, j));
        }
    }

    result
}

impl ScannerData {
    fn match_score(&self, other: &Self) -> u32 {
        let mut score = 0;
        for self_dist in self.internal_distances.keys() {
            for other_dist in other.internal_distances.keys() {
                score += (self_dist == other_dist) as u32;
            }
        }
        score
    }

    fn place_original(self) -> PlacedScanner {
        PlacedScanner {
            placement: Affine3A::IDENTITY,
            absolute_beacons: self.relative_beacons.clone(),
            data: self,
        }
    }

    fn place_relative(self, other: &PlacedScanner) -> PlacedScanner {
        todo!()
    }
}

fn count_unique_beacon_positions(scanners: &HashMap<u8, PlacedScanner>) -> usize {
    let mut points = HashSet::<IVec3>::new();

    for scan in scanners.values() {
        for pos in scan.absolute_beacons.iter() {
            points.insert(*pos);
        }
    }

    points.len()
}
