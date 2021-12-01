pub fn main() {
    let data = std::fs::read_to_string("data/day1.txt").unwrap();
    let ints: Vec<i32> = data.lines().map(|x| x.parse().unwrap()).collect();

    let part_1 = ints.windows(2).filter(|win| win[1] > win[0]).count();

    let sums: Vec<i32> = ints.windows(3).map(|xs| xs.iter().sum()).collect();
    let part_2 = sums.windows(2).filter(|win| win[1] > win[0]).count();

    println!("\n{} {}\n", part_1, part_2);
}
