#![allow(unused)]

fn modify (s: &mut String) {
    *s  += "?"; // does this take ownership
}

fn main () {
// deref
let mut s = String::from("rust");
let s1 = &mut s;
// to modify original data,
*s1 += "?"; // deref to get actual a value and work with original value
println!("{s}");

let mut s = String::from("rust");
modify(&mut s);
println!("{s}");

let x =1;
let y = &x;
let z = &x;
let w = *y + *z; // deref
println!("{w}");

}
