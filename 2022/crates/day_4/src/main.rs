// 256 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    let (mut m, mut n, mut v) = (0, 0, [0; 4]);
    for l in std::fs::read_to_string("I").unwrap().lines() {
        for (i, x) in l.replace(",", "-").split("-").enumerate() {
            v[i] = x.parse::<i32>().unwrap()
        }
        let [a, b, c, d] = v;
        m += (a <= c && b >= d || a >= c && b <= d) as i32;
        n += (a <= d && b >= c) as i32
    }
    dbg!(m, n);
}
