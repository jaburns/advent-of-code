// 225 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    let mut m = 0;
    let mut n = 0;
    for l in std::fs::read_to_string("I").unwrap().lines() {
        if let [a, b, c, d] = *l
            .split([',', '-'])
            .flat_map(str::parse)
            .collect::<Vec<i32>>()
        {
            m += (a <= c && b >= d || a >= c && b <= d) as i32;
            n += (a <= d && b >= c) as i32
        }
    }
    dbg!(m, n);
}
