#![allow(unused)]

fn take_string(s: String) {

}
fn borrow_string(s: &String) {

}
fn mut_string(s: &mut String)  {
s.push_str("");
}
fn make_string() -> String {
    "".to_string()
}
fn borrow_str(s:  &str) {}
// fn take_str(s:str) {} wont compile because str is dnamic size type size not known at compile time
fn take_str(s: &str) {
    
}
//fn make_str () -> str {}
fn main() {
let s = String::from("rust");
// string -> struct(vec <u8>) // ownership (there is owner)is transfered // mutable and growable // allocated at the heap // string can be coerced to &str
take_string(s); // ownership has moved
let mut s = String::from("rust");
s += "";
let s = String::from("rust");
borrow_str(&s);
println!("{s}");

borrow_str(&s); // reference to a string is convertible to a string slice (str)

let mut s = String::from("rust");
let s1: &mut String = &mut s;
mut_string(s1);

// string slices -> not known at compile time dinamically sized
/*let a: str = "hello";
let b: str = "hello rust"; 
*/ // this can not happen because a and b are not reference it is not pointing to anything in memory

// &str (size known at compile time(pointer))
// immutable reference
let s: &str = "hello";
borrow_str(s); 
println!("{s}");

// &mut str
let mut s = String::from("rust");
let r: &mut str = &mut s;

}