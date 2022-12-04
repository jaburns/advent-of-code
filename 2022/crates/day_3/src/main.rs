// 318 characters
#[allow(clippy::all)]
#[rustfmt::skip]
fn main() {
    let s = std::fs::read("I").unwrap();
    let (mut m, v, f) = (
        0,
        s.split(|&x| x < 11).collect::<Vec<_>>(),
        |z: &[&[_]]| {
            let i = z[0]
                .iter()
                .filter(|x| z[1].contains(x))
                .fold(0, |a, x| a | x * z[2].contains(&x) as u8);
            i as u64 - 38 - (i > 96) as u64 * 58
        }
    );
    for x in &v {
        let y = x.split_at(x.len() / 2);
        m += f(&[y.0, y.0, y.1])
    }
    dbg!(m, v.chunks(3).map(f).sum::<u64>());
}
