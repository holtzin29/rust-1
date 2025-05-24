#![allow(unused)]

fn f(s: &mut String) {

}
fn main() {
    let mut s = "hi".to_string();

    f(&mut s);
    println!("{s}");
  // Closure can capture variables by
  // - borrow immutable reference &T
  // borrow mutable reference &mut T    
  // take ownership of T  

    // - borrow immutable reference &T
    let s = "hi".to_string();
    let f = || println!("borrow: {s}");
    f();
    println!("main: {s}");

    // borrow mutable reference &mut T   
    let mut s = "hello".to_string();
    let mut f = || s += "world";
  //  f(); cannot borrow value as mutable we need to mut f 
  f(); 
  println!("{s}");
  // take ownership of T  
  let s = "hello".to_string();
  let f = move || {
    println!("{s}"); // take ownership
    s 
};
    f();
//    f(); value is dropped of s inside closure
    // println!("{f}"); // cannot use f here, it has been moved


}
