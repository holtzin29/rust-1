#![allow(unused)]

use std::ops::ControlFlow;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    let color: Color = Color::Red;
    let color: Color = Color::Green;
    let color: Color = Color::Rgba((0), (0), (255), (0.1));
    let color: Color = Color::Hex("#fffff".to_string());
    let color: Color = Color::Hsl {
        h: (0),
        s: (1),
        l: (2),
    };
    println!("{:?}", color);

    // partial eq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // option -> some() | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-2);
    println!("option {:?} ", x);

    // result
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("div by 0".to_string());
    println!("{:?}", res);
}
