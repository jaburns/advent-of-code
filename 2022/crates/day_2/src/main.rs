fn main() {
    [
        |a, b| (b - a - 1) % 3 * 3 + b - 87,
        |a, b| (b + a + 2) % 3 + 3 * b - 7,
    ]
    .map(|f| {
        dbg!(std::fs::read("I")
            .unwrap()
            .chunks(4)
            .map(|s| f(s[0], s[2]) as i32)
            .sum::<i32>())
    });
}
