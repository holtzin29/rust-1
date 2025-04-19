#![allow(unused)]

fn f(s: String) {

}
fn main () {
   /* et  s = String::from("rust");
    f(s);
    println!("{s}"); */
    // this breaks ownership rules
    // memory -> stack and heap
    // stack -> stores data at a compile time(known) -fast, lipo
    // heap -> stores data at not known at compile time -> slower than stack

    // ownership rules
    // each value has a owner
    let s = String::from("rust"); // owner is s
    let i = -1; // owner
    // there can only be only owner at a time
    let  s= String::from("rust");
    let s1 = s; // owner of string is now s1 (was transfered)
    let s2 = s1; // ownership was tranfered from s1 to s2;
    println!("{s2}"); // can only print vlue

    // owner of -1 is i 
    let i = -1 ;
    // value of -1 is i1 (value was copied but there is different owners); value is stored in stack
    let i1 = i;

    // when the owner goes out of scope, the value will be dropped
    let s = String::from("rust"); 
    if (true) {
        s; // in scope
    } // s is dropped 
    let s = String::from("rust"); 
    {
        let s1 = s; // out of scope
    } // value of s is out of scope, value will be dropped

}