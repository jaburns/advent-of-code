pub fn main(raw_input: &str) -> (i64, i64) {
    let lines = raw_input.lines();
    let bit_count = lines.clone().next().unwrap().len();
    let line_count = lines.clone().count();

    let mut counts = vec![0u64; bit_count];

    for line in lines.clone() {
        for (i, b) in line.chars().enumerate() {
            counts[bit_count - 1 - i] += (b == '1') as u64;
        }
    }

    let gamma = counts
        .iter()
        .map(|x| *x > line_count as u64 / 2)
        .enumerate()
        .fold(0u64, |acc, (i, x)| acc | (x as u64) << i);

    let all_ones = 2u64.pow(bit_count as u32) - 1;

    let power_consumption = gamma * (!gamma & all_ones);

    (power_consumption as i64, 0)
}
