#![allow(unused)]

fn main() {
    let x: i32 = 10;

    if x % 2 == 0 {
        println!("X IS EVEN");
    } else {
        println!("X IS ODD");
    }
    let z: i32 = if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }; // rust is cool becaues of that :)
    println!("z is equal to {z}"); // 10 is greater than 0
}
