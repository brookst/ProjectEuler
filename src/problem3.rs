#[cfg(test)]
extern crate test;

fn problem3() -> isize {
  let (mut _i,mut _j,mut _k) = (0i, 0i, 0i);
  let num = 600851475143f64;
  let mut max = 1.0f64;
  let mut largest = 0is;
  while _i<=(num/max) as isize{
      _k=0;
      if num%_i as f64==0f64{
         _j=1;
          while _j<=_i{
            if _i%_j==0 as isize{
                 _k += 1;
                 }
             _j+=1;
          }
          if _k==2 {
             max *= _i as f64;
             largest = _i;
             println!("{} is a prime factor",_i);
            }
      }
      _i+=1;
   }
   largest
}

pub fn main(){
    problem3();
}

#[test]
fn test3() {
    main();
    assert!(problem3() == 6857is);
}

#[bench]
fn bench3(b: &mut test::Bencher) {
    b.iter(|| problem3());
}
