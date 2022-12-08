// 602 characters
#[allow(clippy::all)]
fn main() {
    use {std::collections::HashMap as M, u64 as U};
    struct T(M<String, T>, U);

    unsafe {
        let mut t = T(M::new(), 0);
        let mut s = vec![&mut t as *mut T];

        for l in std::fs::read_to_string("I").unwrap().lines() {
            let i = s.len() - 1;
            if &l[4..] == " .." {
                s.pop();
            } else if &l[2..3] == "c" {
                let c = (&mut *s[i])
                    .0
                    .entry(l[5..].into())
                    .or_insert(T(M::new(), 0));
                s.push(c as *mut _);
            }
            if &l[0..1] != "d" && &l[0..1] != "$" {
                l.split_once(' ')
                    .map(|p| (&mut *s[i]).1 += p.0.parse::<U>().unwrap());
            }
        }

        static mut S: (U, U, U) = (0, 40000000, U::MAX);

        fn f(t: &T) -> U {
            let z = t.1 + t.0.values().map(f).sum::<U>();
            unsafe {
                if z < 100001 {
                    S.0 += z
                }
                if z > S.2 && z < S.1 {
                    S.1 = z
                }
            }
            z
        }

        S.2 = f(&t) - S.1;
        f(&t);
        dbg!(S.0 / 2, S.1);
    }
}
