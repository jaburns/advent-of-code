// 606 characters (part 1 only)
#[allow(clippy::all)]
fn main() {
    use {std::collections::HashMap as M, u64 as U};
    #[derive(Default)]
    struct T(M<String, T>, M<String, U>);

    unsafe {
        let mut t = T::default();
        let mut s = vec![(&mut t) as *mut T];

        for l in std::fs::read_to_string("I").unwrap().lines() {
            let i = s.len() - 1;
            if &l[3..] == "d .." {
                s.pop();
            } else if &l[0..3] == "$ c" {
                let c = (&mut *s[i]).0.entry(l[5..].into()).or_default();
                s.push(c as *mut _);
            } else if &l[0..1] != "d" && &l[2..3] != "l" {
                let (p, q) = l.split_once(' ').unwrap();
                (&mut *s[i]).1.insert(q.into(), p.parse().unwrap());
            }
        }

        static mut S: U = 0;
        fn f(t: &mut T) -> U {
            let z = t.1.values().sum::<U>() + t.0.values_mut().map(f).sum::<U>();
            if z <= 100000 {
                unsafe { S += z }
            }
            z
        }

        f(&mut t);
        dbg!(S);
    }
}
