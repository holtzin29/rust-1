#![allow(unused)]

fn main() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // borrow -> use value without transfering ownership & -> reference(mutable or immutable)
    // immutable borrow
    let s1 = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    let s3 = s2; // stores value of s2

    // mutable borrow
    // only one mutable refence at a time
    let mut s = String::from("rust");
    // however,
    let s0 = &mut s; // we can do that because after this borrow this variable is not used anymore
    let s1 = &mut s;
    // lets modify s1
    s1.push_str("hi"); // s1 is used so one mutable reference at a time
   println!("{s}"); // hi is added to s 

   // cannot create mutable and immutable simultaneosly
   let mut s = String::from("rust");
   let s1 = &s;
   let s2 = &s;
 //  let s3 = &mut s; // cant do that
 println!("{s1} {s2}");

 // reference must not outlive the value
 let mut s = String::from("rust");
 let s1 = &s;
 println!("{s1}");
 {
    let s1 =s; // inside curly brackets, s no longer exists()
 } // s1 is dropped

}
// does not compile because string is input and ownership of s is dropped inside function and we try to reference it in a return 
/* 
fn f(s:String) -> &String {
 &s
}
 */