#![allow(unused)]

fn main() {
    // once you declare variables, value cannot change
    let x: i32 = -123;
    // this is immutable
    let mut y: i32 = -123;
    y += 1;

    let z = -123; // rust infer type by looking at the value(signed integer);
    let w = 123;

    const NUM: u32 = 1;

    let x: i32 = -1;
    let x: bool = true;

    let v: Vec<_> = vec![1, 2, 3];
}
