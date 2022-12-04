// 185 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    let mut v: Vec<i32> = std::fs::read_to_string("I")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().map(|y| -y.parse::<i32>().unwrap()).sum())
        .collect();
    v.sort();
    dbg!(dbg!(-v[0]) - v[1] - v[2]);
}
