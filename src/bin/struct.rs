#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
struct Point3d(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("point.x {}, point.y {}", p.x, p.y);

    let p = Point3d(1.0, 2.0, 3.0);
    println!("Point3d {},", p.0);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 1,
    };
    println!("{:?}", circle);

    let x = 1.0;
    let y = 1.0;
    let p = Point { x, y };

    let p0 = Point { x: 1.0, y: 1.0 };
    let p1: Point = Point { x: 2.0, ..p0 }; // for the rest of field, copy value stored in p0
    println!("{:?}", p1);

    let mut p = Point { x: 1.0, y: 1.0 };
    p.x += 1.0;
    p.y += 1.0;
    println!("{:?}", p);
}
