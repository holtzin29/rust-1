#![allow(unused)]

fn take(s: String) {
    println!("take {s}");
}
fn borrow(s: &str) {
    println!("borrwed {s}");
}
fn borrow_mut(s: &mut String) {
 s.push_str("emoji");
}
fn print_len(s: String) {
    println!("length {}", s.len());
}
fn print_len_return_ownership(s: String) -> String{
    println!("length {}", s.len());
    s
}

fn print_len_borrow(s: &str) {
    println!("borrow length {}", s.len());
}
fn main() {
    let s = String::from("rust");
    take(s);

    // borrow immutables
    let s = String::from("rust");
    borrow(&s);
    println!("{s}"); 
 
 // borrow mutable
 let mut s = String::from("rust");
 borrow_mut(&mut s);
 println!("{s}");

 let mut s = String::from("rust");
 borrow_mut(&mut s);
 println!("{s}");

 // modify a fn 
 // 1.take ownership
 let s = String::from("rust");
 print_len(s);
 // returns ownership
 let s = String::from("rust");
 let s = print_len_return_ownership(s);
 println!("rust {}",s);

 // borrow 
 let s = String::from("rust");
 print_len_borrow(&s);
 println!("{s}");
}
