#[cfg(test)]
extern crate test;

fn problem1() -> isize {
    let mut x: isize = 0;
    let mut a: usize = 0;
    let v: Vec<isize> = (0is..1001).collect();
    for _i in (0is..v.len() as isize) {
        if v[a] % 5 == 0 || v[a] % 3 == 0 { x += v[a]; }
        a += 1;
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
