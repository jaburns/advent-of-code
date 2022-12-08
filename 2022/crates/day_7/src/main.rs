// 411 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    type U = u64;
    f(f(0, 0) - 40000000, U::MAX);
    fn f(q: U, mut x: U) -> U {
        let (mut s, mut v, mut d) = (0, vec![], 0);
        let mut g = |l: &str| {
            if &l[4..] == " .." {
                if q < 1 && s < 100001 { x += s }
                if q > 0 && s > q && s < x { x = s }
                s += v.pop().unwrap()
            } else if &l[2..3] == "c" {
                v.push(s);
                s = 0
            }
            if &l[0..1] != "d" && &l[0..1] != "$" {
                l.split_once(' ').map(|p| s += p.0.parse::<U>().unwrap());
            }
            v.len()
        };
        for l in std::fs::read_to_string("I").unwrap().lines() { d = g(l) }
        for _ in 1..d { g("$    .."); }
        dbg!(x);
        s
    }
}
