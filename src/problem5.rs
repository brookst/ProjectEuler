#[cfg(test)]
extern crate test;

fn problem5() -> isize {
    let mut ph = 1is;
    let mut i = 20is;
    while ph==1 {
        if i % 11 == 0 && i % 12 == 0 && i % 13 == 0 && i % 14 == 0 && i % 15 == 0 && i % 16 == 0 && i % 17 == 0 && i % 18 == 0 && i % 19 == 0 {
            return i;
            ph = 2;
        }
        i += 20;
    }
    i
}

pub fn main() {
    println!("Problem5: {}", problem5());
}

#[test]
fn test5() {
    main();
    assert!(problem5() == 232792560is);
}

#[bench]
fn bench5(b: &mut test::Bencher) {
    b.iter(|| problem5());
}
