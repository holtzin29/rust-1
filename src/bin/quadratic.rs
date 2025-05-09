#![allow(unused)]

fn main() {
    let a: f32 = 1.0;
    let b: f32 = 10.0;
    let c: f32 = 16.0;

    let delta: f32 = (b * b) - (4.0 * a * c);

    if delta < 0.0 {
        println!("No real solutions");
        return;
    }
    let x1: f32 = (-b + delta.sqrt()) / (2.0 * a);
    let x2: f32 = (-b - delta.sqrt()) / (2.0 * a);

    println!("x1 = {}, x2 = {}", x1, x2);
}
