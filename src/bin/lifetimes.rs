#![allow(unused)]

// all variables have same lifetime 
fn longest_str<'a>(x: &'a str, y: &'a str)  -> &'a str // the reference might return to invalid place in memory if reference to x or y becomes invalid after returning the reference
{
 if x.len() > y.len() {
    x
 }
 else {
    y
 }
}
fn main() {
   let x = "Hello".to_string();
   // z is outside of curly brackets
   //let z = {
{
    let y = "Hey".to_string();
    let z = longest_str(&x, &y);
    println!("{:?}", z); // z is only used inside curly brackes
    // y is dropped
   }}
