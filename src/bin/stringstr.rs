#![allow(unused)]

use std::fmt::format;

fn main() {
    // string -> vector of u8
    // str -> slice of u8

    // string -> mutate data or data that needs to be owned
    // str -> read only

    let msg: String = String::from("Hello Rust");
    let len: usize = msg.len();
    println!("msg {msg}");
    println!("len {len}");

    // str -> string slice
    // immutable
    let msg: String = String::from("Str");
    let s: &str = &msg[0..]; // wrap from string
    let len: usize = msg.len();
    println!("msg {:?}", msg);
    println!("str {s}");
    println!("len {:?}", len);

    let hello: &str = "Hello Rust"; //  hardcoding variable(string literal (stored inside binary))

    let s: &str = r#"
{ "a": 1,
  "b": {"c": 2},
  "d": 3
}"#;
    println!("{s}");

    // deref coercion
    let msg: String = String::from("v");
    let s: &str = &msg;

    // add &str to string
    let mut msg: String = "Hello rust".to_string();
    msg += "!";
    println!("{msg}");

    let lang = "Rust";
    let emoji = "emoji";
    let msg = format!("Hello {lang} {emoji}");
}
