fn find_xmas_defect(preamble_len: usize, data: &[u64]) -> u64 {
    'top: for i in preamble_len..data.len() {
        let lo = i - preamble_len;
        let hi = i;

        for j in (lo + 1)..hi {
            for k in lo..j {
                if data[j] + data[k] == data[i] {
                    continue 'top;
                }
            }
        }

        return data[i];
    }
    panic!();
}

fn find_contiguous_sum_range(sum: u64, data: &[u64]) -> &[u64] {
    let mut lo = 0usize;
    let mut hi = 1usize;

    loop {
        let range = &data[lo..=hi];
        let test_sum = range.iter().sum::<u64>();

        if test_sum == sum {
            return range;
        } else if test_sum < sum {
            hi += 1;
        } else {
            lo += 1;
        }
    }
}

pub fn main() {
    let data: Vec<_> = std::fs::read_to_string("data/day9.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let part1 = find_xmas_defect(25, &data);

    let range = find_contiguous_sum_range(part1, &data);
    let min = range.iter().min().unwrap();
    let max = range.iter().max().unwrap();

    let part2 = min + max;

    println!("{} {}", part1, part2);
}
