#![allow(unused)]

fn main() {
    let mut i = 0;
    loop {
        println!("loop {i}");
        if i == 5 {
            break;
            i += 1;
        }
    }
    // while loop
    let mut i = 0;
    while i <= 3 {
        println!("while {i}");
        i += 1;
    }
    // for loo
    for i in 0..5 {
        println!("for loop {i}");
    } // looping from 0 to 4 (when == 5 it wont run)
    let arr = [1, 2, 3];
    for a in arr {
        println!("array {a}");
    }
    // usize
    let n = arr.len();
    for i in 0..n {
        println!("array {}", arr[i]);
    }
    // loop thru vector
    let v = vec![1, 2, 3];

    for x in v.iter() {
        println!("vector {:?}", v);
    }
    // return value fro loops
    let mut i = 0;
    let z = loop {
        if i == 3 {
            break 99;
        }
        i += 1;
    };
    println!("return loop {z}");

    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i} {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }
}
