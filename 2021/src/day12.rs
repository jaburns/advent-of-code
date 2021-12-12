use hashbrown::{HashMap, HashSet};
use std::fmt::Write;

pub fn part1(lines: &[&str], out: &mut String) {
    let result = find_paths(lines, false);
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = find_paths(lines, true);
    write!(out, "{}", result).unwrap();
}

fn find_paths(lines: &[&str], allow_second_visit: bool) -> u32 {
    let caves = CaveSystem::parse(lines);
    let mut walks = vec![CaveWalk::new(&caves, allow_second_visit)];
    let mut completed_paths = 0_u32;

    loop {
        let mut should_break = true;
        let mut walks_out = vec![];

        for walk in walks {
            if walk.step(&mut walks_out) {
                completed_paths += 1;
            } else {
                should_break = false;
            }
        }

        walks = walks_out;

        if should_break {
            break;
        }
    }

    completed_paths
}

#[derive(Debug)]
struct CaveSystem<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    big_caves: HashSet<&'a str>,
}

impl<'a> CaveSystem<'a> {
    fn parse(lines: &[&'a str]) -> Self {
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
struct CaveWalk<'a> {
    system: &'a CaveSystem<'a>,
    visited: HashMap<&'a str, u32>,
    allow_second_visit: bool,
    node: &'a str,
}

impl<'a> CaveWalk<'a> {
    fn new(system: &'a CaveSystem<'a>, allow_second_visit: bool) -> Self {
        Self {
            system,
            visited: HashMap::default(),
            allow_second_visit,
            node: "start",
        }
    }

    fn step(mut self, walks_out: &mut Vec<CaveWalk<'a>>) -> bool {
        if self.node == "end" {
            return true;
        }

        if !self.system.big_caves.contains(self.node) {
            *self.visited.entry(self.node).or_insert(0) += 1;
        }

        for &next_node in &self.system.edges[self.node] {
            if next_node == "start" {
                continue;
            }

            let visit_count = *self.visited.get(next_node).unwrap_or(&0);
            let max_visits = if self.allow_second_visit { 2 } else { 1 };

            if visit_count >= max_visits {
                continue;
            }

            walks_out.push(Self {
                system: self.system,
                visited: self.visited.clone(),
                allow_second_visit: self.allow_second_visit && visit_count == 0,
                node: next_node,
            });
        }

        false
    }
}
