#[cfg(test)]
extern crate test;

use std::num::Float;

fn problem6() -> isize {
    let mut a = 0.0;
    let mut sqa = 0is;
    let mut nat = 0.0;
    for _ in range(0i, 101i) {
    sqa += a.powi(2) as isize;
    a += 1.0;
}
a = 0.0;
    for _ in range(0i, 100i) {
    a += 1.0;
    nat += a;
    }
    nat.powi(2) as isize - sqa
}

pub fn main() {
    println!("{}", problem6());
}

#[test]
fn test6() {
    main();
    assert!(problem6() == 25164150is);
}

#[bench]
fn bench6(b: &mut test::Bencher) {
    b.iter(|| problem6());
}
