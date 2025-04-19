#![allow(unused)]
// GENERIC DATA TYPE FOR EVERY TYPE ON FUNCTION
fn swap<A,B>(t: (A,B)) -> (B,A) {
    (t.1, t.0)
}
use std::cmp::PartialOrd; // IMPORT
// t has to implement partial ord trait 
fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None
    }
    
    let mut largest = &s[0];
    for item in s {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
fn main() {
   let t: (u32, u32) = (1,2);
   let s = swap(t);
   println!("{:?}", s);

   let t: (i32, u32) = (-1,2);
   let s = swap(t);
   println!("{:?}", s);
   
   let nums = vec![33,1,24,5,66,99];
   let largest = max(&nums);
   println!("largest = {:?}", largest);

   let chars = vec!['c','a','b','d'];
   let largest = max(&chars);
   println!("largest = {:?}", largest);

}
