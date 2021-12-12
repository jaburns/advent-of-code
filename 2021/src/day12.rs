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
struct CaveWalk<'a> {
    system: Rc<CaveSystem<'a>>,
    blacklist: HashSet<&'a str>,
    node: &'a str,
}

impl<'a> CaveWalk<'a> {
    fn new(system: CaveSystem<'a>) -> Self {
        Self {
            system: Rc::new(system),
            blacklist: HashSet::default(),
            node: "start",
        }
    }

    fn step(mut self, out_walks: &mut Vec<CaveWalk<'a>>) -> bool {
        if self.node == "end" {
            return true;
        }

        if !self.system.big_caves.contains(self.node) {
            self.blacklist.insert(self.node);
        }

        for &next_node in &self.system.edges[self.node] {
            if self.blacklist.contains(next_node) {
                continue;
            }

            out_walks.push(Self {
                system: self.system.clone(),
                blacklist: self.blacklist.clone(),
                node: next_node,
            });
        }

        false
    }
}

pub fn part1(lines: &[&str], out: &mut String) {
    let caves = CaveSystem::parse(lines);

    let mut walks0 = vec![CaveWalk::new(caves)];
    let mut walks1: Vec<CaveWalk> = vec![];
    let mut total_paths = 0_u32;

    let mut buffer_flip = false;

    loop {
        buffer_flip = !buffer_flip;
        let (walks_from, walks_to) = if buffer_flip {
            (&mut walks0, &mut walks1)
        } else {
            (&mut walks1, &mut walks0)
        };

        let mut non_terminated_count = 0;

        for walk in walks_from.drain(0..walks_from.len()) {
            if walk.step(walks_to) {
                total_paths += 1;
            } else {
                non_terminated_count += 1;
            }
        }

        if non_terminated_count == 0 {
            break;
        }
    }

    let result = total_paths;

    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let result = lines.len();

    write!(out, "{}", result).unwrap();
}
