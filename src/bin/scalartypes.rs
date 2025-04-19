#![allow(unused)]

fn main() {
    // int: signed integers
    // i followed by n (n is the range it can represent)
    let i0: i8 = 0; // -2**(8 -1) ~ 2**(8 -1) - 1
    // we have up to 128
    let i1: i16 = 1;
    let i2: i32 = 2;
    let i3: i64 = 3;
    let i4: i128 = 4;

    let isize = 2; // depends on computer architc

    // unsigned integers
    // range -> 0 ~ 2**n - 1
    let u1: u32 = 4;
    let u2: u8 = 3;
    let u3: u64 = 2;
    let u4: u128 = 4;
    let u5: usize = 4;

    // Floats
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // boolean
    let b: bool = true;

    // char
    let c: char = 'a';

    // type conversion
    let i: i32 = 1;
    let u: u32 = i as u32; // sign integer to unsigned 
    let x: u32 = u + (i as u32);

    // min and max values
    let mini: i32 = i32::MIN;
    let maxi: i32 = i32::MAX;

    println!("max value {:?}", mini);
    println!("min value {:?}", maxi);

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("max char {:?}", max_char);
    println!("min char {:?}", min_char);

    // overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("overflow u32 {:?}", u);

    u32::checked_add(u32::MAX, 1);
    println!("checked add {:?}", u);

    let u: u32 = u32::MAX.wrapping_add(1);
    println!("wrapping add, {:?}", u);
}
