#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}
// these two are created by the rust compiler itself whenever we create a new variable with new tyype of point
struct Point_u32 {
    x: u32,
    y: u32,
}
struct Point_i32 {
    x: i32,
    y: i32,
}

fn get_x<T>(p: Point<T>) -> T {
    p.x
}
fn get_x_u32(p: Point_u32) -> u32 {
    p.x
}
fn get_x_i32(p: Point_i32) -> i32 {
    p.x
}
fn main () {
   // rust creates a struct of point of u32 and other of i32, monomorphazations  -> increases size of binary and compilation time
   let p0: Point<u32> = Point { x: 2, y: 3 }; // rust will create a new type for u32, i32, so on (it will copy point and set t to i32)
   let p1: Point<i32> = Point {x : -1, y: -3}; // monomorphazation

   get_x(p0);
   get_x(p1);

   let p0: Point_u32 = Point_u32 { x: 1, y: 2};
   let p1: Point_i32 = Point_i32 { x: -1, y: -2 };

}