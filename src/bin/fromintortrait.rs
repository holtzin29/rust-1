#![allow(unused)]

use std::convert::{From, Into};
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self { x: value.0, y: value.1}
    }
}
fn main () {
    let t: (u32, u32) = (1, 2);
    let p = Point::from(t);  // convert tuple  into point
    println!("{:?}", p);

    // implementing from also is into
    let p: Point = t.into(); // convert one data type for other data type
    println!("{:?}", p);

}