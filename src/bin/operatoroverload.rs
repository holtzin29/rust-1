#![allow(unused)]

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
impl <T> Add for Point<T>  where T: Add<Output = T> {
    type Output = Point<T>;
    type Output = Point;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn main() {
    let p0 = Point {
        x: 1.4,
        y: 2.3,
    }
    let p1 = Point {
        x: 3,
        y: 4,
    }
    let p2 = p0 + p1;
    println!("{:?}", p2);
}
