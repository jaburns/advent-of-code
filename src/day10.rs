fn count_deltas_in_sorted_list(delta: u32, nums: &[u32]) -> u32 {
    let mut result = 0u32;
    for i in 1..nums.len() {
        if nums[i] - nums[i - 1] == delta {
            result += 1;
        }
    }
    result
}

fn split_at_delta(delta: u32, nums: &[u32]) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for i in 0..(nums.len() - 1) {
        chunk.push(nums[i]);
        if nums[i + 1] - nums[i] == delta {
            result.push(chunk);
            chunk = Vec::new();
        }
    }

    chunk.push(*nums.last().unwrap());
    result.push(chunk);
    result
}

fn count_permutations(max_delta: u32, nums: &Vec<u32>) -> u64 {
    let mut perms = Vec::<Vec<u32>>::new();
    perms.push(nums.clone());

    fn recurse(max_delta: u32, nums: &Vec<u32>, perms: &mut Vec<Vec<u32>>) {
        for i in 1..(nums.len() - 1) {
            if nums[i + 1] - nums[i - 1] <= max_delta {
                let mut sub = nums.clone();
                sub.remove(i);
                recurse(max_delta, &sub, perms);
                perms.push(sub);
            }
        }
    }
    recurse(max_delta, nums, &mut perms);

    perms.sort();
    perms.dedup();

    perms.len() as u64
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

    adapters.insert(0, 0);

    let part2 = split_at_delta(3, &adapters)
        .iter()
        .map(|x| count_permutations(3, x))
        .product::<u64>();

    println!("{} {}", part1, part2);
}
