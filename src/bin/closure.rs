#![allow(unused)]

// anoynmous function
/*  instead of this, we do this at main
fn add(x: u32, y: u32) -> u32 {
    x + y   
}
*/
fn main() {

    let x = |x: u32, y: u32| -> u32 {
        x + y
    }; // anonymous function
    let x = |x: u32, y: u32| -> u32 {
        x + y}; // anonymous function again we can do same thing

   let f = |x, y| x + y; // rust knows the type
   let z = f(1,2); // rust knows the type

   println!{"{z}"};

  //  let f = f(1.2, 3.2); this wont compile because rust already innered the type

 // capture variables where anonymous functions is declared
 let v = 1;
 let f = |x: i32| x + v;

 let w = vec![1,2,3];
 let v2: Vec<i32> = w.iter().map(|x| x + 1).collect(); // the map itself takes a anonymous function sort of
 // this is useful for example if we want addition a variable which the function is not aware of but the main is
}
