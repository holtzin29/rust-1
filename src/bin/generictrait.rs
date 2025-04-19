
#![allow(unused)]

use std::array;

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &u32 {
       &self.0
    }
}
// T Is generic data type
impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len() // Removed semicolon here
    }
    fn first(&self) -> &T {
        &self[0] // Removed semicolon here
    }
}
impl <X,Y> List<(X,Y)> for [(X,Y);2] {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &(X,Y) {
        &self[0] // Removed semicolon here
    }
}
fn main () {
    let xy: (u32, u32) = (1,2);
    println!("xy count {:?}", xy.count());
    println!("xy first {:?}", xy.first());

    let arr: [(u32, &str); 2] = [(1, "rust"), (2, "hello")];
    println!("arr count {:?}", arr.count());
    println!("arr first {:?}", arr.first());

    // Added vector example
    let vec = vec![10, 20, 30, 40];
    println!("vec count {:?}", vec.count());
    println!("vec first {:?}", vec.first());
}
