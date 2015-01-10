#[cfg(test)]
extern crate test;

fn problem3(verbose: bool) -> isize {
    let (mut candidate, mut _partner, mut _factors, mut largest) = (0is, 0, 0, 0);
    let num = 600851475143f64;
    let mut max = 1.0f64;
    while candidate <= (num / max) as isize {
        _factors = 0;
        if num % candidate as f64 == 0f64{
            // candidate is a factor of num
            _partner = 1;
            while _partner <= candidate {
                if candidate % _partner == 0 as isize{
                    // candidate has a _partner
                    _factors += 1;
                }
                _partner += 1;
            }
            if _factors == 2 {
                // candidate has only 2 factors, itself and 1
                if verbose {
                    println!("    {:4} is a prime factor",candidate);
                }
                largest = candidate;
                max *= candidate as f64;
            }
        }
        candidate += 1;
    }
    largest
}

pub fn main(){
    println!("Problem3: {}", problem3(true));
}

#[test]
fn test3() {
    main();
    assert!(problem3(false) == 6857is);
}

#[bench]
fn bench3(b: &mut test::Bencher) {
    b.iter(|| problem3(false));
}
