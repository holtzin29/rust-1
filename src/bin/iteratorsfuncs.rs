#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;

fn main() {
    let vals: Vec<i32> = vec![1, 2, 3];
    let mut data: Vec<i32> = Vec::new();
    for v in vals {
        if v <= 2 {
            data.push(v * 2);
            // data.insert()
        }
    }
    let vals: Vec<i32> = vec![1, 2, 3];
    // let data: Vec<i32> = vals.iter().map(|x| x * 2).collect();
    // filter gives a reference to value that is interating, it is moving ownership from vals to data.
    let data: HashSet<u32> = vals
        .into_iter()
        .filter(|x| x <= &2)
        .map(|x| (x as u32) * 2)
        .collect();
    println!("{:?}", data);

    // there is no value for d that it can iterat with, 3:3, not 3:4.
    let keys: Vec<String> = vec!["a", "b", "c", "d"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let vals = vec![1, 2, 3];
    // takes ownership from values stored in vals and keys (thats why into iter)
    // let zipped: Vec<(String, i32)> = keys.into_iter().zip(vals.into_iter()).collect();  // iterating thru two vectors at same time, bigger value will be ommited.
    let zipped: HashMap<String, i32> = keys.into_iter().zip(vals.into_iter()).collect(); // rust knows how to convert u32/i32 to hashmap.
    println!("{:?}", zipped);


    let vals = vec![1,2,3];
    let sum = vals.iter().fold(0, |acc, x| acc + x);
    println!("{sum}");
}
