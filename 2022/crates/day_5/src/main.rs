// 586 characters
#[allow(clippy::all, unused_must_use)]
#[rustfmt::skip]
fn main() {
    let f = std::fs::read_to_string("I").unwrap();
    let c = f.split_once("\n\n").unwrap();
    let h: Vec<_> = c.0.lines().rev().skip(1).collect();

    let mut s = vec![vec![]; h[0].len() / 4 + 2];
    for i in 1..s.len() {
        for l in &h {
            let j = 1 + (i - 1) * 4;
            let a = l.as_bytes();
            if j < a.len() && a[j] > 32 {
                s[i].push(a[j])
            }
        }
    }

    dbg!([(s.clone(), 1), (s, 0)].map(|(mut s, r)| {
        for l in c.1.lines().map(|x| x.split(' ')) {
            let a: Vec<usize> = l.flat_map(str::parse).collect();
            let l = s[a[1]].len();
            let p = s[a[1]].drain((l - a[0])..);
            let o: Vec<_> = if r == 1 {
                p.rev().collect()
            } else {
                p.collect()
            };
            s[a[2]].extend(o)
        }
        String::from_utf8(s.iter_mut().flat_map(Vec::pop).collect())
    }));
}
