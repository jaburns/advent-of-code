// 412 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    f(f(0) - 40000000);
    fn f(q: u64) -> u64 {
        let (mut s, mut v, mut x, mut y, mut d) = (0, vec![], 0, u64::MAX, 0);
        let mut g = |l: &str| {
            if &l[4..] == " .." {
                if s < 100001 { x += s }
                if s >= q && s < y { y = s }
                s += v.pop().unwrap()
            } else if &l[2..3] == "c" {
                v.push(s);
                s = 0
            }
            if &l[0..1] != "d" && &l[0..1] != "$" {
                l.split_once(' ').map(|p| s += p.0.parse::<u64>().unwrap());
            }
            v.len()
        };
        for l in std::fs::read_to_string("I").unwrap().lines() { d = g(l) }
        for _ in 1..d { g("$    .."); }
        if q > 0 { dbg!(x, y); }
        s
    }
}
