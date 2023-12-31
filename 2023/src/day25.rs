use std::{fmt::Write, mem::swap};

use arrayvec::ArrayVec;

// Determined by examining day25-graph.html
#[rustfmt::skip]
const BRIDGE_PAIRS: &[(&str, &str)] = &[
    ("mzb", "fjn"),
    ("zqg", "mhb"),
    ("jlt", "sjr")
];

const NODE_CAP: usize = 0x0800;
const EDGE_CAP: usize = 0x1000;

type HashSet<K> = std::collections::HashSet<K, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut edges = ArrayVec::<(&str, &str), EDGE_CAP>::new();

    for line in lines {
        let mut parts = line.split(": ");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        for node in b.split(' ') {
            edges.push((a, node));
        }
    }

    edges.retain(|(a, b)| {
        for (c, d) in BRIDGE_PAIRS {
            if a == c && b == d || a == d && b == c {
                return false;
            }
        }
        true
    });

    let count0 = count_nodes(&edges, BRIDGE_PAIRS[0].0);
    let count1 = count_nodes(&edges, BRIDGE_PAIRS[0].1);

    let result = count0 * count1;

    write!(out, "{}", result).unwrap();
}

fn count_nodes(edges: &[(&str, &str)], start: &str) -> u32 {
    let mut visited = HashSet::with_capacity_and_hasher(
        NODE_CAP,
        core::hash::BuildHasherDefault::<ahash::AHasher>::default(),
    );

    let mut frontier_buffer_0 = ArrayVec::<&str, NODE_CAP>::new();
    let mut frontier_buffer_1 = ArrayVec::<&str, NODE_CAP>::new();

    let mut curr_frontier = &mut frontier_buffer_0;
    let mut next_frontier = &mut frontier_buffer_1;

    curr_frontier.push(start);

    while !curr_frontier.is_empty() {
        for node in curr_frontier.drain(0..) {
            visited.insert(node);

            for (a, b) in edges {
                if *a == node {
                    if !visited.contains(b) {
                        next_frontier.push(*b);
                    }
                } else if *b == node && !visited.contains(a) {
                    next_frontier.push(*a);
                }
            }
        }

        swap(&mut curr_frontier, &mut next_frontier);
    }

    visited.len() as u32
}
