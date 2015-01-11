fn problem7() -> isize {
   let (mut n, mut i, mut count, mut c, mut a) = (0is,3,0,0,1);//int n, i = 3, count, c;
 
    n = 10001is;
    count = 2is;
 
   if  n >= 1 
   {
      println!("First {} prime numbers are :\n",n);
      println!("1: 2\n");
   }
 
   while count <= n //( count = 2 ; count <= n ;  )
   {
       c = 2is;
      while c <= i // OR for c in range(c, i -1) ( c = 2 ; c <= i - 1 ; c++ )
      {
         if  i%c == 0  {
            break;
            }
        c+=1
      }
      if  c == i {
        a+=1;
         count+=1;
      }
      i+=1;
   }
   c
}

pub fn main() {
    println!("Problem7: {}", problem7());
}

#[test]
fn test7() {
    assert!(problem7() == 104743is);
}
