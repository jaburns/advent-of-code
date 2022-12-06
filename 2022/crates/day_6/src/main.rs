// 149 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    for j in [4, 14] {
        let mut i = j;
        for w in std::fs::read("I").unwrap().windows(i) {
            let mut x = w.to_vec();
            x.sort();
            x.dedup();
            if x.len() == j {
                dbg!(i);
                break
            }
            i += 1
        }
    }
}
