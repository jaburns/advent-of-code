use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

fn run_combat_game(deck0: &VecDeque<u32>, deck1: &VecDeque<u32>) -> usize {
    let mut deck0 = deck0.clone();
    let mut deck1 = deck1.clone();

    while deck0.len() > 0 && deck1.len() > 0 {
        let c0 = deck0.pop_front().unwrap();
        let c1 = deck1.pop_front().unwrap();

        if c1 > c0 {
            deck1.push_back(c1);
            deck1.push_back(c0);
        } else {
            deck0.push_back(c0);
            deck0.push_back(c1);
        }
    }

    score_deck(if deck0.len() > 0 { &deck0 } else { &deck1 })
}

fn hash_decks(deck0: &VecDeque<u32>, deck1: &VecDeque<u32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    let mut hash_deck = deck0.clone();
    hash_deck.extend(&[99999]);
    hash_deck.extend(deck1);
    hash_deck.hash(&mut hasher);
    hasher.finish()
}

// returns (winner, score)
fn run_recursive_combat_game(deck0: &VecDeque<u32>, deck1: &VecDeque<u32>) -> (u8, usize) {
    let mut deck0 = deck0.clone();
    let mut deck1 = deck1.clone();
    let mut history = HashSet::<u64>::new();

    while deck0.len() > 0 && deck1.len() > 0 {
        let hash = hash_decks(&deck0, &deck1);
        if history.contains(&hash) {
            return (0, score_deck(&deck0));
        }
        history.insert(hash);

        let c0 = deck0.pop_front().unwrap();
        let c1 = deck1.pop_front().unwrap();

        let c1_wins = if c0 as usize <= deck0.len() && c1 as usize <= deck1.len() {
            1 == run_recursive_combat_game(
                &deck0
                    .clone()
                    .iter()
                    .take(c0 as usize)
                    .map(|x| *x)
                    .collect::<VecDeque<_>>(),
                &deck1
                    .clone()
                    .iter()
                    .take(c1 as usize)
                    .map(|x| *x)
                    .collect::<VecDeque<_>>(),
            )
            .0
        } else {
            c1 > c0
        };

        if c1_wins {
            deck1.push_back(c1);
            deck1.push_back(c0);
        } else {
            deck0.push_back(c0);
            deck0.push_back(c1);
        }
    }

    if deck0.len() > 0 {
        (0, score_deck(&deck0))
    } else {
        (1, score_deck(&deck1))
    }
}

fn score_deck(deck: &VecDeque<u32>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| (i + 1) * x as usize)
        .sum::<usize>()
}

pub fn main() {
    let decks = std::fs::read_to_string("data/day22.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = run_combat_game(&decks[0], &decks[1]);
    let part2 = run_recursive_combat_game(&decks[0], &decks[1]).1;

    println!("{} {}", part1, part2);
}
