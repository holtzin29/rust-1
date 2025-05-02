#![allow(unused)]
use std::io::{self, Write};
use std::fs::File;

fn main() {
    let s = String::from("Hello");
    let s_str = transform_string_to_str(s.clone()); // Clone to keep original
    println!("{s}");
    
    let str = "Hello";
    let string = transform_str_to_string(str);
    println!("{str}");
    
    let i: i32 = 42;
    let f = transform_int_to_float(i);
    println!("{i}, {f}");

    let f: f32 = 42.1;
    let i = transform_float_to_int(f);
    println!("{f}, {i}");
}

fn transform_string_to_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}  

fn transform_str_to_string(s: &str) -> String {
    s.to_string()
}

fn transform_int_to_float(i: i32) -> f32 {
    i as f32 + 1.01 // Remove semicolon to return this value
}

fn transform_float_to_int(f: f32) -> i32 {
    f as i32
}
