#![allow(unused)]

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32 = a + b;
    let d: i32 = a - b;
    let e: i32 = a * b;
    let f: i32 = a / b; // this rounds down and decimal part gets cuff off 1/2 = 0.5 -> 0
    println!("{a} / {b} = {f}");

    // remainder -> not mod
    // -1 % 2 = -1
    let a: i32 = -1;
    let b: i32 = 2;
    let c: i32 = a % b;
    println!("remainder {}", c);

    // literals
    let a = 1i32;
    let b = 3u64;
    let c = 1.23e3;

    // boolean
    let a = true && false;
    let b = true || false;
    let c = !true;

    //bit wise
    let a: u8 = 5; //101    

    let b: u8 = 3; //011
    println!("a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    println!("a ^ b = {:03b}", a ^ b);
    println!("!a = {:03b}", !a);
    println!("1 << 3 = {:03b}", 1u32 << 3);
    println!("10 >> 3 = {:3b}", 10u32 >> 2);
}
