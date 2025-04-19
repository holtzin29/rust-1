#![allow(unused)]

fn main() {
    // array -> fixed len,  known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("array at index 1 {}", arr[0]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);

    // slice -> len not known at compile time
    let nums: [i32; 10] = [-1, -2, -3, -4, -5, -6, -7, -8, -9, -10];

    let s = &nums[0..3]; // first three elements

    println!("{:?}", s);

    let s = &nums[3..7]; // middle 4 elements
    println!("middle 4 elements {:?}", s);

    let s = &nums[7..];
    println!("last 3 elements {:?}", s);

    let s = &nums[..];
    println!("all elements {:?}", s);
}
