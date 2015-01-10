#[cfg(test)]
extern crate test;

fn problem1() -> isize {
    let mut x: isize = 0;
    let v: Vec<isize> = (0is..1000).collect();
    for i in (0..v.len()) {
        if v[i] % 5 == 0 || v[i] % 3 == 0 { x += v[i]; }
    }
    x
}

pub fn main() {
    println!("Problem1: {}", problem1());
}

#[test]
fn test1() {
    main();
    assert!(problem1() == 233168is);
}

#[bench]
fn bench1(b: &mut test::Bencher) {
    b.iter(|| problem1());
}
