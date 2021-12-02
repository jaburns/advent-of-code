pub fn main(data: &str) -> (i64, i64) {
    let ints: Vec<i64> = data.lines().map(|x| x.parse().unwrap()).collect();

    let part_1 = ints.windows(2).filter(|win| win[1] > win[0]).count();

    let sums: Vec<i64> = ints.windows(3).map(|xs| xs.iter().sum()).collect();
    let part_2 = sums.windows(2).filter(|win| win[1] > win[0]).count();

    (part_1 as i64, part_2 as i64)
}
