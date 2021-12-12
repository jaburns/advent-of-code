use hashbrown::{HashMap, HashSet};
use std::{fmt::Write, rc::Rc};

#[derive(Debug)]
struct CaveSystem<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    big_caves: HashSet<&'a str>,
}

impl<'a> CaveSystem<'a> {
    fn parse(lines: &'_ [&'a str]) -> Self {
        let mut ret = CaveSystem {
            edges: HashMap::default(),
            big_caves: HashSet::default(),
        };

        let capitals = 'A'..='Z';

        for line in lines {
            let mut iter = line.split('-');
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();

            ret.edges.entry(a).or_insert(vec![]).push(b);
            ret.edges.entry(b).or_insert(vec![]).push(a);

            let cha = a.chars().next().unwrap();
            if capitals.contains(&cha) {
                ret.big_caves.insert(a);
            }

            let chb = b.chars().next().unwrap();
            if capitals.contains(&chb) {
                ret.big_caves.insert(b);
            }
        }

        ret
    }
}

#[derive(Debug)]
struct CaveWalk<'a, const MAX_VISITS: u32> {
    system: Rc<CaveSystem<'a>>,
    visited: HashMap<&'a str, u32>,
    breadcrumbs: Vec<&'a str>,
    node: &'a str,
}

impl<'a, const MAX_VISITS: u32> CaveWalk<'a, MAX_VISITS> {
    fn new(system: CaveSystem<'a>) -> Self {
        Self {
            system: Rc::new(system),
            visited: HashMap::default(),
            breadcrumbs: vec!["start"],
            node: "start",
        }
    }

    fn step(mut self, walks_out: &mut Vec<CaveWalk<'a, MAX_VISITS>>) -> bool {
        if self.node == "end" {
            println!("{:?}", self.breadcrumbs);
            return true;
        }

        if !self.system.big_caves.contains(self.node) {
            *self.visited.entry(self.node).or_insert(0) += 1;
        }

        for &next_node in &self.system.edges[self.node] {
            if next_node == "start" {
                continue;
            }

            if *self.visited.get(next_node).unwrap_or(&0) >= MAX_VISITS {
                continue;
            }

            let mut breadcrumbs = self.breadcrumbs.clone();
            breadcrumbs.push(next_node);

            walks_out.push(Self {
                system: self.system.clone(),
                visited: self.visited.clone(),
                breadcrumbs,
                node: next_node,
            });
        }

        false
    }
}

fn find_paths<const MAX_VISITS: u32>(lines: &[&str]) -> u32 {
    let caves = CaveSystem::parse(lines);

    let mut walks = vec![CaveWalk::<MAX_VISITS>::new(caves)];
    let mut total_paths = 0_u32;

    loop {
        let mut non_terminated_count = 0;
        let mut walks_out = vec![];

        for walk in walks {
            if walk.step(&mut walks_out) {
                total_paths += 1;
            } else {
                non_terminated_count += 1;
            }
        }

        walks = walks_out;

        if non_terminated_count == 0 {
            break;
        }
    }

    total_paths
}

pub fn part1(lines: &[&str], out: &mut String) {
    let result = find_paths::<1>(lines);
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = find_paths::<2>(lines);
    write!(out, "{}", result).unwrap();
}
