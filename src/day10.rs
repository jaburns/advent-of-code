fn count_deltas_in_sorted_list(delta: u32, nums: &[u32]) -> u32 {
    let mut result = 0u32;
    for i in 1..nums.len() {
        if nums[i] - nums[i-1] == delta {
            result += 1;
        }
    }
    result
}

pub fn main() {
    let mut adapters = std::fs::read_to_string("data/day10.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    adapters.sort();

    let by_one = count_deltas_in_sorted_list(1, &adapters) + 1;
    let by_three = count_deltas_in_sorted_list(3, &adapters) + 1;
    let part1 = by_one * by_three;

    println!("{:?}", part1);
}
