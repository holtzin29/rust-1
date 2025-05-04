#![allow(unused)]

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

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
    };
    let p1 = Point {
        x: 3.0,  // changed to f64
        y: 4.0,  // changed to f64
    };
    let p2 = p0 + p1;
    println!("{:?}", p2);
}
