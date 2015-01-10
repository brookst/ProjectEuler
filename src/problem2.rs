#[cfg(test)]
extern crate test;

fn problem2() -> isize {
    let mut x: isize = 0;
    let mut a: usize = 0;
    let mut v = vec!(1is , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55);
    for _i in (0is..23) {
        let x = (v[v.len() - 2] + v[v.len() - 1]) as isize;
        v.push(x);
    }
    for _i in (0is..v.len() as isize) {
        if v[a] % 2 == 0 { x += v[a]; }
        a += 1;
    }
    x
}

pub fn main() {
    println!("Problem2: {}", problem2());
}

#[test]
fn test2() {
    main();
    assert!(problem2() == 4613732is);
}

#[bench]
fn bench2(b: &mut test::Bencher) {
    b.iter(|| problem2());
}
