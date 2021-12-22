use glam::{IVec3, Mat3, Vec3};
use hashbrown::{HashMap, HashSet};
use std::fmt::Write;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
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
            (
                j,
                scanners.remove(&j).unwrap(),
                placed_scanners.get(&i).unwrap(),
            )
        } else {
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

    let beacons = get_unique_beacon_positions(&placed_scanners);
    let scanner_positions: Vec<_> = placed_scanners.values().map(|x| x.position).collect();

    let result_1 = beacons.len();
    let result_2 = max_manhattan_between_pts(&scanner_positions);

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

#[derive(Debug)]
struct ScannerData {
    relative_beacons: Vec<IVec3>,
    internal_distances: HashMap<i32, (usize, usize)>,
}

#[derive(Debug)]
struct PlacedScanner {
    position: IVec3,
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
            // Assert every distance pair observed by a beacon is unique
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
            position: IVec3::ZERO,
            absolute_beacons: self.relative_beacons.clone(),
            data: self,
        }
    }

    fn place_relative(self, other: &PlacedScanner) -> PlacedScanner {
        let mut self_line_i: usize = 0;
        let mut self_line_j: usize = 0;
        let self_line_k: usize;
        let mut other_line_i: usize = 0;
        let mut other_line_j: usize = 0;
        let other_line_k: usize;

        'top: for (d0, (i0, j0)) in self.internal_distances.iter() {
            for (d1, (i1, j1)) in other.data.internal_distances.iter() {
                if d0 != d1 {
                    continue;
                }
                // We have a first edge in self and other with matching lengths

                for (d2, (i2, j2)) in self.internal_distances.iter() {
                    if i0 != i2 && i0 != j2 && j0 != j2 && j0 != i2 {
                        continue;
                    }
                    if i0 == i2 && j0 == j2 || i0 == j2 && j0 == i2 {
                        continue;
                    }
                    // We have a second edge in self sharing a node with first edge

                    for (d3, (i3, j3)) in other.data.internal_distances.iter() {
                        if i1 != i3 && i1 != j3 && j1 != j3 && j1 != i3 {
                            continue;
                        }
                        if i1 == i3 && j1 == j3 || i1 == j3 && j1 == i3 {
                            continue;
                        }
                        // We have a second edge in other sharing a node with the first edge

                        if d3 == d2 {
                            // We have a matching pair of segments connected by one node

                            if i0 == i2 {
                                self_line_i = *j0;
                                self_line_j = *i0;
                                self_line_k = *j2;
                            } else if i0 == j2 {
                                self_line_i = *j0;
                                self_line_j = *i0;
                                self_line_k = *i2;
                            } else if j0 == i2 {
                                self_line_i = *i0;
                                self_line_j = *j0;
                                self_line_k = *j2;
                            } else {
                                self_line_i = *i0; // j0 == j2
                                self_line_j = *j0;
                                self_line_k = *i2;
                            }

                            if i1 == i3 {
                                other_line_i = *j1;
                                other_line_j = *i1;
                                other_line_k = *j3;
                            } else if i1 == j3 {
                                other_line_i = *j1;
                                other_line_j = *i1;
                                other_line_k = *i3;
                            } else if j1 == i3 {
                                other_line_i = *i1;
                                other_line_j = *j1;
                                other_line_k = *j3;
                            } else {
                                other_line_i = *i1; // j1 == j3
                                other_line_j = *j1;
                                other_line_k = *i3;
                            }

                            // Sanity check
                            {
                                let da0 = self.relative_beacons[self_line_i]
                                    - self.relative_beacons[self_line_j];
                                let da0 = da0.dot(da0);
                                let da1 = self.relative_beacons[self_line_j]
                                    - self.relative_beacons[self_line_k];
                                let da1 = da1.dot(da1);

                                let db0 = other.data.relative_beacons[other_line_i]
                                    - other.data.relative_beacons[other_line_j];
                                let db0 = db0.dot(db0);
                                let db1 = other.data.relative_beacons[other_line_j]
                                    - other.data.relative_beacons[other_line_k];
                                let db1 = db1.dot(db1);

                                assert!(da0 == db0);
                                assert!(da1 == db1);
                            }

                            break 'top;
                        }
                    }
                }
            }
        }

        let base_va = other.absolute_beacons[other_line_i] - other.absolute_beacons[other_line_j];

        let incoming_rotated_va =
            self.relative_beacons[self_line_i] - self.relative_beacons[self_line_j];

        let rotation_to_global_space = find_cube_rotation_matrix(&incoming_rotated_va, &base_va);

        let scanner_global_pos = other.absolute_beacons[other_line_j]
            + rotation_to_global_space
                .mul_vec3(-self.relative_beacons[self_line_j].as_vec3())
                .as_ivec3();

        let absolute_beacons: Vec<_> = self
            .relative_beacons
            .iter()
            .map(|&rel| {
                rotation_to_global_space.mul_vec3(rel.as_vec3()).as_ivec3() + scanner_global_pos
            })
            .collect();

        PlacedScanner {
            position: scanner_global_pos,
            data: self,
            absolute_beacons,
        }
    }
}

fn find_cube_rotation_matrix(from: &IVec3, to: &IVec3) -> Mat3 {
    let xcol = if from.x.abs() == to.x.abs() {
        Vec3::X * (to.x * from.x).signum() as f32
    } else if from.x.abs() == to.y.abs() {
        Vec3::Y * (to.y * from.x).signum() as f32
    } else {
        // if from.x.abs() == to.z.abs() {
        Vec3::Z * (to.z * from.x).signum() as f32
    };

    let ycol = if from.y.abs() == to.x.abs() {
        Vec3::X * (to.x * from.y).signum() as f32
    } else if from.y.abs() == to.y.abs() {
        Vec3::Y * (to.y * from.y).signum() as f32
    } else {
        // if from.y.abs() == to.z.abs() {
        Vec3::Z * (to.z * from.y).signum() as f32
    };

    let zcol = if from.z.abs() == to.x.abs() {
        Vec3::X * (to.x * from.z).signum() as f32
    } else if from.z.abs() == to.y.abs() {
        Vec3::Y * (to.y * from.z).signum() as f32
    } else {
        // if from.z.abs() == to.z.abs() {
        Vec3::Z * (to.z * from.z).signum() as f32
    };

    Mat3::from_cols(xcol, ycol, zcol)
}

fn get_unique_beacon_positions(scanners: &HashMap<u8, PlacedScanner>) -> Vec<IVec3> {
    let mut points = HashSet::<IVec3>::new();

    for scan in scanners.values() {
        for pos in scan.absolute_beacons.iter() {
            points.insert(*pos);
        }
    }

    points.into_iter().collect()
}

fn manhattan(a: &IVec3, b: &IVec3) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn max_manhattan_between_pts(pts: &[IVec3]) -> i32 {
    let mut max = 0_i32;

    for i in 1..pts.len() {
        for j in 0..i {
            max = max.max(manhattan(&pts[i], &pts[j]));
        }
    }

    max
}
