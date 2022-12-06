// 146 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    for mut i in [4, 14] {
        std::fs::read("I").unwrap().windows(i).any(|w| {
            i += 1;
            let mut x = w.to_vec();
            x.sort();
            x.dedup();
            x.len() == w.len()
        });
        dbg!(i - 1);
    }
}
