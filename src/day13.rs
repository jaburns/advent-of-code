use modinverse::egcd;
use num::integer::{lcm, mod_floor};

type IndexedBus = (i64, i64);

fn find_earliest_available_bus_solution(departure_time: i64, buses: &[IndexedBus]) -> i64 {
    let (best_bus, wait_time) = buses
        .iter()
        .map(|(_, x)| (x, x - departure_time % x))
        .min_by(|(_, b0), (_, b1)| b0.cmp(b1))
        .unwrap();

    best_bus * wait_time
}

fn synchronize_steps(a: i64, b: i64, b_offset: i64) -> (i64, i64) {
    // https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset

    let (gcd, s, _) = egcd(a, b);
    let phase_diff = -mod_floor(-b_offset, b);
    let period = a / gcd * b;
    let phase = mod_floor(-s * phase_diff / gcd * a, period);

    (mod_floor(-phase, period), lcm(a, b))
}

fn find_earliest_time_departures_match_indices(buses: &[IndexedBus]) -> i64 {
    let (index0, bus0) = buses[0];
    let (mut offset, mut period) = (0i64, bus0);

    for &(index, bus) in &buses[1..] {
        let (offset1, period1) = synchronize_steps(period, bus, -(index - index0 + offset));
        offset += offset1;
        period = period1;
    }

    offset
}

pub fn main() {
    let lines = std::fs::read_to_string("data/day13.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let departure_time = lines[0].parse::<i64>().unwrap();

    let indexed_buses = lines[1]
        .split(",")
        .enumerate()
        .filter_map(|(i, x)| match x.parse::<i64>() {
            Ok(num) => Some((i as i64, num)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let part1 = find_earliest_available_bus_solution(departure_time, indexed_buses.as_slice());
    let part2 = find_earliest_time_departures_match_indices(indexed_buses.as_slice());

    println!("{} {}", part1, part2);
}
