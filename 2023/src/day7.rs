use arrayvec::ArrayVec;
use std::fmt::Write;

const HANDS_COUNT: usize = 1024;
const JOKER: u8 = 0;

pub fn parts_1_and_2(lines: &[&str], out: &mut String) {
    let mut hands = lines
        .iter()
        .map(|x| parse_and_score_hand(x))
        .collect::<ArrayVec<Hand, HANDS_COUNT>>();

    hands.sort_unstable_by_key(|hand| hand.score);

    let result_1: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .sum();

    for hand in hands.iter_mut() {
        let cards = score_to_cards_with_joker(hand.score);
        hand.score = score_hand::<true>(cards);
    }

    hands.sort_unstable_by_key(|hand| hand.score);

    let result_2: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .sum();

    write!(out, "{}  {}", result_1, result_2).unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    bid: u64,
    score: u64,
}

fn parse_and_score_hand(line: &str) -> Hand {
    let card_str = &line[0..5];
    let bid_str = &line[6..];

    let mut cards = [0u8; 5];
    for (ch, out) in card_str.chars().zip(cards.iter_mut()) {
        *out = match ch {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            x => x.to_digit(10).unwrap() as u8,
        }
    }

    let bid = bid_str.parse().unwrap();
    let score = score_hand::<false>(cards);

    Hand { score, bid }
}

fn score_to_cards_with_joker(score: u64) -> [u8; 5] {
    let bytes = score.to_be_bytes().map(|x| if x == 11 { JOKER } else { x });
    [bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
}

fn score_hand<const JOKERS: bool>(mut cards: [u8; 5]) -> u64 {
    let mut sets = [0_u32; 5];
    let mut score_bytes = [0_u8; 8];
    score_bytes[3..].copy_from_slice(&cards);

    cards.sort_unstable();

    let mut last_card = 255_u8;
    let mut run_length = 0_usize;
    let mut jokers = 0_usize;
    for card in cards {
        if JOKERS && card == JOKER {
            jokers += 1;
        } else if card == last_card {
            run_length += 1;
        } else {
            sets[run_length] += 1;
            run_length = 0;
            last_card = card;
        }
    }
    sets[run_length] += 1;

    if JOKERS && jokers > 0 {
        for i in (0..4).rev() {
            if sets[i] > 0 {
                sets[i] -= 1;
                sets[i + jokers.min(4)] += 1;
                break;
            }
        }
    }

    score_bytes[0] = if sets[4] > 0 {
        6
    } else if sets[3] > 0 {
        5
    } else if sets[2] == 1 && sets[1] == 1 {
        4
    } else if sets[2] > 0 {
        3
    } else if sets[1] > 1 {
        2
    } else if sets[1] > 0 {
        1
    } else {
        0
    };

    u64::from_be_bytes(score_bytes)
}
