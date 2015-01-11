#[cfg(test)]
extern crate test;

fn is_palindrome(n: isize) -> bool {
    let chars: Vec<char> = format!("{}", n).chars().collect();
    let max = chars.len() - 1;
    for i in (0..chars.len()) {
        if chars[i] != chars[max - i] {
            return false;
        }
    }
    true
}

fn problem4() -> isize {
    let mut largest = 0is;
    for i in (100..999) {
        for j in (100..999) {
            let candidate = i * j;
            if candidate > largest && is_palindrome(candidate) {
                largest = candidate;
            }
        }
    }
    largest
}

pub fn main() {
    println!("Problem4: {}", problem4());
}

#[test]
fn test4() {
    main();
    assert!(is_palindrome(9009) == true);
    assert!(is_palindrome(9010) == false);
    assert!(problem4() == 906609is);
}

#[bench]
fn bench4(b: &mut test::Bencher) {
    b.iter(|| problem4());
}
