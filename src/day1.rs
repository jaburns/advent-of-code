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

fn product_of_nums_adding_to(sum: i32, num_count: usize, nums: &[i32]) -> i32 {
    let mut indices = (0..num_count).rev().collect::<Vec<usize>>();

    'top: loop {
        for i in (0..num_count).rev() {
            if i == 0 && indices[i] < nums.len() || indices[i] < indices[i - 1] {
                let sum_test = indices.iter().map(|&x| nums[x]).fold(0i32, |a,b| a + b);
                if sum_test == sum {
                    return indices.iter().map(|&x| nums[x]).fold(1i32, |a,b| a * b);
                }
                indices[i] += 1;
                continue 'top;
            } else if i == 0 {
                break 'top;
            } else {
                indices[i] = 0;
            }
        }
    }

    panic!()
}

pub fn main() {
    let entries: Vec<i32> = std::fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let part1 = product_of_2_nums_adding_to(2020, entries.as_slice());
    let part2 = product_of_3_nums_adding_to(2020, entries.as_slice());
    let part1x = product_of_nums_adding_to(2020, 2, entries.as_slice());
    let part2x = product_of_nums_adding_to(2020, 3, entries.as_slice());

    println!("{} {}", part1, part2);
    println!("{} {}", part1x, part2x);
}
