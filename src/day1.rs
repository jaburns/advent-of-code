fn product_of_2_nums_adding_to(sum: i32, nums: &[i32]) -> i32 {
    for i in 1..nums.len() {
        for j in 0..i {
            let a = nums[i];
            let b = nums[j];

            if a + b == sum {
                return a * b;
            }
        }
    }
    panic!();
}

fn product_of_3_nums_adding_to(sum: i32, nums: &[i32]) -> i32 {
    for i in 2..nums.len() {
        for j in 1..i {
            for k in 0..j {
                let a = nums[i];
                let b = nums[j];
                let c = nums[k];

                if a + b + c == sum {
                    return a * b * c;
                }
            }
        }
    }
    panic!();
}

pub fn main() {
    let entries: Vec<i32> = std::fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let part1 = product_of_2_nums_adding_to(2020, entries.as_slice());
    let part2 = product_of_3_nums_adding_to(2020, entries.as_slice());

    println!("{} {}", part1, part2);
}
