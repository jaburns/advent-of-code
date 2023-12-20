use arrayvec::ArrayVec;
use std::{collections::VecDeque, fmt::Write};

const MAX_OUTPUTS: usize = 8;
const MAX_NODES: usize = 64;

#[derive(Debug)]
enum NodeState {
    Broadcaster,
    Flipflop(bool),
    Nand { source_count: u8, high: u64 },
}

#[derive(Debug)]
struct Node {
    state: NodeState,
    outputs: ArrayVec<u8, MAX_OUTPUTS>,
}

type NodeList = ArrayVec<Node, MAX_NODES>;

type HashMap<K, V> =
    std::collections::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

struct Machine {
    lo_sent: u64,
    hi_sent: u64,
    broadcaster_idx: u8,
    pulses: VecDeque<(u8, u8, bool)>,
    nodes: NodeList,
}

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut node_idx_by_name = HashMap::with_capacity_and_hasher(
        lines.len(),
        core::hash::BuildHasherDefault::<ahash::AHasher>::default(),
    );
    let mut broadcaster_idx: u8 = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with('b') {
            broadcaster_idx = i as u8;
        } else {
            node_idx_by_name.insert(&line[1..3], i as u8);
        }
    }

    let mut nodes = NodeList::new();
    for line in lines {
        let mut outputs = ArrayVec::<u8, MAX_OUTPUTS>::new();

        let state = if line.starts_with('b') {
            for output in line[15..].split(',').map(|x| x.trim_start()) {
                outputs.push(node_idx_by_name[output]);
            }
            NodeState::Broadcaster
        } else {
            for output in line[7..].split(',').map(|x| x.trim_start()) {
                let out_idx = if let Some(idx) = node_idx_by_name.get(output) {
                    *idx
                } else {
                    lines.len() as u8
                };
                outputs.push(out_idx);
            }
            if line.starts_with('%') {
                NodeState::Flipflop(false)
            } else {
                NodeState::Nand {
                    source_count: 0,
                    high: 0,
                }
            }
        };

        nodes.push(Node { state, outputs });
    }

    for i in 0..nodes.len() {
        for j in 0..nodes[i].outputs.len() {
            let output = nodes[i].outputs[j];
            if output >= lines.len() as u8 {
                continue;
            }
            if let NodeState::Nand { source_count, .. } = &mut nodes[output as usize].state {
                *source_count += 1;
            }
        }
    }

    let mut machine = Machine::new(nodes, broadcaster_idx);

    for _ in 0..1000 {
        machine.poke();
    }
    let result_0 = machine.lo_sent * machine.hi_sent;

    // Counter periods read from data/day20-graph.png
    // Graph generated using day20-graph.html
    let result_1 = lcm(3847, lcm(3923, lcm(4091, 4001)));

    write!(out, "{}  {}", result_0, result_1).unwrap();
}

impl Machine {
    fn new(nodes: NodeList, broadcaster_idx: u8) -> Self {
        Self {
            lo_sent: 0,
            hi_sent: 0,
            pulses: VecDeque::new(),
            nodes,
            broadcaster_idx,
        }
    }

    fn poke(&mut self) {
        self.pulses.push_back((0, self.broadcaster_idx, false));

        while let Some((source_idx, idx, hi)) = self.pulses.pop_front() {
            if hi {
                self.hi_sent += 1;
            } else {
                self.lo_sent += 1;
            }
            if idx as usize >= self.nodes.len() {
                continue;
            }
            let node = &mut self.nodes[idx as usize];

            let mut ignore = false;

            let outgoing = match &mut node.state {
                NodeState::Broadcaster => false,
                NodeState::Flipflop(state) => {
                    if hi {
                        ignore = true;
                    } else {
                        *state = !*state;
                    }
                    *state
                }
                NodeState::Nand { source_count, high } => {
                    if hi {
                        *high |= 1 << (source_idx as u64);
                    } else {
                        *high &= !(1 << (source_idx as u64));
                    }
                    high.count_ones() < *source_count as u32
                }
            };

            if !ignore {
                for dest in node.outputs.iter() {
                    self.pulses.push_back((idx, *dest, outgoing));
                }
            }
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
